use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
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

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    kind: u8,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => Some(self.cards.cmp(&other.cards)),
            x => Some(x),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let hs = cards
            .iter()
            .map(|x| (x, cards.iter().filter(move |&y| x == y).count()))
            .collect::<HashMap<_, _>>();
        if hs.iter().any(|(_, &count)| count == 5) {
            return Hand { cards, kind: 6 };
        }
        if hs.iter().any(|(_, &count)| count == 4) {
            return Hand { cards, kind: 5 };
        }
        if hs.iter().any(|(_, &count)| count == 3) && hs.iter().any(|(_, &count)| count == 2) {
            return Hand { cards, kind: 4 };
        }
        if hs.iter().any(|(_, &count)| count == 3) {
            return Hand { cards, kind: 3 };
        }
        if hs.iter().filter(|(_, &count)| count == 2).count() == 2 {
            return Hand { cards, kind: 2 };
        }
        if hs.iter().any(|(_, &count)| count == 2) {
            return Hand { cards, kind: 1 };
        }
        Hand { cards, kind: 0 }
    }
}

fn parse_input(input: &str) -> Option<Vec<(usize, Hand)>> {
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
            Some((bid, Hand::new(hand)))
        })
        .collect::<Option<Vec<_>>>()
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut game = parse_input(input).ok_or("Failed")?;
    game.sort_by(|(_, c), (_, c2)| match c.cmp(c2) {
        Ordering::Equal => c.cards.cmp(&c2.cards),
        x => x,
    });
    let res: usize = game
        .iter()
        .enumerate()
        .map(|(idx, (bid, _))| (idx + 1) * bid)
        .sum();
    println!("{:#?}", game);
    println!("{}", Card::Two < Card::Three);
    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    todo!()
}
