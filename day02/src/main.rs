use std::collections::BTreeMap;

fn main() {
    let counts: BTreeMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    let sum: u64 = util::input_lines!().unwrap().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(':');

        let id: u64;
        text_io::scan!(split.next().unwrap().bytes() => "Game {}", id);

        let is_valid = split.next().unwrap().split(';').all(|set| set.split(", ").all(|entry| {
            let mut entry_split = entry.split_whitespace();
            let count = entry_split.next().expect(entry).parse::<u64>().expect(entry);
            let color = entry_split.next().unwrap();
            count <= counts[color]
        }));

        if is_valid { id } else { 0 }
    }).sum();
    println!("Part 1: {:?}", sum);

    let sum: u64 = input_lines!().unwrap().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(':');

        let mut counts: BTreeMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into_iter().collect();

        for set in split.next_back().unwrap().split(';') {
            for entry in set.split(", ") {
                let mut entry_split = entry.split_whitespace();
                let count = entry_split.next().expect(entry).parse::<u64>().expect(entry);
                let color = entry_split.next().unwrap();
                counts.entry(color).and_modify(|c| *c = count.max(*c));
            }
        }

        counts.values().product::<u64>()
    }).sum();
    println!("Part 2: {:?}", sum);
}
