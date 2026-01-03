pub fn find_accessible_rolls() {
    let rows: Vec<Vec<char>> = include_str!("puzzle.txt")
        .lines()
        .map(|r| r.chars().collect())
        .collect();

    let mut accessible = 0;

    rows.iter().enumerate().for_each(|(i, r)| {
        r.iter().enumerate().for_each(|(j, c)| match c == &'@' {
            true => {
                let mut adjacent = 0;

                if i > 0 && j > 0 && rows[i - 1][j - 1] == '@' {
                    adjacent += 1;
                }
                if i > 0 && rows[i - 1][j] == '@' {
                    adjacent += 1;
                }
                if i > 0 && j < rows[i].len() - 1 && rows[i - 1][j + 1] == '@' {
                    adjacent += 1;
                }
                if j > 0 && rows[i][j - 1] == '@' {
                    adjacent += 1;
                }
                if j < rows[i].len() - 1 && rows[i][j + 1] == '@' {
                    adjacent += 1;
                }
                if i < rows.len() - 1 && j > 0 && rows[i + 1][j - 1] == '@' {
                    adjacent += 1;
                }
                if i < rows.len() - 1 && rows[i + 1][j] == '@' {
                    adjacent += 1;
                }
                if i < rows.len() - 1 && j < rows[i].len() - 1 && rows[i + 1][j + 1] == '@' {
                    adjacent += 1;
                }

                if adjacent < 4 {
                    accessible += 1;
                }
            }
            false => (),
        })
    });
    println!("Accessible toilet paper rolls: {}", accessible);
}
