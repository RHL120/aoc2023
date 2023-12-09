fn parse_input(input: &str) -> Option<Vec<Vec<i128>>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i128>().ok())
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
}

fn diff(seq: &[i128]) -> (Vec<i128>, bool) {
    let mut zeros = seq.first().map(|&x| x == 0).unwrap_or(false);
    let mut ret = Vec::new();
    for i in 1..seq.len() {
        ret.push(seq[i] - seq[i - 1]);
        zeros &= ret[i - 1] == 0;
    }
    (ret, zeros)
}

fn diff_seq(seq: &[i128]) -> Vec<Vec<i128>> {
    let mut zeros = false;
    let mut ret = vec![seq.to_vec()];
    while !zeros {
        let nsq;
        (nsq, zeros) = diff(ret.last().unwrap());
        ret.push(nsq);
    }
    ret
}

pub fn part1(input: &str) -> Result<String, String> {
    let data = parse_input(input).ok_or("Failed to parse input")?;
    let s: i128 = data
        .iter()
        .map(|x| {
            let mut dsq = diff_seq(x);
            let mut i = dsq.len();
            while i > 0 {
                i -= 1;
                if i == dsq.len() - 1 {
                    dsq[i].push(0);
                } else {
                    let last = *dsq[i].last().unwrap();
                    let dlast = *dsq[i + 1].last().unwrap();
                    dsq[i].push(last + dlast)
                }
            }
            *dsq[0].last().unwrap()
        })
        .sum();
    Ok(s.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let data = parse_input(input).ok_or("Failed to parse input")?;
    let s: i128 = data
        .iter()
        .map(|x| {
            let mut dsq = diff_seq(x);
            let mut i = dsq.len();
            while i > 0 {
                i -= 1;
                if i == dsq.len() - 1 {
                    dsq[i].insert(0, 0);
                } else {
                    let last = *dsq[i].first().unwrap();
                    let dlast = *dsq[i + 1].first().unwrap();
                    println!("{last} - {dlast} = {}", last - dlast);
                    dsq[i].insert(0, last - dlast)
                }
            }
            println!("{}", *dsq[0].last().unwrap());
            *dsq[0].first().unwrap()
        })
        .sum();
    Ok(s.to_string())
}
