pub fn part1(input: &str) -> Result<String, String> {
    let solution = input
        .lines()
        .map(|x| {
            let mut digits = x.chars().filter_map(|x| x.to_digit(10));
            let first = digits.next()?;
            let last = digits.last().unwrap_or(first);
            Some(first * 10 + last)
        })
        .sum::<Option<u32>>()
        .ok_or("Failed to parse input")?;
    Ok(solution.to_string())
}
