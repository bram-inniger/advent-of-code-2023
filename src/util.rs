pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut pair = (a, b);

    while pair.1 > 0 {
        pair = (pair.1, pair.0 % pair.1)
    }

    pair.0
}
