use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("Part 1: {}", part1(&buffer));
    println!("Part 2: {}", part2(&buffer));
    Ok(())
}

fn is_safe(levels: &[u32]) -> bool {
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    // <= 3 and >= 1
    let all_increasing = levels
        .windows(2)
        .all(|w| w[1] - w[0] <= 3 && w[1] - w[0] >= 1);
    let all_decreasing = levels
        .windows(2)
        .all(|w| w[0] - w[1] <= 3 && w[0] - w[1] >= 1);
    all_increasing || all_decreasing
}

fn part1(buf: &str) -> usize {
    let reports: Vec<bool> = buf
        .lines()
        .map(|l| {
            let v = l
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            is_safe(&v)
        })
        .collect();
    reports.iter().filter(|&&b| b).count()
}

fn part2(buf: &str) -> usize {
    struct Report {
        levels: Vec<u32>,
    }

    impl Report {
        fn new(line: &str) -> Self {
            Self {
                levels: line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            }
        }

        fn is_salvageable(&self) -> bool {
            if is_safe(&self.levels) {
                return true;
            }

            (0..self.levels.len()).any(|skip_idx| {
                let modified = self
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip_idx)
                    .map(|(_, &v)| v);
                let check: Vec<_> = modified.collect();
                is_safe(&check)
            })
        }
    }

    buf.lines()
        .map(Report::new)
        .filter(|report| report.is_salvageable())
        .count()
}
