pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for n in 1..=i {
        res.push(format!("{}{}", " ".repeat(n as usize), v.repeat(n as usize)))
    }
    for n in (1..i).rev() {
        res.push(format!("{}{}", " ".repeat(n as usize), v.repeat(n as usize)))
    }
    res
}