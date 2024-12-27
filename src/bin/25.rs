use std::str::FromStr;

advent_of_code::solution!(25);

type Lock = [u8; 5];

fn key_fits_lock(key: Lock, lock: Lock) -> bool {
    (0..5).all(|c| lock[c] + key[c] <= 7)
}

#[derive(Debug, PartialEq)]
struct Door {
    locks: Vec<Lock>,
    keys: Vec<Lock>,
}

impl Door {
    fn non_overlapping_combos(&self) -> usize {
        self.locks
            .iter()
            .flat_map(|lock| self.keys.iter().filter(|key| key_fits_lock(**key, *lock)))
            .count()
    }
}

#[derive(Debug, PartialEq)]
struct ParseDoorError;

impl FromStr for Door {
    type Err = ParseDoorError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for part in input.split("\n\n") {
            let mut lines = part.lines().peekable();
            let mut heights: Lock = [0; 5];

            let is_key = if let Some(first) = lines.peek() {
                first == &"....."
            } else {
                return Err(ParseDoorError);
            };

            for line in lines {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        heights[col] += 1;
                    }
                }
            }

            if is_key {
                keys.push(heights);
            } else {
                locks.push(heights);
            }
        }

        Ok(Self { locks, keys })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Door::from_str(input)
        .ok()
        .map(|door| door.non_overlapping_combos())
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_door() -> Door {
        Door {
            locks: vec![[1, 6, 4, 5, 4], [2, 3, 1, 6, 4]],
            keys: vec![[6, 1, 3, 2, 4], [5, 4, 5, 1, 3], [4, 1, 3, 1, 2]],
        }
    }

    #[test]
    fn test_parse_door() {
        assert_eq!(
            Door::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_door()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
