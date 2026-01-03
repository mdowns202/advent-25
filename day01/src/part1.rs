use std::fmt::Display;
use std::fs;

const DEFAULT_POS: i32 = 50;
const TARGET_POS: i32 = 0;
const DEFAULT_PASSWORD: i32 = 0;
static PUZZLE_PATH: &str = "day01/src/test.txt";

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    steps: i32,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.steps)
    }
}

pub struct Lock {
    position: i32,
    pub password: i32,
    pub secret: i32,
}

impl Lock {
    pub fn new() -> Self {
        Self {
            position: DEFAULT_POS,
            password: DEFAULT_PASSWORD,
            secret: DEFAULT_PASSWORD,
        }
    }

    pub fn crack(&mut self, instructions: Vec<Instruction>) {
        instructions.iter().for_each(|instruction| {
            println!(
                "- Current Position: {}, Instruction: {}",
                self.position, instruction
            );
            match instruction.direction {
                Direction::Left => self.position -= instruction.steps,
                Direction::Right => self.position += instruction.steps,
            }

            while self.position <= 0 || self.position > 99 {
                match self.position {
                    TARGET_POS => {
                        self.password += 1;
                        println!("Password: {}", self.password);
                        break;
                    }
                    ..0 => {
                        self.position += 100;
                        self.position.eq(&0).then(|| {
                            self.secret += 1;
                            println!(
                                "Current Position: {}, Secret: {}",
                                self.position, self.secret
                            );
                        });
                    }
                    100.. => {
                        self.position -= 100;
                        self.position.eq(&0).then(|| {
                            self.secret += 1;
                            println!(
                                "Current Position: {}, Secret: {}",
                                self.position, self.secret
                            );
                        });
                    }
                    _ => (),
                }
            }
        });
    }
}

pub fn solve_safe_password() {
    let mut lock = Lock::new();
    let instructions: Vec<Instruction> = get_instructions();

    lock.crack(instructions);
    println!("Password: {}", lock.password);
}

pub fn get_instructions() -> Vec<Instruction> {
    let input = fs::read_to_string(PUZZLE_PATH).unwrap();
    input
        .lines()
        .map(|line| {
            let (direction, steps) = line.split_at(1);
            Instruction {
                direction: match direction {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction"),
                },
                steps: steps.parse().unwrap(),
            }
        })
        .collect()
}
