use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use std::sync::Arc;
use tokio::sync::OnceCell;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::from_str;
use futures::future::join_all;

static IPA_DICT: OnceCell<Arc<HashMap<String, String>>> = OnceCell::const_new();

#[derive(Deserialize)]
struct RequestData {
    text: String,
}

#[derive(Serialize)]
struct ResponseData {
    phonetic: String,
}

// Load IPA Dictionary from file
async fn load_ipa_dict() -> Arc<HashMap<String, String>> {
    let mut dict = HashMap::new();
    if let Ok(file) = File::open("en_UK.txt") {
        let reader = BufReader::new(file);
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

// Convert a word to phonetic notation
async fn word_to_phonetic(word: &str) -> String {
    let dict = IPA_DICT.get().unwrap();
    dict.get(word).cloned().unwrap_or_else(|| word.to_string())
}

// Handle TCP client connection
async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(size) if size > 0 => {
            if let Ok(request) = from_str::<RequestData>(std::str::from_utf8(&buffer[..size]).unwrap()) {
                let words = request.text.split_whitespace().map(word_to_phonetic);
                let phonetics = join_all(words).await;

                let response = serde_json::to_string(&ResponseData { phonetic: phonetics.join(" ") }).unwrap();
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        }
        _ => {
            let _ = stream.write_all(b"Invalid request").await;
        }
    }
}

#[tokio::main]
async fn main() {
    let dict = load_ipa_dict().await;
    IPA_DICT.set(dict).unwrap();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("TCP Server running on 127.0.0.1:3000...");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
