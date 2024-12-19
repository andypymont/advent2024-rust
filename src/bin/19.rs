use std::str::FromStr;

advent_of_code::solution!(19);

#[derive(Debug, PartialEq)]
struct Onsen {
    towels: Vec<String>,
    patterns: Vec<String>,
}

impl Onsen {
    fn is_pattern_possible(&self, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        self.towels.iter().any(|towel| {
            if towel.len() > pattern.len() {
                false
            } else {
                &pattern[..towel.len()] == towel
                    && self.is_pattern_possible(&pattern[towel.len()..])
            }
        })
    }

    fn possible_patterns(&self) -> usize {
        self.patterns
            .iter()
            .filter(|pattern| self.is_pattern_possible(pattern))
            .count()
    }
}

#[derive(Debug, PartialEq)]
struct ParseOnsenError;

impl FromStr for Onsen {
    type Err = ParseOnsenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (towels_str, patterns_str) = input.split_once("\n\n").ok_or(ParseOnsenError)?;
        let mut towels = Vec::new();
        for towel in towels_str.trim().split(", ") {
            towels.push(towel.to_string());
        }

        let mut patterns = Vec::new();
        for pattern in patterns_str.lines() {
            patterns.push(pattern.to_string());
        }

        Ok(Self { towels, patterns })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Onsen::from_str(input).map_or(None, |onsen| Some(onsen.possible_patterns()))
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_onsen() -> Onsen {
        Onsen {
            towels: vec![
                "r".to_string(),
                "wr".to_string(),
                "b".to_string(),
                "g".to_string(),
                "bwu".to_string(),
                "rb".to_string(),
                "gb".to_string(),
                "br".to_string(),
            ],
            patterns: vec![
                "brwrr".to_string(),
                "bggr".to_string(),
                "gbbr".to_string(),
                "rrbgbr".to_string(),
                "ubwu".to_string(),
                "bwurrg".to_string(),
                "brgr".to_string(),
                "bbrgwb".to_string(),
            ],
        }
    }

    #[test]
    fn test_parse_onsen() {
        assert_eq!(
            Onsen::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_onsen()),
        );
    }

    #[test]
    fn test_is_pattern_possible() {
        let onsen = example_onsen();

        assert_eq!(onsen.is_pattern_possible("brwrr"), true);
        assert_eq!(onsen.is_pattern_possible("bggr"), true);
        assert_eq!(onsen.is_pattern_possible("gbbr"), true);
        assert_eq!(onsen.is_pattern_possible("rrbgbr"), true);
        assert_eq!(onsen.is_pattern_possible("ubwu"), false);
        assert_eq!(onsen.is_pattern_possible("bwurrg"), true);
        assert_eq!(onsen.is_pattern_possible("brgr"), true);
        assert_eq!(onsen.is_pattern_possible("bbrgwb"), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
