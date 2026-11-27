fn main() {
    let mut state = State::new();
    while !state.is_coin(Part::One) {
        state.advance();
    }
    println!("Part 1: {}", state.step);

    let mut state = State::new();
    while !state.is_coin(Part::Two) {
        state.advance();
    }
    println!("Part 2: {}", state.step);
}

enum Part {
    One,
    Two,
}

struct State {
    step: u32,
    key: String,
}

impl State {
    const SEED: &str = "yzbqklnj";

    fn new() -> Self {
        let step = 1;
        let mut key = String::from(State::SEED);
        key.push_str(&step.to_string());
        State { step, key }
    }

    fn advance(&mut self) {
        self.key.clear();
        self.key.push_str(State::SEED);
        self.step += 1;
        self.key.push_str(&self.step.to_string());
    }

    fn is_coin(&self, part: Part) -> bool {
        let hash = md5::compute(self.key.bytes().collect::<Vec<u8>>());
        //let hash_str = format!("{:?}", hash);
        //match part {
        //    Part::One => &hash_str[0..5] == "00000",
        //    Part::Two => &hash_str[0..6] == "000000",
        //}
        match part {
            Part::One => hash[0..2] == [0, 0] && hash[2] < 16,
            Part::Two => hash[0..3] == [0, 0, 0],
        }
    }
}
