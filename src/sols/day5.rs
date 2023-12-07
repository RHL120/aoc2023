use core::ops::RangeInclusive;
type Mapping = Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>;
#[derive(Debug, Default)]
struct Map {
    mappings: Mapping,
}

impl Map {
    fn get_value(&self, v: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|(dst, src)| src.contains(&v).then(|| (v - src.start() + dst.start())))
            .unwrap_or(v)
    }
    #[allow(dead_code)]
    fn restrict(&self, rng: &RangeInclusive<usize>) -> Map {
        let mut ret = Map {
            mappings: Vec::new(),
        };
        for (dst, src) in &self.mappings {
            let start = *src.start().max(rng.start());
            let end = *src.end().min(rng.end());
            if start <= end {
                ret.mappings.push((dst.clone(), start..=end));
            }
        }
        ret
    }
    fn compose(&self, other: &Self) -> Self {
        let mut res = Map {
            mappings: Vec::new(),
        };
        let mappings = other
            .mappings
            .iter()
            .map(|(dst, src)| {
                let mappings = other
                    .restrict(dst)
                    .mappings
                    .iter()
                    .map(|(dst, _)| (dst.clone(), src.clone()))
                    .collect();
                Map { mappings }
            })
            .collect::<Vec<_>>();
        for mut map in mappings {
            res.mappings.append(&mut map.mappings);
        }
        todo!()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

fn parse_input(src: &str) -> Option<Almanac> {
    let mut sections = src.split("\n\n");
    let seeds = sections
        .next()?
        .strip_prefix("seeds: ")?
        .split(' ')
        .map(|x| x.parse::<usize>().ok())
        .collect::<Option<Vec<usize>>>()?;
    let maps = sections
        .map(|section| {
            let mut ranges = section.lines();
            ranges.next();
            let mappings = ranges
                .map(|range| {
                    let mut values = range.split(' ').map(|x| x.parse::<usize>().ok());
                    let dst = values.next()??;
                    let src = values.next()??;
                    let len = values.next()??;
                    Some((dst..=dst + len, src..=src + len))
                })
                .collect::<Option<_>>()?;
            Some(Map { mappings })
        })
        .collect::<Option<_>>()?;
    Some(Almanac { seeds, maps })
}

pub fn part1(src: &str) -> Result<String, String> {
    let data = parse_input(src).ok_or("Failed to parse file")?;
    let res = data
        .seeds
        .iter()
        .map(|&seed| {
            data.maps
                .iter()
                .fold(seed, move |seed, map| map.get_value(seed))
        })
        .min()
        .unwrap();
    Ok(res.to_string())
}

pub fn part2(src: &str) -> Result<String, String> {
    let _data = parse_input(src).ok_or("Failed to parse file")?;
    todo!()
}
