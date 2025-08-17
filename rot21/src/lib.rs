pub fn rot21(input: &str) -> String {
    let mut res = "".to_string();
    for ch in input.chars() {
        if ch >= 'a' && ch <= 'z' {
            let mut nb = (ch as u8) - b'a' + 21;
            if nb > 26 {
                nb -= 26;
            }
            res.push((b'a' + nb) as char);
        } else if ch >= 'A' && ch <= 'Z' {
            let mut nb = (ch as u8) - b'A' + 21;
            if nb > 26 {
                nb -= 26;
            }
            res.push((b'A' + nb) as char);
        } else {
            res.push(ch);
        }
    }
    res
}
