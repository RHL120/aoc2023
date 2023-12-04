use crate::utils::parser;
use std::iter;
type Card = (u32, Vec<u32>, Vec<u32>);
fn parse_card(s: &str) -> Option<(Card, &'_ str)> {
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

fn parse_game(s: &str) -> Option<Vec<Card>> {
    s.lines()
        .map(|s| {
            let (c, s) = parse_card(s)?;
            parser::parse_eof(s)?;
            Some(c)
        })
        .collect()
}

fn num_matching(w: &[u32], m: &[u32]) -> usize {
    m.iter().filter(|&x| w.contains(x)).count()
}

pub fn part1(input: &str) -> Result<String, String> {
    let game = parse_game(input).ok_or("Failed to parse game")?;
    let res = game
        .iter()
        .map(|(_, v1, v2)| {
            let num = num_matching(v1, v2);
            if num == 0 {
                0
            } else {
                (2_usize).pow((num - 1) as u32)
            }
        })
        .sum::<usize>();
    Ok(res.to_string())
}

fn matchings(game: &[(u32, Vec<u32>, Vec<u32>)]) -> Vec<usize> {
    game.iter()
        .map(|(_, v1, v2)| (num_matching(v1, v2)))
        .collect()
}

fn create_instances(cards: &mut [usize], current: usize, matches: &[usize]) {
    for mat in 1..=matches[current] {
        cards[current + mat] += 1;
        create_instances(cards, current + mat, matches);
    }
}

pub fn part2(input: &str) -> Result<String, String> {
    let game = parse_game(input).ok_or("Failed to parse game")?;
    let matches = matchings(&game);
    let mut instances: Vec<usize> = iter::repeat(1).take(game.len()).collect();
    for card in 0..game.len() {
        create_instances(&mut instances, card, &matches);
    }
    let res = instances.iter().sum::<usize>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "13");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "30");
    }
}
