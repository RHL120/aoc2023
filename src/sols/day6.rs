fn parse_input(input: &str) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut data = input.lines().map(|s| {
        let s = s
            .strip_prefix("Time:")
            .or_else(|| s.strip_prefix("Distance: "))?
            .trim()
            .split(" ")
            .filter_map(|x| (!x.is_empty()).then(|| x.parse::<usize>().ok()));
        s.collect::<Option<Vec<_>>>()
    });
    Some((data.next()??, data.next()??))
}

pub fn part1(input: &str) -> Result<String, String> {
    let (time, distance) = parse_input(input).ok_or("Failed to parse input")?;
    let d: usize = time
        .iter()
        .zip(distance)
        .map(|(&t, record)| {
            (1..t)
                .filter_map(move |v| {
                    let d = v * (t - v);
                    (d > record).then(|| d)
                })
                .count()
        })
        .product();
    Ok(d.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
