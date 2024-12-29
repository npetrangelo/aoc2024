use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Lists = (Vec<i32>, Vec<i32>);

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Lists {
    input.lines().fold((vec![], vec![]), |mut lists, line| {
        let mut pair = line.split_whitespace();
        lists.0.push(pair.next().unwrap().parse().unwrap());
        lists.1.push(pair.next().unwrap().parse().unwrap());
        lists
    })
}

#[aoc(day1, part1)]
fn solve_part1(input: &Lists) -> u32 {
    let mut lists = input.clone();
    lists.0.sort();
    lists.1.sort();
    lists.0.iter().zip(lists.1.iter()).fold(0, |sum, pair| {
        sum + (pair.0 - pair.1).abs() as u32
    })
}

#[aoc(day1, part2)]
fn solve_part2(input: &Lists) -> u32 {
    let mut hist1: BTreeMap<u32, u32> = BTreeMap::new();
    for item in input.0.iter() {
        let index = *item as u32;
        if let Some(entry) = hist1.get(&index) {
            hist1.insert(index, entry + 1);
        } else {
            hist1.insert(index, 1);
        }
    }

    let mut hist2: BTreeMap<u32, u32> = BTreeMap::new();
    for item in input.1.iter() {
        let index = *item as u32;
        if let Some(entry) = hist2.get(&index) {
            hist2.insert(index, entry + 1);
        } else {
            hist2.insert(index, 1);
        }
    }

    // Sum of #occurences in hist1 * #occurences in hist2 * number
    let mut sum = 0;
    for key in hist1.keys() {
        sum += key * hist1.get(&key).unwrap_or(&0) * hist2.get(&key).unwrap_or(&0);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(input_generator("2 4\n5  7"), (vec![2, 5], vec![4, 7]));
    }

    #[test]
    fn example1() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        assert_eq!(solve_part1(&input_generator(input)), 11);
    }

    #[test]
    fn example2() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        assert_eq!(solve_part2(&input_generator(input)), 31);
    }
}
