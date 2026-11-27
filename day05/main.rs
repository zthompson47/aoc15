use std::collections::HashMap;

fn main() {
    const VOWELS: &str = "aeiou";

    let part1 = include_str!("input.txt")
        .lines()
        .filter(|x| {
            for bad in ["ab", "cd", "pq", "xy"] {
                if x.contains(bad) {
                    return false;
                }
            }

            /*
            let vowels = x
                .chars()
                .filter(|x| VOWELS.contains(*x))
                .collect::<Vec<_>>()
                .len();

            let mut repeat = false;
            for i in 0..x.len() - 1 {
                // Not UTF-8 safe?
                if x.as_bytes()[i] == x.as_bytes()[i + 1] {
                    repeat = true;
                }
            }
            */

            let mut ch = x.chars();
            let mut prev = ch.next().unwrap();
            let mut vowels = if VOWELS.contains(prev) { 1 } else { 0 };
            let mut repeat = false;

            for cur in ch {
                if VOWELS.contains(cur) {
                    vowels += 1;
                }
                if prev == cur {
                    repeat = true;
                }
                prev = cur;
            }

            vowels >= 3 && repeat
        })
        .collect::<Vec<_>>()
        .len();

    println!("Part 1: {part1}");

    let part2 = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter(|line| {
            let mut pairs = HashMap::<(char, char), Vec<usize>>::new();

            // Check non-overlapping pairs of characters
            let mut has_pairs = false;
            for i in 0..line.len() - 1 {
                let pair = (line[i], line[i + 1]);
                pairs
                    .entry(pair)
                    .and_modify(|x| {
                        for j_idx in 0..x.len() {
                            if (x[j_idx] as i32 - i as i32).abs() > 1 {
                                x.push(i);
                                // hmmmmm....
                                has_pairs = true;
                            }
                        }
                    })
                    .or_insert(Vec::from([i]));
            }

            // Check for a letter repeating with one letter between the repeats
            let mut repeat = false;
            let mut i = 0;
            while i < line.len() - 2 {
                if line[i] == line[i + 2] {
                    repeat = true;
                }
                i += 1;
            }

            repeat && has_pairs
        })
        .collect::<Vec<_>>()
        .len();

    println!("Part 2: {part2}");
}
