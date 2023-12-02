use crate::utils::parser;

#[derive(Debug)]
struct Set(u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_cube(s: &str) -> Option<((u32, &'_ str), &'_ str)> {
    let (n, s) = parser::parse_unsigned_int(s)?;
    let s = s.trim_start();
    let (color, s) = parser::parse_const(s, "red")
        .or_else(|| parser::parse_const(s, "green"))
        .or_else(|| parser::parse_const(s, "blue"))?;
    Some(((n, color), s))
}

fn parse_set(s: &str) -> Option<(Set, &'_ str)> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let (cubes, s) = parser::parse_collection(s, ",", parse_cube)?;
    for (n, color) in cubes {
        match color {
            "red" => r += n,
            "green" => g += n,
            "blue" => b += n,
            _ => panic!("cube color must be red, green or blue"),
        }
    }
    Some((Set(r, g, b), s))
}

fn parse_game(s: &str) -> Option<(Game, &'_ str)> {
    let (_, s) = parser::parse_const(s, "Game")?;
    let (id, s) = parser::parse_unsigned_int(s.trim_start())?;
    let (_, s) = parser::parse_const(s.trim_start(), ":")?;
    let s = s.trim_start();
    let (sets, s) = parser::parse_collection(s, ";", parse_set)?;
    Some((Game { id, sets }, s))
}

fn parse_file(s: &str) -> Option<Vec<Game>> {
    s.lines()
        .map(|line| {
            let (g, s) = parse_game(line)?;
            parser::parse_eof(s)?;
            Some(g)
        })
        .collect()
}

pub fn part1(input: &str) -> Result<String, String> {
    let games = parse_file(input).ok_or("Failed to parse file".to_string())?;
    let res = games
        .iter()
        .filter_map(|g| {
            g.sets
                .iter()
                .all(|&Set(r, g, b)| r <= 12 && g <= 13 && b <= 14)
                .then_some(g.id)
        })
        .sum::<u32>();
    Ok(res.to_string())
}
pub fn part2(input: &str) -> Result<String, String> {
    let games = parse_file(input).ok_or("Failed to parse file".to_string())?;
    let res: u32 = games
        .iter()
        .map(|game| {
            let (mut mr, mut mg, mut mb) = (0, 0, 0);
            for &Set(r, g, b) in &game.sets {
                if r > mr {
                    mr = r
                }
                if g > mg {
                    mg = g
                }
                if b > mb {
                    mb = b
                }
            }
            mr * mg * mb
        })
        .sum();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(example_input).unwrap(), "8");
    }
    #[test]
    fn test_part2() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(example_input).unwrap(), "2286");
    }
}
