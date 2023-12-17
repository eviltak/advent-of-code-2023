use std::io::{BufRead, BufReader};
use std::ops::Range;

struct Entry {
    source: u64,
    dest: u64,
    len: u64,
}

impl Entry {
    fn contains(&self, x: u64) -> bool {
        self.source <= x && x < self.source + self.len
    }

    fn map(&self, x: u64) -> u64 {
        if self.contains(x) {
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

    fn first_entry_ending_after(&self, x: u64) -> Option<&Entry> {
        self.entries
            .get(self.entries.partition_point(|e| x >= e.source + e.len))
    }

    fn map(&self, x: u64) -> u64 {
        self.first_entry_ending_after(x)
            .filter(|e| e.contains(x))
            .map_or(x, |r| r.map(x))
    }

    fn map_range(&self, mut r: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = vec![];

        while let (false, Some(e)) = (r.is_empty(), self.first_entry_ending_after(r.start)) {
            if r.end <= e.source {
                break;
            }

            if !e.contains(r.start) {
                ranges.push(r.start..e.source);
                r.start = e.source;
            }

            let new_start = r.end.min(e.source + e.len);
            ranges.push(e.map(r.start)..e.map(new_start - 1) + 1);
            r.start = new_start;
        }

        if !r.is_empty() {
            ranges.push(r);
        }

        ranges
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

    let part2 = seeds.chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flat_map(|seed_range|
            maps.iter().fold(vec![seed_range], |ranges, m| ranges.into_iter().flat_map(|r| m.map_range(r)).collect())
        )
        .map(|r| r.start)
        .min().unwrap();

    println!("Part 2: {}", part2);
}
