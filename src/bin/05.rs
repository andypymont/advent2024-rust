use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

advent_of_code::solution!(5);

type Updates = Vec<Vec<u8>>;

#[derive(Debug, PartialEq)]
struct Rules {
    rules: BTreeMap<u8, BTreeSet<u8>>,
}

impl Rules {
    const fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
        }
    }

    fn correct_order(&self, update: &[u8]) -> bool {
        let mut invalid: BTreeSet<u8> = BTreeSet::new();

        for page in update {
            if invalid.contains(page) {
                return false;
            }

            if let Some(newly_invalid) = self.rules.get(page) {
                invalid.extend(newly_invalid)
            }
        }

        true
    }
    
    fn get(&self, page: u8) -> Option<&BTreeSet<u8>> {
        self.rules.get(&page)
    }

    fn insert(&mut self, page: u8, not_before: u8) {
        self.rules.entry(page)
            .and_modify(|entry| { entry.insert(not_before); })
            .or_insert_with(|| {
                let mut entry = BTreeSet::new();
                entry.insert(not_before);
                entry
            });
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

            rules.insert(second, first);
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
            Ok(Self {
                rules,
                updates,
            })
        } else {
            Err(ParsePuzzleInputError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    input.parse::<PuzzleInput>().map_or(None, |input| {
        Some(input.updates.iter()
                    .map(|update| if input.rules.correct_order(update) {
                        u32::from(update[update.len()/2])
                    } else {
                        0
                    })
                    .sum())
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_rules_node(contents: &[u8]) -> BTreeSet<u8> {
        let mut node = BTreeSet::new();
        for item in contents {
            node.insert(*item);
        }
        node
    }

    fn example_puzzle_input() -> PuzzleInput {
        let mut rules = BTreeMap::new();
        rules.insert(13, example_rules_node(&[97, 61, 29, 47, 75, 53]));
        rules.insert(29, example_rules_node(&[75, 97, 53, 61, 47]));
        rules.insert(47, example_rules_node(&[97, 75]));
        rules.insert(53, example_rules_node(&[47, 75, 61, 97]));
        rules.insert(61, example_rules_node(&[97, 47, 75]));
        rules.insert(75, example_rules_node(&[97]));
        let rules = Rules { rules };

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        PuzzleInput {
            rules,
            updates,
        }
    }

    #[test]
    fn test_correct_order() {
        let input = example_puzzle_input();
        let rules = input.rules;

        assert_eq!(rules.correct_order(&[75, 47, 61, 53, 29]), true);
        assert_eq!(rules.correct_order(&[97, 61, 53, 29, 13]), true);
        assert_eq!(rules.correct_order(&[75, 29, 13]), true);
        assert_eq!(rules.correct_order(&[75, 97, 47, 61, 53]), false);
        assert_eq!(rules.correct_order(&[61, 13, 29]), false);
        assert_eq!(rules.correct_order(&[97, 13, 75, 29, 47]), false);
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
