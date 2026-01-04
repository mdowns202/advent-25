use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err("Invalid operation"),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    nums: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn new() -> Problem {
        Problem {
            nums: vec![],
            operation: Operation::Add,
        }
    }

    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.nums.iter().sum(),
            Operation::Multiply => self.nums.iter().product(),
        }
    }
}

pub fn find_grand_total() {
    let input = include_str!("puzzle.txt");
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(" ").filter(|s| !s.is_empty()).collect())
        .collect();

    let mut problems = vec![Problem::new(); rows[0].len()];

    problems.iter_mut().enumerate().for_each(|(i, p)| {
        rows.iter().take(rows.len() - 1).for_each(|row| {
            p.nums.push(row[i].parse().unwrap());
        });
        p.operation = rows[rows.len() - 1][i].parse().unwrap();
    });

    println!(
        "Grand Total: {}",
        problems.iter().map(|p| p.solve()).sum::<u64>()
    );
}
