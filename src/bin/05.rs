use std::str::FromStr;

advent_of_code::solution!(5);

type Updates = Vec<Vec<usize>>;

const MAX_PAGE: usize = 100;

#[derive(Debug, PartialEq)]
struct Rules {
    rules: [bool; MAX_PAGE * MAX_PAGE],
}

impl Rules {
    const fn new() -> Self {
        Self {
            rules: [false; MAX_PAGE * MAX_PAGE],
        }
    }

    fn insert(&mut self, before: usize, after: usize) {
        self.rules[(after * MAX_PAGE) + before] = true;
    }

    const fn contains(&self, before: usize, after: usize) -> bool {
        self.rules[(after * MAX_PAGE) + before]
    }

    fn invalid_after(&self, page: usize) -> &[bool] {
        let begin = page * MAX_PAGE;
        let end = begin + MAX_PAGE;
        &self.rules[begin..end]
    }

    fn in_correct_order(&self, update: &[usize]) -> bool {
        let mut invalid = [false; MAX_PAGE];

        for page in update {
            if invalid[*page] {
                return false;
            }

            for ix in self
                .invalid_after(*page)
                .iter()
                .enumerate()
                .filter_map(|(ix, other)| if *other { Some(ix) } else { None })
            {
                invalid[ix] = true;
            }
        }

        true
    }

    fn corrected_order(&self, update: &[usize]) -> Option<Vec<usize>> {
        let mut output = Vec::new();
        let mut reordered = false;

        for page in update {
            let mut inserted = false;

            for (ix, other) in output.iter().enumerate() {
                if self.contains(*page, *other) {
                    output.insert(ix, *page);
                    inserted = true;
                    break;
                }
            }

            if inserted {
                reordered = true;
            } else {
                output.push(*page);
            }
        }

        if reordered {
            Some(output)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct PuzzleInput {
    rules: Rules,
    updates: Updates,
}

#[derive(Debug, PartialEq)]
struct ParsePuzzleInputError;

fn parse_updates(input: &str) -> Result<Updates, ParsePuzzleInputError> {
    let mut updates = Vec::new();

    for line in input.lines() {
        let mut update = Vec::new();

        for element in line.split(',') {
            let element = element.parse().map_err(|_| ParsePuzzleInputError)?;
            update.push(element);
        }

        updates.push(update);
    }

    Ok(updates)
}

impl FromStr for Rules {
    type Err = ParsePuzzleInputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rules = Self::new();

        for line in input.lines() {
            let Some((first, second)) = line.split_once('|') else {
                return Err(ParsePuzzleInputError);
            };

            let first = first.parse().map_err(|_| ParsePuzzleInputError)?;
            let second = second.parse().map_err(|_| ParsePuzzleInputError)?;

            rules.insert(first, second);
        }

        Ok(rules)
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = input.split_once("\n\n") {
            let rules = first.parse()?;
            let updates = parse_updates(second)?;
            Ok(Self { rules, updates })
        } else {
            Err(ParsePuzzleInputError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    PuzzleInput::from_str(input).map_or(None, |input| {
        Some(
            input
                .updates
                .iter()
                .map(|update| {
                    if input.rules.in_correct_order(update) {
                        update[update.len() / 2]
                    } else {
                        0
                    }
                })
                .sum(),
        )
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    PuzzleInput::from_str(input).map_or(None, |input| {
        Some(
            input
                .updates
                .iter()
                .filter_map(|update| input.rules.corrected_order(update))
                .map(|update| update[update.len() / 2])
                .sum(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(before: usize, after: usize) -> usize {
        (after * MAX_PAGE) + before
    }

    fn example_puzzle_input() -> PuzzleInput {
        let mut rules = [false; MAX_PAGE * MAX_PAGE];
        rules[index(47, 53)] = true;
        rules[index(97, 13)] = true;
        rules[index(97, 61)] = true;
        rules[index(97, 47)] = true;
        rules[index(75, 29)] = true;
        rules[index(61, 13)] = true;
        rules[index(75, 53)] = true;
        rules[index(29, 13)] = true;
        rules[index(97, 29)] = true;
        rules[index(53, 29)] = true;
        rules[index(61, 53)] = true;
        rules[index(97, 53)] = true;
        rules[index(61, 29)] = true;
        rules[index(47, 13)] = true;
        rules[index(75, 47)] = true;
        rules[index(97, 75)] = true;
        rules[index(47, 61)] = true;
        rules[index(75, 61)] = true;
        rules[index(47, 29)] = true;
        rules[index(75, 13)] = true;
        rules[index(53, 13)] = true;
        let rules = Rules { rules };

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        PuzzleInput { rules, updates }
    }

    #[test]
    fn test_invalid_after() {
        let rules = example_puzzle_input().rules;

        let after = rules.invalid_after(53);
        assert_eq!(after[10], false);
        assert_eq!(after[29], false);
        assert_eq!(after[47], true);
        assert_eq!(after[61], true);
        assert_eq!(after[75], true);
        assert_eq!(after[97], true);
        assert_eq!(after[98], false);
    }

    #[test]
    fn test_in_correct_order() {
        let rules = example_puzzle_input().rules;

        assert_eq!(rules.in_correct_order(&[75, 47, 61, 53, 29]), true);
        assert_eq!(rules.in_correct_order(&[97, 61, 53, 29, 13]), true);
        assert_eq!(rules.in_correct_order(&[75, 29, 13]), true);
        assert_eq!(rules.in_correct_order(&[75, 97, 47, 61, 53]), false);
        assert_eq!(rules.in_correct_order(&[61, 13, 29]), false);
        assert_eq!(rules.in_correct_order(&[97, 13, 75, 29, 47]), false);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_puzzle_input()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_corrected_order() {
        let input = example_puzzle_input();
        let rules = input.rules;

        assert_eq!(rules.corrected_order(&[75, 47, 61, 53, 29]), None);
        assert_eq!(
            rules.corrected_order(&[75, 97, 47, 61, 53]),
            Some(vec![97, 75, 47, 61, 53])
        );
        assert_eq!(rules.corrected_order(&[61, 13, 29]), Some(vec![61, 29, 13]));
        assert_eq!(
            rules.corrected_order(&[97, 13, 75, 29, 47]),
            Some(vec![97, 75, 47, 29, 13])
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
