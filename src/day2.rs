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
}
