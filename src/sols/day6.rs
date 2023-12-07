fn parse_input_part1(input: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut data = input.lines().map(|s| {
        let s = s
            .strip_prefix("Time:")
            .or_else(|| s.strip_prefix("Distance: "))?
            .trim()
            .split(' ')
            .filter(|&x| (!x.is_empty()))
            .map(|x| x.parse::<usize>().ok());
        s.collect::<Option<Vec<_>>>()
    });
    Some((data.next()??, data.next()??))
}

pub fn part1(input: &str) -> Result<String, String> {
    let (time, distance) = parse_input_part1(input).ok_or("Failed to parse input")?;
    let d: usize = time
        .iter()
        .zip(distance)
        .map(|(&t, record)| {
            (1..t)
                .filter_map(move |v| {
                    let d = v * (t - v);
                    (d > record).then_some(d)
                })
                .count()
        })
        .product();
    Ok(d.to_string())
}

fn parse_input_part2(input: &str) -> Option<(usize, usize)> {
    let mut data = input.lines().map(|s| {
        s.strip_prefix("Time:")
            .or_else(|| s.strip_prefix("Distance: "))?
            .replace(' ', "")
            .parse()
            .ok()
    });
    Some((data.next()??, data.next()??))
}

pub fn part2(input: &str) -> Result<String, String> {
    let (time, record) = parse_input_part2(input).ok_or("Failed to parse file")?;
    let res = (1..time)
        .filter_map(move |v| {
            let d = v * (time - v);
            (d > record).then_some(d)
        })
        .count();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "288");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "71503");
    }
}
