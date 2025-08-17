pub fn reverse_it(v: i32) -> String {
    let n_v = v as i64;
    if n_v < 0 {
        format!("-{}{}", n_v.abs().to_string().chars().rev().collect::<String>(), n_v.abs())
    } else {
        format!("{}{}", n_v.to_string().chars().rev().collect::<String>(), n_v)
    }
}