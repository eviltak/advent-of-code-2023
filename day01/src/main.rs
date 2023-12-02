#[macro_use]
extern crate util;

fn main() {
    let sum: u32 = input_lines!().unwrap().map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();
        let first = chars.find(char::is_ascii_digit).and_then(|d| d.to_digit(10)).unwrap();
        let last = chars.rfind(char::is_ascii_digit).and_then(|d| d.to_digit(10)).unwrap_or(first);
        first * 10 + last
    }).sum();
    println!("Part 1: {:?}", sum);

    let words = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let sum: u32 = input_lines!().unwrap().map(|line| {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        let find_digit = |i: usize| -> Option<_> {
            bytes[i].is_ascii_digit().then(|| (bytes[i] - b'0') as u32)
                .or_else(|| words.iter().find(|(w, _)| bytes[i..].starts_with(w.as_bytes())).map(|(_, d)| *d))
        };
        let first = (0..bytes.len()).find_map(find_digit).expect(&line);
        let last = (0..bytes.len()).rev().find_map(find_digit).unwrap_or(first);
        first * 10 + last
    }).sum();
    println!("Part 2: {:?}", sum);
}
