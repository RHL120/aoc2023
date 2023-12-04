use crate::utils::parser;
fn parse_card(s: &str) -> Option<((u32, Vec<u32>, Vec<u32>), &'_ str)> {
    let (_, s) = parser::parse_const(s, "Card")?;
    let (id, s) = parser::parse_unsigned_int(s.trim())?;
    let (_, s) = parser::parse_const(s, ":")?;
    let (list1, s) = parser::many(s.trim_start(), |s| {
        let (n, s) = parser::parse_unsigned_int(s)?;
        Some((n, s.trim_start()))
    })?;
    let (_, s) = parser::parse_const(s.trim_start(), "|")?;
    let s = s.trim_start();
    let (list2, s) = parser::many(s, |s| {
        let (n, s) = parser::parse_unsigned_int(s)?;
        Some((n, s.trim_start()))
    })?;
    Some(((id, list1, list2), s))
}

fn parse_game(s: &str) -> Option<Vec<(u32, Vec<u32>, Vec<u32>)>> {
    s.lines()
        .map(|s| {
            let (c, s) = parse_card(s)?;
            parser::parse_eof(s)?;
            Some(c)
        })
        .collect()
}

fn num_winning(w: &Vec<u32>, m: &Vec<u32>) -> usize {
    m.iter().filter(|&x| w.contains(x)).count()
}

pub fn part1(input: &str) -> Result<String, String> {
    let game = parse_game(input).ok_or("Failed to parse game")?;
    let res = game
        .iter()
        .map(|(id, v1, v2)| {
            let num = num_winning(v1, v2);
            if num == 0 {
                0
            } else {
                (2 as usize).pow((num - 1) as u32)
            }
        })
        .sum::<usize>();
    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
