use std::collections::HashSet;

fn main() {
    let part1 =
        include_str!("input.txt")
            .trim()
            .chars()
            .fold(State::new(), |mut state, direction| {
                state.move_santa(direction);
                state
            });

    let mut directions = include_str!("input.txt").trim().chars();
    let mut part2 = State::new();
    while let Some(direction) = directions.next() {
        part2.move_santa(direction);
        part2.move_robo_santa(directions.next().unwrap());
    }

    println!("Part 1: {}", part1.visited.len());
    println!("Part 2: {}", part2.visited.len());
}

struct State {
    position: (i32, i32),
    robo_position: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl State {
    fn new() -> Self {
        State {
            position: (0, 0),
            robo_position: (0, 0),
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn move_santa(&mut self, direction: char) {
        progress(&mut self.position, direction);
        self.visited.insert(self.position);
    }

    fn move_robo_santa(&mut self, direction: char) {
        progress(&mut self.robo_position, direction);
        self.visited.insert(self.robo_position);
    }
}

fn progress(position: &mut (i32, i32), direction: char) {
    match direction {
        '^' => position.1 += 1,
        '>' => position.0 += 1,
        '<' => position.0 -= 1,
        'v' => position.1 -= 1,
        _ => unreachable!(),
    }
}
