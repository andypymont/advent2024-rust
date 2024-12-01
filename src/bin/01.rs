use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq)]
struct LocationList {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl LocationList {
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

        left.sort_unstable();
        right.sort_unstable();

        Ok(LocationList { left, right })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    match input.parse::<LocationList>() {
        Ok(list) => Some(list.total_distance()),
        Err(_) => None,
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(LocationList {
                left: vec![1, 2, 3, 3, 3, 4,],
                right: vec![3, 3, 3, 4, 5, 9,],
            })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
