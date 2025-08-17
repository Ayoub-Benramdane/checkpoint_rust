pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }
    let mut res = "".to_string();
    let mut index = 0;
    let mut ngz = 0;

    while res.len() < s.len() {
        res.push(s.chars().nth(index  as usize).unwrap());
        index += letters_per_turn;
        if index >= s.len() as u32 {
            ngz += 1;
            index = ngz;
        }
    }
    Some(res)
}
