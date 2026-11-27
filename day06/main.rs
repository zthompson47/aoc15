use std::ops::Range;

fn main() {
    let grid = part1();
    //print_grid(&grid1);
    //save_img("part1.png", &grid);
    println!("Part 1: {}", grid.iter().sum::<u32>());

    let grid = part2();
    //print_grid(&grid2);
    //save_img("part2.png", &grid);
    println!("Part 2: {}", grid.iter().sum::<u32>());
}

fn part1() -> Vec<u32> {
    let mut grid = Vec::from([0; 1000 * 1000]);
    for instruction in instructions() {
        for x in instruction.x_range {
            for y in instruction.y_range.clone() {
                let idx = y * 1000 + x;
                match instruction.operation {
                    Operation::TurnOn => grid[idx] = 1,
                    Operation::TurnOff => grid[idx] = 0,
                    Operation::Toggle => {
                        if grid[idx] == 0 {
                            grid[idx] = 1
                        } else {
                            grid[idx] = 0
                        }
                    }
                };
            }
        }
    }
    grid
}

fn part2() -> Vec<u32> {
    let mut grid = Vec::from([0; 1000 * 1000]);
    for instruction in instructions() {
        for x in instruction.x_range {
            for y in instruction.y_range.clone() {
                let idx = y * 1000 + x;
                match instruction.operation {
                    Operation::TurnOn => grid[idx] += 1,
                    Operation::TurnOff => {
                        if grid[idx] > 0 {
                            grid[idx] -= 1
                        }
                    }
                    Operation::Toggle => grid[idx] += 2,
                };
            }
        }
    }
    grid
}

fn instructions() -> Vec<Instruction> {
    include_str!("input.txt")
        .lines()
        .fold(Vec::<Instruction>::new(), |mut acc, line| {
            let first_digit = line.find(|x: char| x.is_ascii_digit()).unwrap();
            let operation = match &line[0..first_digit - 1] {
                "turn on" => Operation::TurnOn,
                "turn off" => Operation::TurnOff,
                "toggle" => Operation::Toggle,
                _ => unreachable!(),
            };
            let (p1, p2) = line[first_digit..].split_once(" through ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();
            let (p1, p2) = (
                (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap()),
                // Puzzle uses inclusive ranges
                (
                    x2.parse::<usize>().unwrap() + 1,
                    y2.parse::<usize>().unwrap() + 1,
                ),
            );
            let x_range = p1.0.min(p2.0)..p1.0.max(p2.0);
            let y_range = p1.1.min(p2.1)..p1.1.max(p2.1);

            acc.push(Instruction {
                operation,
                x_range,
                y_range,
            });
            acc
        })
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    operation: Operation,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

#[allow(unused)]
fn print_grid(grid: &[u32]) {
    for y in 0..1000 {
        for x in 0..1000 {
            let idx = y * 1000 + x;
            if grid[idx] > 0 {
                print!("{}", grid[idx]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[allow(unused)]
fn save_img(filename: &str, grid: &[u32]) {
    let max_idx = *grid.iter().max().unwrap();
    use image::{Rgb, RgbImage};
    let mut img = RgbImage::new(1000, 1000);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let brightness = (grid[y as usize * 1000 + x as usize] as f32 / max_idx as f32) * 256.0;
        *pixel = Rgb([brightness as u8, brightness as u8, brightness as u8]);
    }
    img.save(filename).unwrap();
}
