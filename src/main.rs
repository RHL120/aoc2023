use aoc2023::sols::*;
use std::env;
use std::fs;

type Solver = fn(&str) -> Result<String, String>;

fn parse_args() -> Option<(usize, usize, String)> {
    let args: Vec<String> = env::args().collect();
    let day: usize = args.get(1).and_then(|x| x.parse().ok())?;
    let part: usize = args.get(2).and_then(|x| x.parse().ok())?;
    let input: &String = args.get(3)?;
    if day == 0 || part == 0 {
        None
    } else {
        Some((day, part, input.to_string()))
    }
}

const SOLVERS: &[&[Solver]] = &[
    &[day1::part1, day1::part2],
    &[day2::part1, day2::part2],
    &[day3::part1, day3::part2],
];
fn main() -> Result<(), String> {
    let usage_string = "Usage: ./aoc2023 <day> <part> <input>";
    let (day, part, input_path) = parse_args().ok_or(usage_string)?;
    let input =
        fs::read_to_string(&input_path).map_err(|_| format!("Failed to read {input_path}"))?;
    let &day_solutions = SOLVERS
        .get(day - 1)
        .ok_or_else(|| format!("Failed to get day {day}"))?;
    let solution = day_solutions
        .get(part - 1)
        .ok_or_else(|| format!("Failed to get part {part}"))?;
    println!(
        "The solution for day {day} part {part} is:\n{}",
        solution(&input)?
    );
    Ok(())
}
