use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut min = i32::MAX;
    for (_, nb) in h {
        if nb < min {
            min = nb
        }
    }
    min
}