use std::collections::{BTreeMap, HashSet};

fn main() {
    let schematic: Vec<_> = util::input_lines!().unwrap().map(|l| l.unwrap().into_bytes()).collect();
    let n = schematic.len();
    let m = schematic[0].len();

    let neighbors = |r: usize, c: usize| {
        [
            (r.wrapping_sub(1), c.wrapping_sub(1)),
            (r.wrapping_sub(1), c),
            (r.wrapping_sub(1), c + 1),
            (r, c.wrapping_sub(1)),
            (r, c + 1),
            (r + 1, c.wrapping_sub(1)),
            (r + 1, c),
            (r + 1, c + 1)
        ]
            .into_iter()
            .filter(|&(r, c)| r < n && c < m)
            .map(|(r, c)| ((r, c), schematic[r][c]))
    };

    let mut s = 0;
    let mut gears = BTreeMap::new();

    for r in 0..n {
        let mut x = 0u64;
        let mut is_valid = false;
        let mut adj_gears = HashSet::new();

        for c in 0..m {
            let char = schematic[r][c];

            if char.is_ascii_digit() {
                x = 10 * x + (char - b'0') as u64;
                is_valid = is_valid || neighbors(r, c).any(|(_, nc)| !nc.is_ascii_digit() && nc != b'.');
                adj_gears.extend(neighbors(r, c).filter(|&(_, nc)| nc == b'*').map(|(loc, _)| loc));
            } else if x != 0 {
                for g in adj_gears.drain() {
                    gears.entry(g).or_insert_with(|| vec![]).push(x);
                }

                if is_valid {
                    s += x;
                }

                is_valid = false;
                x = 0;
            }
        }

        for g in adj_gears.drain() {
            gears.entry(g).or_insert_with(|| vec![]).push(x);
        }

        if is_valid {
            s += x;
        }
    }

    println!("Part 1: {}", s);
    println!("Part 2: {}", gears.values().filter_map(|nums| (nums.len() == 2).then(|| nums.iter().product::<u64>())).sum::<u64>())
}
