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
    let diffRange = report.windows(2).fold((u32::max_value(), 0), |mut range, pair| {
        let diff = pair[1].abs_diff(pair[0]);
        if range.1 < diff {
            range.1 = diff;
        }
        if range.0 > diff {
            range.0 = diff;
        }
        range
    });
    sorted && 1 <= diffRange.0 && 3 >= diffRange.1
}

#[aoc(day2, part1)]
fn part1(reports: &Reports) -> u32 {
    reports.iter().fold(0, |safe, report| {
        safe + if is_safe(&report) { 1 } else { 0 }
    })
}

fn is_safe2(report: &Vec<i32>) -> bool {
    if is_safe(&report) {
        return true;
    }
    for index in 0..report.len() {
        let mut cloned = report.clone();
        cloned.remove(index);
        if is_safe(&cloned) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
fn part2(reports: &Reports) -> u32 {
    reports.iter().fold(0, |safe, report| {
        safe + if is_safe2(&report) { 1 } else { 0 }
    })
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
        assert!(!is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn day1() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";
        assert_eq!(part1(&reports(input)), 2);
    }

    #[test]
    fn test_safe2() {
        assert!(is_safe2(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe2(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe2(&vec![9, 7, 6, 2, 1]));
        assert!(is_safe2(&vec![1, 3, 2, 4, 5]));
        assert!(is_safe2(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe2(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn day2() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";
        assert_eq!(part2(&reports(input)), 4);
    }
}
