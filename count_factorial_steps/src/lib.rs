pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut count = 0;
    let mut nb = 1;
    while nb < factorial {
        count += 1;
        nb *= count;
        if nb == factorial {
            return count;
        }
    }
    0
}
