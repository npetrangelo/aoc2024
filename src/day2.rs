use aoc_runner_derive::{aoc, aoc_generator};

type Reports = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn reports(input: &str) -> Reports {
    input.lines()
        .map(|line| line.split_whitespace()
            .map(|level| level.parse().expect("Invalid input"))
            .collect())
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let sorted = report.is_sorted() || report.iter().rev().is_sorted();
    let maxDiff = report.windows(2).fold(0, |mut max, pair| {
        let diff = pair[1].abs_diff(pair[0]);
        if max < diff {
            max = diff;
        }
        max
    });
    sorted && (1..=3).contains(&maxDiff)
}

#[aoc(day2, part1)]
fn part1(reports: &Reports) -> u32 {
    let safe = 0;

    safe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";
        let output = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
        ];
        assert_eq!(reports(input), output);
    }

    #[test]
    fn test_safe() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1]));
        assert!(is_safe(&vec![1, 3, 6, 7, 9]));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5]));
    }
}
