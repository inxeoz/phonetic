use std::collections::HashMap;

/// Creates a mapping of IPA (International Phonetic Alphabet) symbols to English sound approximations.
/// Returns a HashMap with static string references for keys and values.
pub fn create_ipa_to_english_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Consonants
    map.insert("p", "p");       // pin
    map.insert("b", "b");       // bin
    map.insert("t", "t");       // tin
    map.insert("d", "d");       // din
    map.insert("k", "k");       // kin
    map.insert("g", "g");       // gin
    map.insert("f", "f");       // fin
    map.insert("v", "v");       // van
    map.insert("s", "s");       // sin
    map.insert("z", "z");       // zoo
    map.insert("h", "h");       // hat
    map.insert("m", "m");       // man
    map.insert("n", "n");       // net
    map.insert("l", "l");       // let
    map.insert("w", "w");       // win
    map.insert("ʃ", "sh");      // ship
    map.insert("ʒ", "zh");      // measure
    map.insert("tʃ", "ch");     // church
    map.insert("dʒ", "j");      // judge
    map.insert("ŋ", "ng");      // sing
    map.insert("j", "y");       // yes
    map.insert("θ", "th");      // think
    map.insert("ð", "dh");      // this (distinguished from θ)
    map.insert("ɹ", "r");       // red (American rhotic)
    map.insert("ʔ", "'");       // glottal stop, as in "uh-oh"
    map.insert("x", "kh");      // loch (Scottish)
    map.insert("ɲ", "ny");      // canyon (approximating Spanish ñ)

    // Monophthong Vowels
    map.insert("i", "ee");      // fleece (high front)
    map.insert("ɪ", "ih");      // kit (near-high front)
    map.insert("e", "eh");      // dress (mid front)
    map.insert("ɛ", "e");       // trap (low-mid front)
    map.insert("æ", "a");       // cat (low front)
    map.insert("ɑ", "ah");      // palm (low back)
    map.insert("ɒ", "o");       // lot (British rounded)
    map.insert("ɔ", "aw");      // thought (open-mid back)
    map.insert("o", "oh");      // goat (monophthong approximation)
    map.insert("ʊ", "uh");      // foot (near-high back)
    map.insert("u", "oo");      // goose (high back)
    map.insert("ʌ", "u");       // strut (mid back)
    map.insert("ə", "uh");      // comma (schwa)
    map.insert("ɜ", "er");      // nurse (British non-rhotic)

    // Diphthongs
    map.insert("eɪ", "ay");     // face
    map.insert("aɪ", "ai");     // price
    map.insert("aʊ", "ow");     // mouth
    map.insert("ɔɪ", "oi");     // choice
    map.insert("oʊ", "oh");     // goat (American)
    map.insert("ɪə", "eer");    // near (British non-rhotic)

    // Suprasegmentals
    map.insert("ˈ", "'");       // primary stress (e.g., 'apple)
    map.insert("ˌ", ",");       // secondary stress
    map.insert("ː", ":");       // length marker

    map
}

/// Converts an IPA word into an English sound approximation using the provided mapping.
/// Multi-character IPA symbols (e.g., "tʃ") are prioritized over single characters.
/// Returns a hyphen-separated string of English sound equivalents.
pub fn ipa_to_english_sound(ipa_word: &str, map: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    let mut chars = ipa_word.chars().peekable();

    while let Some(c) = chars.next() {
        let mut symbol = c.to_string();

        // Check for two-character IPA symbols (e.g., "tʃ", "aɪ")
        if let Some(&next_c) = chars.peek() {
            let potential_symbol = format!("{}{}", c, next_c);
            let potential_symbol_str = potential_symbol.as_str();  // Ensure `symbol` lives long enough
            if map.contains_key(potential_symbol_str) {
                symbol = potential_symbol;
                chars.next(); // Skip the next character since it’s part of the symbol
            }
        }
        let symbol_str = symbol.as_str();

        // Append the mapped sound or the original symbol if unmapped
        let sound = map.get(symbol_str).unwrap_or(&symbol_str);
        if !result.is_empty() {
            result.push('-');
        }
        result.push_str(sound);
    }

    result
}

// pub fn main() {
//     let ipa_map = create_ipa_to_english_map();
//
//     // Example IPA words
//     let examples = vec![
//         "/hæt/",    // "hat"
//         "/dʒʌmp/",  // "jump"
//         "/ʃɪp/",    // "ship"
//         "/aɪ/",     // "eye"
//         "/kæt/",    // "cat"
//     ];
//
//     for ipa in examples {
//         let english_sound = ipa_to_english_sound(&ipa[1..ipa.len()-1], &ipa_map); // Strip slashes
//         println!("IPA: {} -> English sound: {}", ipa, english_sound);
//     }
// }