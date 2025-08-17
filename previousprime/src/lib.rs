pub fn prev_prime(nbr: u64) -> u64 {
    if nbr <= 2 {
        return 0;
    }
    if is_prime(nbr-1) {
        return nbr-1;
    }
    prev_prime(nbr - 1)
}

pub fn is_prime(nbr: u64) -> bool {
    for i in 2..=((nbr as f64).sqrt() as u64) {
        if nbr % i == 0 {
            return false;
        }
    }
    true
}
