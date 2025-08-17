use std::cmp::{min, max};
pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    (min(nb_1, min(nb_2, nb_3)), max(nb_1, max(nb_2, nb_3)))
}