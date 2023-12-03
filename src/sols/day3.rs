use std::collections::HashSet;
fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn get_adj(data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize, char)> {
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
        Some((nx, ny, t))
    })
    .collect()
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
                accept |= get_adj(&data, x, y).iter().any(|&(_, _, c)| is_symbol(c));
                num.push(line[x]);
                x += 1;
            }
            if accept {
                res += num.parse::<u64>().unwrap();
            }
        }
    }
    Ok(res.to_string())
}
pub fn part2(input: &str) -> Result<String, String> {
    let data: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut res: u64 = 0;
    for (y, line) in data.iter().enumerate() {
        for (x, &cell) in line.iter().enumerate() {
            if cell == '*' {
                // We transfrom the numbers into a hash map to preserve uniqueness
                let numbers: HashSet<((usize, usize), u64)> = (get_adj(&data, x, y))
                    .iter()
                    .filter_map(|&(nx, ny, c)| {
                        if !c.is_ascii_digit() {
                            return None;
                        }
                        let s1 = data[ny][nx..]
                            .iter()
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>();
                        let s2: String = data[ny][..nx]
                            .iter()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect();
                        let l2 = nx - s2.len();
                        let s: String = s2 + &s1;
                        //We need the start of the integer sequenece (l2)
                        //in order to distinguish between to adjecent cells with
                        //the same value (42*42 for example)
                        (!s.is_empty()).then(|| ((l2, ny), s.parse::<u64>().unwrap()))
                    })
                    .collect();
                let numbers: Vec<&((usize, usize), u64)> = numbers.iter().collect();
                if numbers.len() == 2 {
                    res += numbers[0].1 * numbers[1].1;
                }
            }
        }
    }
    Ok(res.to_string())
}
