use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(11);

fn number_to_digits(number: u64) -> Vec<u64> {
    let mut remain = number;
    let mut digits = Vec::new();

    while remain > 0 {
        let digit = remain % 10;
        remain = remain / 10;
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

#[derive(Debug, PartialEq)]
struct StoneLine(VecDeque<u64>);

impl StoneLine {
    fn blink(&mut self) {
        for _ in 0..self.0.len() {
            self.blink_stone();
        }
    }

    fn blink_stone(&mut self) {
        let Some(stone) = self.0.pop_front() else {
            return;
        };

        if stone == 0 {
            self.0.push_back(1);
            return;
        }

        if let Some((first, second)) = Self::split_digits(stone) {
            self.0.push_back(first);
            self.0.push_back(second);
            return;
        }

        self.0.push_back(stone * 2024);
    }

    fn split_digits(stone: u64) -> Option<(u64, u64)> {
        let digits = number_to_digits(stone);

        if digits.len() % 2 == 0 {
            let half = digits.len() / 2;
            let small = digits_to_number(&digits[..half]);
            let large = digits_to_number(&digits[half..]);
            Some((large, small))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseStoneLineError;

impl FromStr for StoneLine {
    type Err = ParseStoneLineError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stones = VecDeque::new();

        for stone in input.split_whitespace() {
            let stone = stone.parse().map_err(|_| ParseStoneLineError)?;
            stones.push_back(stone);
        }

        Ok(Self(stones))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    StoneLine::from_str(input).map_or(None, |mut stones| {
        for _ in 0..25 {
            stones.blink();
        }
        Some(stones.0.len())
    })
}

#[must_use]
pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stone_line_from_vec(v: Vec<u64>) -> StoneLine {
        StoneLine(VecDeque::from(v))
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
    fn test_split_digits() {
        assert_eq!(StoneLine::split_digits(1000), Some((10, 0)));
        assert_eq!(StoneLine::split_digits(10), Some((1, 0)));
        assert_eq!(StoneLine::split_digits(3467), Some((34, 67)));
        assert_eq!(StoneLine::split_digits(4), None);
        assert_eq!(StoneLine::split_digits(157), None);
    }

    #[test]
    fn test_stone_line_blink() {
        let mut line = stone_line_from_vec(vec![0, 1, 10, 99, 999]);
        line.blink();
        assert_eq!(
            line,
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
            line.blink();
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
        assert_eq!(result, None);
    }
}
