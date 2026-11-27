fn main() {
    let mut floor_negative_one = None;
    let part1 = include_str!("input.txt").trim().chars().enumerate().fold(0, |mut floor, (i, x)| {
        floor = match x {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => panic!(),
        };
        if floor == -1 {
            // How many stops until first time at floor negative one
            floor_negative_one.get_or_insert(i + 1); // 1-based index for answer
        }
        floor
    });

    println!("Part 1: {part1}");
    println!("Part 2: {}", floor_negative_one.unwrap());
}
