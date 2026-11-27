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

            let mut prev = None;
            let mut vowels = 0;
            let mut repeat = false;
            for cur in x.chars() {
                // Contains at least 3 vowels?
                if VOWELS.contains(cur) {
                    vowels += 1;
                }
                // Has two identical characters in a row?
                if prev == Some(cur) {
                    repeat = true;
                }
                prev = Some(cur);
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
            let mut pairs = HashMap::<(char, char), usize>::new();
            let mut repeat = false;
            let mut has_pairs = false;

            for cur_idx in 0..line.len() - 1 {
                // Check for a letter repeating with one letter between the repeats
                if cur_idx < line.len() - 2 && line[cur_idx] == line[cur_idx + 2] {
                    repeat = true;
                }

                // Check non-overlapping pairs of characters
                let pair = (line[cur_idx], line[cur_idx + 1]);
                pairs
                    .entry(pair)
                    .and_modify(|pair_idx| {
                        if pair_idx.abs_diff(cur_idx) > 1 {
                            has_pairs = true;
                        }
                    })
                    .or_insert(cur_idx);
            }

            repeat && has_pairs
        })
        .collect::<Vec<_>>()
        .len();

    println!("Part 2: {part2}");
}
