use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn cmp(self, other: Card, part2: bool) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if part2 {
            if self == Card::J {
                return Ordering::Less;
            }
            if other == Card::J {
                return Ordering::Greater;
            }
        }
        (self as usize).cmp(&(other as usize))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    part2: bool,
    cards: Vec<Card>,
    kind: u8,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_eq!(self.part2, other.part2);
        let ord = match self.kind.cmp(&other.kind) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(&c1, &c2)| (c1 != c2).then(|| c1.cmp(c2, self.part2)))
                .unwrap_or(Ordering::Equal),
            x => x,
        };
        Some(ord)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn new(cards: Vec<Card>, part2: bool) -> Self {
        let hs = cards
            .iter()
            .map(|x| (x, cards.iter().filter(move |&y| x == y).count()))
            .collect::<HashMap<_, _>>();
        if hs.iter().any(|(_, &count)| count == 5) {
            return Hand {
                cards,
                kind: 6,
                part2,
            };
        }
        if hs.iter().any(|(_, &count)| count == 4) {
            return Hand {
                cards,
                kind: 5,
                part2,
            };
        }
        if hs.iter().any(|(_, &count)| count == 3) && hs.iter().any(|(_, &count)| count == 2) {
            return Hand {
                cards,
                kind: 4,
                part2,
            };
        }
        if hs.iter().any(|(_, &count)| count == 3) {
            return Hand {
                cards,
                kind: 3,
                part2,
            };
        }
        if hs.iter().filter(|(_, &count)| count == 2).count() == 2 {
            return Hand {
                cards,
                kind: 2,
                part2,
            };
        }
        if hs.iter().any(|(_, &count)| count == 2) {
            return Hand {
                cards,
                kind: 1,
                part2,
            };
        }
        Hand {
            cards,
            kind: 0,
            part2,
        }
    }
}

fn parse_input(input: &str, part2: bool) -> Option<Vec<(usize, Hand)>> {
    input
        .lines()
        .map(|line| {
            let mut data = line.split(" ");
            let hand = data
                .next()?
                .chars()
                .map(|c| {
                    use Card::*;
                    match c {
                        '2' => Some(Two),
                        '3' => Some(Three),
                        '4' => Some(Four),
                        '5' => Some(Five),
                        '6' => Some(Six),
                        '7' => Some(Seven),
                        '8' => Some(Eight),
                        '9' => Some(Nine),
                        'T' => Some(T),
                        'J' => Some(J),
                        'Q' => Some(Q),
                        'K' => Some(K),
                        'A' => Some(A),
                        _ => None,
                    }
                })
                .collect::<Option<Vec<_>>>()?;
            let bid = data.next()?.parse::<usize>().ok()?;
            Some((bid, Hand::new(hand, part2)))
        })
        .collect::<Option<Vec<_>>>()
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut game = parse_input(input, false).ok_or("Failed to parse input")?;
    game.sort_by(|(_, h1), (_, h2)| h1.cmp(h2));
    let res: usize = game
        .iter()
        .enumerate()
        .map(|(idx, (bid, _))| (idx + 1) * bid)
        .sum();
    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
