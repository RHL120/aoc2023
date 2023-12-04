use crate::utils::parser;
use std::collections::HashMap;
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

fn num_matching(w: &Vec<u32>, m: &Vec<u32>) -> usize {
    m.iter().filter(|&x| w.contains(x)).count()
}

pub fn part1(input: &str) -> Result<String, String> {
    let game = parse_game(input).ok_or("Failed to parse game")?;
    let res = game
        .iter()
        .map(|(id, v1, v2)| {
            let num = num_matching(v1, v2);
            if num == 0 {
                0
            } else {
                (2 as usize).pow((num - 1) as u32)
            }
        })
        .sum::<usize>();
    Ok(res.to_string())
}

fn matchings(game: &Vec<(u32, Vec<u32>, Vec<u32>)>) -> HashMap<usize, usize> {
    game.iter()
        .map(|(id, v1, v2)| ((*id as usize), num_matching(v1, v2)))
        .collect()
}

fn new_instances(card: usize, mats: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut ret = HashMap::new();
    for mat in 1..=mats[&card] {
        let ni = card + mat;
        match ret.get(&ni) {
            Some(x) => ret.insert(ni, x + 1),
            None => ret.insert(ni, 1),
        };
        let nis = new_instances(ni, mats);
        for (k, v) in nis {
            match ret.get(&k) {
                Some(x) => ret.insert(k, x + v),
                None => ret.insert(k, v),
            };
        }
    }
    ret
}

pub fn part2(input: &str) -> Result<String, String> {
    let game = parse_game(input).ok_or("Failed to parse game")?;
    let matches = matchings(&game);
    let mut instances: HashMap<usize, usize> =
        game.iter().map(|(id, _, _)| (*id as usize, 1)).collect();
    for card in matches.keys() {
        let ni = new_instances(*card, &matches);
        for (k, v) in ni {
            match instances.get(&k) {
                Some(x) => instances.insert(k, x + v),
                None => instances.insert(k, v),
            };
        }
    }
    let res = instances.iter().map(|(_, x)| *x).sum::<usize>();
    Ok(res.to_string())
}
