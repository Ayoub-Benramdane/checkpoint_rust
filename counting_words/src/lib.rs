use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    let split_words: Vec<_> = words.split(|c: char| !c.is_alphanumeric() && c != '\'').collect();

    for mut word in split_words {
        if word.trim().is_empty() {
            continue;
        }
        // word = word.trim_matches('\'');
        if word.starts_with('\'') {
            word = &word[1..];
        }
        if word.ends_with('\'') {
            word = &word[..word.len() - 1];
        }
        *res.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    res
}
