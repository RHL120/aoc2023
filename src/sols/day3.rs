fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn check_adj(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    [
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (0, -1),
    ]
    .iter()
    .filter_map(|&(dx, dy)| {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;
        let t = data.get(ny)?.get(nx).map(|x| *x)?;
        Some(t)
    })
    .any(is_symbol)
}

pub fn part1(input: &str) -> Result<String, String> {
    let data: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut res: u64 = 0;
    for (y, line) in data.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let mut num = String::new();
            let mut accept = false;
            if !line[x].is_ascii_digit() {
                x += 1;
                continue;
            }
            while x < line.len() && line[x].is_ascii_digit() {
                accept |= check_adj(&data, x, y);
                num.push(line[x]);
                x += 1;
            }
            if accept {
                println!("{num}");
                res += num.parse::<u64>().unwrap();
            }
        }
    }
    Ok(res.to_string())
}
pub fn part2(_input: &str) -> Result<String, String> {
    todo!()
}
