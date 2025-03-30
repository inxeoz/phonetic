use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use axum::http::Method;
use tokio::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};

static IPA_DICT: OnceCell<Arc<HashMap<String, String>>> = OnceCell::const_new();

#[derive(Deserialize)]
struct RequestData {
    text: String,
}
#[derive(Serialize)]
struct Pair {
    text: String,
    phonetic: String,
}

#[derive(Serialize)]
struct ResponseData {
    phonetic: Vec<Pair>,
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


async fn combined_mapping(word: &str) -> String {
    let phonetic = word_to_phonetic(word).await;
    phonetic
}

async fn process_request(Json(payload): Json<RequestData>) -> Json<ResponseData> {
    let mut response = ResponseData {
        phonetic: Vec::new(),
    };
    let words = payload.text.split_whitespace();

    for word in words {
        let phonetic = combined_mapping(word).await;
        response.phonetic.push(Pair { text: word.to_string(), phonetic });
    }

    Json(response)
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
