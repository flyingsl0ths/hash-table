pub const HASH_PRIME: u32 = 151u32;
pub const HASH_PRIME2: u32 = 163u32;

pub fn next_prime(n: usize) -> usize {
    let mut n = n;

    while !is_prime(n) {
        n += 1;
    }

    n
}

pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let m = (n as f64).sqrt().floor() as usize;
    for i in (3..=m).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
