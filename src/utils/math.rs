pub fn lcm(nums: &[usize]) -> Option<usize> {
    let &mx = nums.iter().max()?;
    let prod = nums.iter().product::<usize>();
    nums.iter().all(|&x| x != 0).then(|| {
        (mx..=prod)
            .find(|&n| nums.iter().all(|&m| n % m == 0))
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[4, 8, 17, 136]).unwrap(), 136)
    }
}
