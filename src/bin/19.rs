use std::collections::BTreeMap;
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

    fn ways_pattern_possible<'a>(
        &self,
        pattern: &'a str,
        cache: &mut BTreeMap<&'a str, usize>,
    ) -> usize {
        if pattern.is_empty() {
            return 1;
        }
        if let Some(value) = cache.get(pattern) {
            return *value;
        }
        let value = self
            .towels
            .iter()
            .map(|towel| {
                if towel.len() > pattern.len() {
                    0
                } else if &pattern[..towel.len()] == towel {
                    self.ways_pattern_possible(&pattern[towel.len()..], cache)
                } else {
                    0
                }
            })
            .sum();
        cache.insert(pattern, value);
        value
    }

    fn total_ways_patterns_possible(&self) -> usize {
        let mut cache = BTreeMap::new();
        self.patterns
            .iter()
            .map(|pattern| self.ways_pattern_possible(pattern, &mut cache))
            .sum()
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

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Onsen::from_str(input).map_or(None, |onsen| Some(onsen.total_ways_patterns_possible()))
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
    fn test_ways_patterns_possible() {
        let onsen = example_onsen();
        let mut cache = BTreeMap::new();

        assert_eq!(onsen.ways_pattern_possible("brwrr", &mut cache), 2);
        assert_eq!(onsen.ways_pattern_possible("bggr", &mut cache), 1);
        assert_eq!(onsen.ways_pattern_possible("gbbr", &mut cache), 4);
        assert_eq!(onsen.ways_pattern_possible("rrbgbr", &mut cache), 6);
        assert_eq!(onsen.ways_pattern_possible("ubwu", &mut cache), 0);
        assert_eq!(onsen.ways_pattern_possible("bwurrg", &mut cache), 1);
        assert_eq!(onsen.ways_pattern_possible("brgr", &mut cache), 2);
        assert_eq!(onsen.ways_pattern_possible("bbrgwb", &mut cache), 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
