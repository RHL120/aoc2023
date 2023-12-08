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
pub fn lcm(nums: &[u128]) -> Option<u128> {
    let &mx = nums.iter().max()?;
    let prod = nums.iter().product::<u128>();
    nums.iter().all(|&x| x != 0).then(|| {
        (mx..=prod)
            .find(|&n| nums.iter().all(|&m| n % m == 0))
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_gcd() {
        assert_eq!(gcd(34, 134), 2)
    }
}
