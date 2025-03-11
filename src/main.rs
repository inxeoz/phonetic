use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use axum::http::Method;
use tokio::sync::OnceCell;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

static IPA_DICT: OnceCell<Arc<HashMap<String, String>>> = OnceCell::const_new();

#[derive(Deserialize)]
struct RequestData {
    text: String,
}

#[derive(Serialize)]
struct ResponseData {
    phonetic: String,
}

async fn load_ipa_dict() -> Arc<HashMap<String, String>> {
    let mut dict = HashMap::new();
    if let Ok(file) = std::fs::File::open("en_UK.txt") {
        let reader = std::io::BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let word = parts[0].to_lowercase();
                let phonetic = parts[1..].join(" ");
                dict.insert(word, phonetic);
            }
        }
    }
    Arc::new(dict)
}

async fn word_to_phonetic(word: &str) -> String {
    let dict = IPA_DICT.get().unwrap();
    dict.get(word).cloned().unwrap_or_else(|| word.to_string())
}

async fn phonetic_to_english(word: &str) -> String {
    let ipa_map = create_ipa_to_english_map();
    ipa_to_english_sound(&word[1..word.len() - 1], &ipa_map)
}

async fn combined_mapping(word: &str) -> String {
    let phonetic = word_to_phonetic(word).await;
    phonetic_to_english(&phonetic).await
}

async fn process_request(Json(payload): Json<RequestData>) -> Json<ResponseData> {
    let words = payload.text.split_whitespace().map(combined_mapping);
    let phonetics = futures::future::join_all(words).await;
    Json(ResponseData {
        phonetic: phonetics.join(" "),
    })
}

#[tokio::main]
async fn main() {
    let dict = load_ipa_dict().await;
    IPA_DICT.set(dict).unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any) // Allow all origins, change if needed
        .allow_headers(Any);

    let app = Router::new().route("/convert", post(process_request)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3005").await.unwrap();
    println!("Server running on http://127.0.0.1:3005...");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}


pub fn create_ipa_to_english_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("p", "p");
    map.insert("b", "b");
    map.insert("t", "t");
    map.insert("d", "d");
    map.insert("k", "k");
    map.insert("g", "g");
    map.insert("f", "f");
    map.insert("v", "v");
    map.insert("s", "s");
    map.insert("z", "z");
    map.insert("h", "h");
    map.insert("m", "m");
    map.insert("n", "n");
    map.insert("l", "l");
    map.insert("w", "w");
    map.insert("ʃ", "sh");
    map.insert("ʒ", "zh");
    map.insert("tʃ", "ch");
    map.insert("dʒ", "j");
    map.insert("ŋ", "ng");
    map.insert("j", "y");
    map.insert("θ", "th");
    map.insert("ð", "dh");
    map.insert("ɹ", "r");
    map.insert("ʔ", "'");
    map.insert("x", "kh");
    map.insert("ɲ", "ny");
    map.insert("i", "ee");
    map.insert("ɪ", "ih");
    map.insert("e", "eh");
    map.insert("ɛ", "e");
    map.insert("æ", "a");
    map.insert("ɑ", "ah");
    map.insert("ɒ", "o");
    map.insert("ɔ", "aw");
    map.insert("o", "oh");
    map.insert("ʊ", "uh");
    map.insert("u", "oo");
    map.insert("ʌ", "u");
    map.insert("ə", "uh");
    map.insert("ɜ", "er");
    map.insert("eɪ", "ay");
    map.insert("aɪ", "ai");
    map.insert("aʊ", "ow");
    map.insert("ɔɪ", "oi");
    map.insert("oʊ", "oh");
    map.insert("ɪə", "eer");
    map.insert("ˈ", "'");
    map.insert("ˌ", ",");
    map.insert("ː", ":");
    map
}

pub fn ipa_to_english_sound(ipa_word: &str, map: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    let mut chars = ipa_word.chars().peekable();
    while let Some(c) = chars.next() {
        let mut symbol = c.to_string();
        if let Some(&next_c) = chars.peek() {
            let potential_symbol = format!("{}{}", c, next_c);
            let potential_symbol_str = potential_symbol.as_str();

            if map.contains_key(potential_symbol_str) {
                symbol = potential_symbol;
                chars.next();
            }
        }

        let symbol_str = symbol.as_str();
        let sound = map.get(symbol_str).unwrap_or(&symbol_str);
        if !result.is_empty() {
            result.push('-');
        }
        result.push_str(sound);
    }
    result
}
