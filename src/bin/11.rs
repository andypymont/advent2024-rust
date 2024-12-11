use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(11);

fn number_to_digits(number: u64) -> Vec<u64> {
    let mut remain = number;
    let mut digits = Vec::new();

    while remain > 0 {
        let digit = remain % 10;
        remain /= 10;
        digits.push(digit);
    }

    digits
}

fn digits_to_number(digits: &[u64]) -> u64 {
    let mut number = 0;
    let mut mul = 1;

    for digit in digits {
        number += digit * mul;
        mul *= 10;
    }

    number
}

fn split_digits_evenly(number: u64) -> Option<(u64, u64)> {
    let digits = number_to_digits(number);
    if digits.len() % 2 == 1 {
        return None;
    }

    let half = digits.len() / 2;
    let small = digits_to_number(&digits[..half]);
    let large = digits_to_number(&digits[half..]);
    Some((large, small))
}

fn next_stones(stone: u64) -> (Option<u64>, Option<u64>) {
    if stone == 0 {
        return (Some(1), None);
    }

    split_digits_evenly(stone)
        .map_or_else(|| (Some(2024 * stone), None), |(a, b)| (Some(a), Some(b)))
}

#[derive(Debug, PartialEq)]
struct StoneLine(BTreeMap<u64, u64>);

impl StoneLine {
    const fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add(&mut self, stone: u64, quantity: u64) {
        self.0
            .entry(stone)
            .and_modify(|count| *count += quantity)
            .or_insert(quantity);
    }

    fn blink(&self) -> Self {
        let mut after = Self::new();

        for (stone, quantity) in &self.0 {
            let (first, second) = next_stones(*stone);

            if let Some(first) = first {
                after.add(first, *quantity);
            }

            if let Some(second) = second {
                after.add(second, *quantity);
            }
        }

        after
    }

    fn len(&self) -> u64 {
        self.0.values().sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseStoneLineError;

impl FromStr for StoneLine {
    type Err = ParseStoneLineError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stones = Self::new();

        for stone in input.split_whitespace() {
            let stone = stone.parse().map_err(|_| ParseStoneLineError)?;
            stones.add(stone, 1);
        }

        Ok(stones)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    StoneLine::from_str(input).map_or(None, |mut stones| {
        for _ in 0..25 {
            stones = stones.blink();
        }
        Some(stones.len())
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    StoneLine::from_str(input).map_or(None, |mut stones| {
        for _ in 0..75 {
            stones = stones.blink();
        }
        Some(stones.len())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stone_line_from_vec(v: Vec<u64>) -> StoneLine {
        let mut line = StoneLine::new();
        for stone in v {
            line.0.entry(stone).and_modify(|x| *x += 1).or_insert(1);
        }
        line
    }

    #[test]
    fn test_stone_line_from_str() {
        assert_eq!(
            StoneLine::from_str("0 1 10 99 999"),
            Ok(stone_line_from_vec(vec![0, 1, 10, 99, 999])),
        );
        assert_eq!(
            StoneLine::from_str("512 72 2024 2 0 2 4 2867 6032"),
            Ok(stone_line_from_vec(vec![
                512, 72, 2024, 2, 0, 2, 4, 2867, 6032
            ])),
        );
        assert_eq!(
            StoneLine::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(stone_line_from_vec(vec![125, 17])),
        );
    }

    #[test]
    fn test_number_to_digits() {
        assert_eq!(number_to_digits(5), vec![5]);
        assert_eq!(number_to_digits(14), vec![4, 1]);
        assert_eq!(number_to_digits(207), vec![7, 0, 2]);
        assert_eq!(number_to_digits(1517), vec![7, 1, 5, 1]);
        assert_eq!(number_to_digits(207_864), vec![4, 6, 8, 7, 0, 2]);
    }

    #[test]
    fn test_digits_to_number() {
        assert_eq!(digits_to_number(&vec![5]), 5);
        assert_eq!(digits_to_number(&vec![2, 7]), 72);
        assert_eq!(digits_to_number(&vec![7, 4, 1]), 147);
        assert_eq!(digits_to_number(&vec![2, 2, 5, 1, 8]), 81522);
    }

    #[test]
    fn test_split_digits_evenly() {
        assert_eq!(split_digits_evenly(1000), Some((10, 0)));
        assert_eq!(split_digits_evenly(10), Some((1, 0)));
        assert_eq!(split_digits_evenly(3467), Some((34, 67)));
        assert_eq!(split_digits_evenly(4), None);
        assert_eq!(split_digits_evenly(157), None);
    }

    #[test]
    fn test_next_stones_zero() {
        assert_eq!(next_stones(0), (Some(1), None));
    }

    #[test]
    fn test_next_stones_split() {
        assert_eq!(next_stones(14), (Some(1), Some(4)));
        assert_eq!(next_stones(2185), (Some(21), Some(85)));
        assert_eq!(next_stones(147_816), (Some(147), Some(816)));
    }

    #[test]
    fn test_next_stones_replace() {
        assert_eq!(next_stones(1), (Some(2024), None));
        assert_eq!(next_stones(2), (Some(4048), None));
        assert_eq!(next_stones(100), (Some(202_400), None));
    }

    #[test]
    fn test_stone_line_blink() {
        let line = stone_line_from_vec(vec![0, 1, 10, 99, 999]);
        assert_eq!(
            line.blink(),
            stone_line_from_vec(vec![1, 2024, 1, 0, 9, 9, 2021976])
        );
    }

    #[test]
    fn test_stone_line_blink_extended() {
        let stages = vec![
            stone_line_from_vec(vec![253000, 1, 7]),
            stone_line_from_vec(vec![253, 0, 2024, 14168]),
            stone_line_from_vec(vec![512072, 1, 20, 24, 28676032]),
            stone_line_from_vec(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
            stone_line_from_vec(vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32,
            ]),
            stone_line_from_vec(vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ]),
        ];
        let mut line = stone_line_from_vec(vec![125, 17]);
        for expected in stages {
            line = line.blink();
            assert_eq!(line, expected);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55_312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65_601_038_650_482));
    }
}
