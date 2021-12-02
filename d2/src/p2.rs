#![warn(clippy::pedantic)] // because it is what it is

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Copy, Default)]
struct Position {
    pub depth: i32,
    pub distance: i32,
    pub aim: i32,
}

impl Position {
    /// forward X increases distance by X
    /// down X increases the depth by X
    /// up X decreases the depth by X
    pub fn apply(&mut self, action: &str) -> Option<()> {
        let (action, amount) = action.split_once(" ")?;
        let amount: i32 = amount.parse().unwrap();

        match action {
            "forward" => {
                self.distance += amount;
                self.depth += self.aim * amount;
            }
            "down" => self.aim += amount,
            "up" => self.aim -= amount,
            _ => panic!("unknown action: {}", action),
        };

        Some(())
    }
}

fn main() {
    let mut pos = Position::default();
    let lines = INPUT.lines().filter(|x| !x.is_empty());

    for line in lines {
        pos.apply(line);
    }

    dbg!(pos.depth * pos.distance);
}
