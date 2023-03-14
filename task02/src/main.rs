use std::str::FromStr;
use std::fs;
use crate::Direction::{DOWN, FORWARD, UP};

#[derive(Debug)]
enum Direction {
    FORWARD(u32),
    DOWN(u32),
    UP(u32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space = s.find(' ').expect("Direction is missing space");
        let dir = &s[..space];
        let num: u32 = u32::from_str(&s[space + 1..])
            .expect("Direction must be followed by number");
        match dir {
            "forward" => Ok(FORWARD(num)),
            "down" => Ok(DOWN(num)),
            "up" => Ok(UP(num)),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Sub {
    horizontal: u32,
    vertical: u32,
    aim: u32,
}
impl Default for Sub {
    fn default() -> Self {
        Self {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }
}
impl Sub {
    fn move_straight(&self, direction: &Direction) -> Self {
        match direction {
            DOWN(num) => Self { vertical: self.vertical + num, ..*self },
            UP(num) => Self { vertical: self.vertical - num, ..*self },
            FORWARD(num) => Self { horizontal: self.horizontal + num, ..*self }
        }
    }

    fn move_aim(&self, direction: &Direction) -> Self {
        match direction {
            DOWN(num) => Self { aim: self.aim + num, ..*self },
            UP(num) => Self { aim: self.aim - num, ..*self },
            FORWARD(num) => Self {
                horizontal: self.horizontal + num,
                vertical: self.vertical + self.aim * num,
                ..*self
            }
        }
    }

    fn result(&self) -> u32 {
        self.horizontal * self.vertical
    }
}

fn main() {
    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };
    let directions: Vec<Direction> = input.lines()
        .map(|line| Direction::from_str(line).unwrap())
        .collect();

    let (part1, part2) = directions.iter()
        .fold((Sub::default(), Sub::default()),
              |sub, direction|
                  (sub.0.move_straight(direction), sub.1.move_aim(direction)));
    println!("{:?}, {}", part1, part1.result());
    println!("{:?}, {}", part2, part2.result());
}
