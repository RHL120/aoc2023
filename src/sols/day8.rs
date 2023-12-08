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
                println!("{s}");
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
    println!("{:#?}", net);
    let mut curr = net.paths["AAA"];
    let res = net
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
            (next_name != "ZZZ").then_some(net.paths[next_name])
        })
        .count();
    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
