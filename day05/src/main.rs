use std::io::{BufRead, BufReader};

struct Entry {
    source: u64,
    dest: u64,
    len: u64,
}

impl Entry {
    fn map(&self, x: u64) -> u64 {
        if self.source <= x && x < self.source + self.len {
            x - self.source + self.dest
        } else {
            x
        }
    }
}

impl std::str::FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: [u64; 3] = s.split_ascii_whitespace()
            .map(|s| s.parse::<u64>().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ())?;

        Ok(Entry { dest: split[0], source: split[1], len: split[2] })
    }
}

struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn new(entries: Vec<Entry>) -> Map {
        Map { entries }
    }

    fn map(&self, x: u64) -> u64 {
        self.entries
            .get(self.entries.partition_point(|r| r.source <= x).wrapping_sub(1))
            .map_or(x, |r| r.map(x))
    }
}

fn main() {
    let mut reader = BufReader::new(util::input_file!().unwrap());
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();

    let seeds: Vec<_> = line.split_once(':').unwrap()
        .1.trim().split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    reader.read_line(&mut line).unwrap();
    line.clear();

    let mut entries: Vec<Entry> = vec![];
    let mut maps = vec![];

    while let 1.. = reader.read_line(&mut line).unwrap() {
        if line.trim().is_empty() {
            entries.sort_unstable_by_key(|r| r.source);
            maps.push(Map::new(std::mem::take(&mut entries)));
        } else if line.as_bytes()[0].is_ascii_digit() {
            entries.push(line.parse().unwrap());
        }
        line.clear();
    }

    if !entries.is_empty() {
        entries.sort_unstable_by_key(|r| r.source);
        maps.push(Map::new(std::mem::take(&mut entries)));
    }

    assert!(entries.is_empty());
    assert_eq!(maps.len(), 7);

    let part1 = seeds.iter()
        .map(|&seed| maps.iter().fold(seed, |s, m| m.map(s)))
        .min().unwrap();

    println!("Part 1: {}", part1);
}
