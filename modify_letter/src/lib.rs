pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.replace(letter, "")
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut res = "".to_string();
    for ch in s.chars() {
        if ch != letter && ch != letter.to_ascii_lowercase() {
            res.push(ch)
        }
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut res = "".to_string();
    for mut ch in s.chars() {
        if ch == letter {
            ch = letter.to_ascii_uppercase()
        } else if ch == letter.to_ascii_uppercase() {
            ch = letter
        }
        res.push(ch)
    }
    res
}