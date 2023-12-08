pub fn gcd(mut n: u128, mut m: u128) -> u128 {
    while n != m {
        if n > m {
            n -= m;
        } else {
            m -= n
        }
    }
    n
}
pub fn lcm(n: u128, m: u128) -> u128 {
    n * m / gcd(n, m)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_gcd() {
        assert_eq!(gcd(34, 134), 2)
    }
    fn test_lcm() {
        assert_eq!(gcd(3, 2), 6)
    }
}
