use std::str::FromStr;

advent_of_code::solution!(7);

const fn concat(mut first: u64, second: u64) -> u64 {
    let mut digits = second;
    loop {
        first *= 10;
        digits /= 10;
        if digits == 0 {
            break;
        }
    }

    first + second
}

#[derive(Debug, PartialEq)]
struct CalibrationValue {
    target: u64,
    values: Vec<u64>,
}

impl CalibrationValue {
    fn combinations(&self, operators: u64) -> impl Iterator<Item = u64> + use<'_> {
        let len: u32 = self.values.len().try_into().unwrap_or(0);
        let max = operators.pow(len.saturating_sub(1));
        (0..max).map(move |mut combo| {
            let mut total = *self.values.first().unwrap_or(&0);

            for value in &self.values[1..] {
                let op = combo % operators;
                combo /= operators;
                total = match op {
                    2 => concat(total, *value),
                    1 => total + value,
                    _ => total * value,
                }
            }

            total
        })
    }

    fn is_possible(&self, operators: u64) -> bool {
        self.combinations(operators).any(|c| c == self.target)
    }
}

#[derive(Debug, PartialEq)]
struct ParseCalibrationValueError;

impl FromStr for CalibrationValue {
    type Err = ParseCalibrationValueError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((target_str, values_str)) = line.split_once(": ") else {
            return Err(ParseCalibrationValueError);
        };

        let target = target_str.parse().map_err(|_| ParseCalibrationValueError)?;

        let mut values = Vec::new();
        for value in values_str.split_whitespace() {
            let value = value.parse().map_err(|_| ParseCalibrationValueError)?;
            values.push(value);
        }

        Ok(Self { target, values })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                CalibrationValue::from_str(line).map_or(None, |cv| {
                    Some(if cv.is_possible(2) { cv.target } else { 0 })
                })
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                CalibrationValue::from_str(line).map_or(None, |cv| {
                    Some(if cv.is_possible(3) { cv.target } else { 0 })
                })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_calibration_values() -> Vec<CalibrationValue> {
        vec![
            CalibrationValue {
                target: 190,
                values: vec![10, 19],
            },
            CalibrationValue {
                target: 3267,
                values: vec![81, 40, 27],
            },
            CalibrationValue {
                target: 83,
                values: vec![17, 5],
            },
            CalibrationValue {
                target: 156,
                values: vec![15, 6],
            },
            CalibrationValue {
                target: 7290,
                values: vec![6, 8, 6, 15],
            },
            CalibrationValue {
                target: 161011,
                values: vec![16, 10, 13],
            },
            CalibrationValue {
                target: 192,
                values: vec![17, 8, 14],
            },
            CalibrationValue {
                target: 21037,
                values: vec![9, 7, 18, 13],
            },
            CalibrationValue {
                target: 292,
                values: vec![11, 6, 16, 20],
            },
        ]
    }

    #[test]
    fn test_combinations() {
        let values = example_calibration_values();
        assert_eq!(
            values[0].combinations(2).collect::<Vec<u64>>(),
            vec![190, 29]
        );
        assert_eq!(
            values[1].combinations(2).collect::<Vec<u64>>(),
            vec![87480, 3267, 3267, 148]
        );
    }

    #[test]
    fn test_is_possible() {
        let values = example_calibration_values();
        assert_eq!(values[0].is_possible(2), true);
        assert_eq!(values[1].is_possible(2), true);
        assert_eq!(values[2].is_possible(2), false);
        assert_eq!(values[3].is_possible(2), false);
        assert_eq!(values[4].is_possible(2), false);
        assert_eq!(values[5].is_possible(2), false);
        assert_eq!(values[6].is_possible(2), false);
        assert_eq!(values[7].is_possible(2), false);
        assert_eq!(values[8].is_possible(2), true);
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut values = input.lines().map(|line| CalibrationValue::from_str(line));
        let mut exp_values = example_calibration_values().into_iter().map(|cv| Ok(cv));

        for _ in 0..9 {
            assert_eq!(values.next(), exp_values.next());
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 0), 10);
        assert_eq!(concat(12, 13), 1213);
        assert_eq!(concat(271, 1), 2711);
    }

    #[test]
    fn test_combinations_with_concat() {
        let values = example_calibration_values();
        assert_eq!(
            values[0].combinations(3).collect::<Vec<u64>>(),
            vec![190, 29, 1019],
        );
        assert_eq!(
            values[1].combinations(3).collect::<Vec<u64>>(),
            vec![87480, 3267, 219780, 3267, 148, 8167, 324027, 12127, 814027],
        );
    }

    #[test]
    fn test_is_possible_with_concat() {
        let values = example_calibration_values();
        assert_eq!(values[0].is_possible(3), true);
        assert_eq!(values[1].is_possible(3), true);
        assert_eq!(values[2].is_possible(3), false);
        assert_eq!(values[3].is_possible(3), true);
        assert_eq!(values[4].is_possible(3), true);
        assert_eq!(values[5].is_possible(3), false);
        assert_eq!(values[6].is_possible(3), true);
        assert_eq!(values[7].is_possible(3), false);
        assert_eq!(values[8].is_possible(3), true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11_387));
    }
}
