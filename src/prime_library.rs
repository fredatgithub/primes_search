pub fn is_prime_i32(n: i32) -> bool {
    // Check if a number is prime.
    if n <= 1 {
        return false;
    }

    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i32;

    for i in (11..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn is_prime_u128(n: u128) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i: u128 = 3;

    while i <= n / i {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}