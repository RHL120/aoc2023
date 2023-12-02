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

pub fn line_number(line: &str) -> Option<u32> {
    let number_names: Vec<&[char]> = vec![
        &['o', 'n', 'e'],
        &['t', 'w', 'o'],
        &['t', 'h', 'r', 'e', 'e'],
        &['f', 'o', 'u', 'r'],
        &['f', 'i', 'v', 'e'],
        &['s', 'i', 'x'],
        &['s', 'e', 'v', 'e', 'n'],
        &['e', 'i', 'g', 'h', 't'],
        &['n', 'i', 'n', 'e'],
    ];
    let chars: Vec<char> = line.chars().collect();
    let digits = (0..chars.len())
        .flat_map(|i| {
            if let Some(digit) = chars[i].to_digit(10) {
                vec![digit]
            } else {
                number_names
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &num)| {
                        if chars[i..].starts_with(num) {
                            Some((idx + 1) as u32)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        })
        .collect::<Vec<u32>>();
    let first = digits.first()?;
    let last = digits.last()?;
    Some(first * 10 + last)
}

pub fn part2(input: &str) -> Result<String, String> {
    let result = input
        .lines()
        .map(line_number)
        .sum::<Option<u32>>()
        .ok_or("Failed to parse file")?;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let example_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(example_input).unwrap(), "142");
    }
    #[test]
    fn test_part2() {
        let example_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(example_input).unwrap(), "281");
    }
}
