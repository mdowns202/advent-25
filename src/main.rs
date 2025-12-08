use std::fs;
use std::fmt::Display;

const DEFAULT_POS: i32 = 50;
const TARGET_POS: i32 = 0;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

struct Lock {
    position: i32,
    instructions: Vec<Instruction>,
    password: i32
}

impl Lock {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { position: DEFAULT_POS, instructions, password: 0 }
    }

    fn crack(&mut self)  {
        for instruction in self.instructions.iter() {
            match instruction.direction {
                Direction::Left => self.position -= instruction.steps,
                Direction::Right => self.position += instruction.steps,
            }
            while self.position < 0 || self.position > 99 {
                match self.position {
                    ..0 => self.position += 100,
                    100.. => self.position -= 100,
                    _ => (),
                }
            }
            self.position.eq(&TARGET_POS).then(|| self.password += 1);
        }
    }
}

fn main() {
    let input = fs::read_to_string("puzzle.txt").unwrap();

    let instructions: Vec<Instruction> = input.lines()
        .map(|line| {
            let (direction, steps) = line.split_at(1);
            Instruction {
                direction: match direction {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction")
                },
                steps: steps.parse().unwrap()
            }
        })
        .collect();

    let mut  lock = Lock::new(instructions);
    lock.crack();
    println!("Password: {}", lock.password);
}
