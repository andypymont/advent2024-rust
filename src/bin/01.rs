use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq)]
struct LocationList {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl LocationList {
    fn sort(&mut self) {
        self.left.sort_unstable();
        self.right.sort_unstable();
    }

    fn total_distance(&self) -> u32 {
        self.left
            .iter()
            .enumerate()
            .map(|(ix, l)| match self.right.get(ix) {
                Some(r) => l.abs_diff(*r),
                None => 0,
            })
            .sum()
    }

    fn right_counts(&self) -> BTreeMap<u32, u32> {
        let mut counts = BTreeMap::new();

        for item in &self.right {
            let count = 1 + counts.get(item).unwrap_or(&0);
            counts.insert(*item, count);
        }

        counts
    }

    fn similarity_score(&self) -> u32 {
        let right_counts = self.right_counts();
        self.left
            .iter()
            .map(|l| l * right_counts.get(l).unwrap_or(&0))
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseLocationListError;

impl FromStr for LocationList {
    type Err = ParseLocationListError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in text.lines() {
            let mut l: Result<u32, ParseLocationListError> = Err(ParseLocationListError);
            let mut r: Result<u32, ParseLocationListError> = Err(ParseLocationListError);

            for (ix, part) in line.split_whitespace().enumerate() {
                let value = part.parse::<u32>().map_err(|_| ParseLocationListError)?;
                match ix {
                    0 => l = Ok(value),
                    1 => r = Ok(value),
                    _ => return Err(ParseLocationListError),
                }
            }

            let l = l?;
            let r = r?;

            left.push(l);
            right.push(r);
        }

        Ok(LocationList { left, right })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    match input.parse::<LocationList>() {
        Ok(mut list) => {
            list.sort();
            Some(list.total_distance())
        }
        Err(_) => None,
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    match input.parse::<LocationList>() {
        Ok(list) => Some(list.similarity_score()),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_list() -> LocationList {
        LocationList {
            left: vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_list())
        );
    }

    #[test]
    fn test_right_counts() {
        let mut expected = BTreeMap::new();
        expected.insert(3, 3);
        expected.insert(4, 1);
        expected.insert(5, 1);
        expected.insert(9, 1);

        assert_eq!(example_list().right_counts(), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
