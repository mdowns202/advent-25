use std::fs;

#[derive(Debug)]
struct IdRange(u64, u64);

impl IdRange {
    fn find_invalid_ids(&self, n_splits: usize) -> Vec<u64> {
        let mut invalid_ids = Vec::new();

        for i in self.0..=self.1 {
            let id_str = i.to_string();
            let splits: (&str, &str) = id_str.split_at(id_str.len() / n_splits);

            if id_str.len() % n_splits != 0 {
                continue;
            }

            if splits.0 == splits.1 {
                invalid_ids.push(i);
            }
        }
        invalid_ids
    }
}

pub fn sum_invalid_ids() {
    let puzzle = fs::read_to_string("day02/src/puzzle.txt").unwrap();

    let id_ranges: Vec<IdRange> = puzzle
        .split(",")
        .map(|range| {
            let mut range_split = range.split("-");
            let (start, end) = range_split
                .next()
                .zip(range_split.next())
                .map(|(s, e)| (s.trim(), e.trim()))
                .unwrap();
            IdRange(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let invalid_sum = id_ranges.iter().fold(0, |sum, id_range| {
        sum + id_range.find_invalid_ids(2).iter().sum::<u64>()
    });

    println!("Sum: {}", invalid_sum);
}
