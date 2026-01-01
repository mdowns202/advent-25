use super::part1::{get_instructions, Instruction, Lock};

pub fn solve_with_secret_method() {
    let mut lock = Lock::new();
    let instructions: Vec<Instruction> = get_instructions();

    lock.crack(instructions);
    println!("Password: {}", lock.secret);
}
