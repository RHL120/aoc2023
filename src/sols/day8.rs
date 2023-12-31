use crate::utils::math;
use crate::utils::parser;
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Network<'a> {
    instructions: Vec<Instruction>,
    paths: HashMap<&'a str, (&'a str, &'a str)>,
}

fn simulate<'a>(net: &Network<'a>, start: &'a str, condition: impl Fn(&'a str) -> bool) -> u128 {
    let mut curr = net.paths[start];
    let count = net
        .instructions
        .iter()
        .cycle()
        .map_while(|e| {
            let (l, r) = curr;
            let next_name = match e {
                Instruction::Left => l,
                Instruction::Right => r,
            };
            curr = net.paths[next_name];
            (!condition(next_name)).then_some(net.paths[next_name])
        })
        .count();
    count as u128 + 1
}

fn parse_input(input: &str) -> Option<Network> {
    let mut lines = input.lines();
    let instructions = lines
        .next()?
        .chars()
        .map(|c| match c {
            'L' => Some(Instruction::Left),
            'R' => Some(Instruction::Right),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    lines.next()?;
    let paths = lines
        .map(|line| {
            let (name, line) = (line.get(..3)?, line.get(3..)?);
            let (_, line) = parser::parse_const(line, " = ")?;
            let (left, right) = parser::parse_delimited(line, "(", ")", true, |s| {
                let (left, s) = (s.get(..3)?, s.get(3..)?);
                let (_, s) = parser::parse_const(s, ", ")?;
                let (right, s) = (s.get(..3)?, s.get(3..)?);
                (!left.is_empty() && !right.is_empty()).then_some(((left, right), s))
            })?
            .0;
            Some((name, (left, right)))
        })
        .collect::<Option<HashMap<_, _>>>()?;
    Some(Network {
        instructions,
        paths,
    })
}

pub fn part1(input: &str) -> Result<String, String> {
    let net = parse_input(input).ok_or("Failed to parse input")?;
    Ok(simulate(&net, "AAA", |x| x == "ZZZ").to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let net = parse_input(input).ok_or("Failed to parse input")?;
    let steps: Vec<u128> = net
        .paths
        .keys()
        .filter(|&&k| (k.ends_with('A')))
        .map(|&k| simulate(&net, k, |x| x.ends_with('Z')))
        .collect();
    let res = steps.iter().fold(1, |n, m| math::lcm(n, *m));
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
        assert_eq!(part1(input).unwrap(), "6")
    }
    #[test]
    fn test_part2() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(part2(input).unwrap(), "6")
    }
}
