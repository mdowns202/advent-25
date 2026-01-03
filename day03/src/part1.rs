pub fn find_total_joltage() {
    let batteries: Vec<Vec<i32>> = include_str!("puzzle.txt")
        .lines()
        .map(|bat| {
            bat.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut sum = 0;

    batteries.iter().for_each(|bat| {
        let mut first_num = (0, 0);

        bat.iter().enumerate().for_each(|(i, b)| {
            if *b > first_num.1 && i < bat.len() - 1 {
                first_num = (i, *b);
            }
        });

        let second_num = bat
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > first_num.0)
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        let biggest = (first_num.1.to_string() + &second_num.1.to_string())
            .parse::<i32>()
            .unwrap();

        sum += biggest;
    });
    println!("Sum Joltage: {}", sum);
}
