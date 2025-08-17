#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }
    if is_prime(nb) {
        return Some(Ok(nb));
    }
    if nb%2 == 0 {
        return Some(Err(PrimeErr::Even));
    } 
    Some(Err(PrimeErr::Divider((nb as f64).sqrt() as u32)))
}

pub fn is_prime(nb: u32) -> bool {
    for i in 2..=(nb as f64).sqrt() as u32 {
        if nb % i == 0 {
            return false;
        }
    }
    true
}
