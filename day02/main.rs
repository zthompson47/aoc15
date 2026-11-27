fn main() {
    let (part1, part2) = include_str!("input.txt")
        .lines()
        .map(|line| {
            let parts = line
                .split('x')
                .map(|dimension| dimension.parse().unwrap())
                .collect::<Vec<i32>>();
            (parts[0], parts[1], parts[2])
        })
        .fold((0, 0), |(mut part1, mut part2), (l, w, h)| {
            let (a, b, c) = (2 * l * w, 2 * l * h, 2 * w * h);
            part1 += a + b + c + a.min(b.min(c)) / 2;

            let smallest_perimeter = (l + w + h - l.max(w.max(h))) * 2;
            let volume = l * w * h;
            part2 += smallest_perimeter + volume;

            (part1, part2)
        });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
