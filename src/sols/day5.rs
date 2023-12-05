use crate::utils::parser;
use std::collections::HashMap;
use std::iter::repeat;
#[derive(Debug, Clone, Copy)]
struct Rng(u32, u32, u32);
#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    sections: HashMap<String, Vec<Rng>>,
}
fn parse_input(input: &str) -> Option<Almanac> {
    let (_, s) = parser::parse_const(input, "seeds: ")?;
    let (seeds, s) = parser::many(s, |s| parser::parse_unsigned_int(s.trim()))?;
    let sections = s
        .trim_start()
        .split("\n\n")
        .map(|s: &str| {
            let (name, s) = parser::many(s, |s| {
                if s.starts_with(':') {
                    None
                } else {
                    Some((s.chars().next()?, &s[1..]))
                }
            })?;
            let (_, s) = parser::parse_const(s.trim_start(), ":")?;
            let ranges = parser::many(s.trim_start(), |s| {
                let (src_start, s) = parser::parse_unsigned_int(s)
                    .and_then(|(x, s)| parser::parse_const(s, " ").map(|(_, s)| (x, s)))?;
                let (dst_start, s) = parser::parse_unsigned_int(s)
                    .and_then(|(x, s)| parser::parse_const(s, " ").map(|(_, s)| (x, s)))?;
                let (len, s) = parser::parse_unsigned_int(s)?;
                Some((Rng(src_start, dst_start, len), s.trim_start()))
            })?;
            Some((name.iter().collect(), ranges.0))
        })
        .collect::<Option<HashMap<String, Vec<Rng>>>>()?;
    Some(Almanac { seeds, sections })
}
pub fn part1(input: &str) -> Result<String, String> {
    let data = parse_input(input).ok_or("Failed to parse file")?;
    let mut locations: Vec<u32> = repeat(0).take(data.seeds.len()).collect();
    for (idx, seed) in data.seeds.iter().enumerate() {
        let maps = [
            "seed-to-soil map",
            "soil-to-fertilizer map",
            "fertilizer-to-water map",
            "water-to-light map",
            "light-to-temperature map",
            "temperature-to-humidity map",
            "humidity-to-location map",
        ];
        let mut src = *seed;
        for map in maps {
            let secs = &data.sections[map];
            for &Rng(dst_start, src_start, len) in secs {
                if src >= src_start && src <= src_start + len {
                    src = (src - src_start) + dst_start;
                    break;
                }
            }
        }
        locations[idx] = src;
    }
    Ok(locations.iter().min().unwrap().to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
