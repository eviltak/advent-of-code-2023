use std::collections::{HashSet, VecDeque};

fn main() {
    let mut copies = VecDeque::new();
    let mut part1: u64 = 0;
    let mut part2: u32 = 0;

    for  line in util::input_lines!().unwrap() {
        let line = line.unwrap();
        let (winning, card) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<_> = winning.trim().split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        let card: HashSet<_> = card.trim().split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        let n = winning.intersection(&card).count();

        let c = copies.pop_front().unwrap_or(0) + 1;
        part2 += c;

        for j in 0..n {
            if j == copies.len() {
                copies.push_back(c);
            } else {
                copies[j] += c;
            }
        }

        part1 += if n > 0 { 2u64.pow(n as u32 - 1) } else { 0 };
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
