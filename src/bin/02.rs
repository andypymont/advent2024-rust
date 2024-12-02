use std::cmp::Ordering;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct LevelReportLine(Vec<u8>);

impl LevelReportLine {
    fn changes(&self) -> impl Iterator<Item = (Ordering, u8)> + '_ {
        self.0.windows(2).map(|window| {
            let Some(first) = window.first() else {
                return (Ordering::Equal, 0);
            };
            let Some(second) = window.get(1) else {
                return (Ordering::Equal, 0);
            };
            (second.cmp(first), first.abs_diff(*second))
        })
    }

    fn is_safe(&self) -> bool {
        let mut changes = self.changes();
        let Some((direction, mut max)) = changes.next() else {
            return false;
        };
        for (cmp, diff) in changes {
            if cmp == Ordering::Equal {
                return false;
            }
            if cmp != direction {
                return false;
            }
            max = max.max(diff);
            if max > 3 {
                return false;
            }
        }

        true
    }

    fn is_safe_tolerating(&self) -> bool {
        (0..self.0.len()).any(|ix| {
            let except = Self(
                self.0
                    .iter()
                    .enumerate()
                    .filter_map(|(pos, item)| if pos == ix { None } else { Some(*item) })
                    .collect(),
            );
            except.is_safe()
        })
    }
}

#[derive(Debug, PartialEq)]
struct LevelReport {
    lines: Vec<LevelReportLine>,
}

#[derive(Debug, PartialEq)]
struct ParseLevelReportError;

impl FromStr for LevelReportLine {
    type Err = ParseLevelReportError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();

        for item_str in line.split_whitespace() {
            let item = item_str.parse().map_err(|_| ParseLevelReportError)?;
            items.push(item);
        }

        Ok(Self(items))
    }
}

impl FromStr for LevelReport {
    type Err = ParseLevelReportError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = Vec::new();

        for line in text.lines() {
            let report_line = line.parse()?;
            lines.push(report_line);
        }

        Ok(Self { lines })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let Ok(report) = input.parse::<LevelReport>() else {
        return None;
    };
    Some(report.lines.iter().filter(|line| line.is_safe()).count())
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let Ok(report) = input.parse::<LevelReport>() else {
        return None;
    };
    Some(
        report
            .lines
            .iter()
            .filter(|line| line.is_safe_tolerating())
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_report() -> LevelReport {
        LevelReport {
            lines: vec![
                LevelReportLine(vec![7, 6, 4, 2, 1]),
                LevelReportLine(vec![1, 2, 7, 8, 9]),
                LevelReportLine(vec![9, 7, 6, 2, 1]),
                LevelReportLine(vec![1, 3, 2, 4, 5]),
                LevelReportLine(vec![8, 6, 4, 4, 1]),
                LevelReportLine(vec![1, 3, 6, 7, 9]),
            ],
        }
    }

    #[test]
    fn test_report_changes() {
        let report = example_report();

        let mut changes = report.lines[0].changes();
        assert_eq!(changes.next(), Some((Ordering::Less, 1)));
        assert_eq!(changes.next(), Some((Ordering::Less, 2)));
        assert_eq!(changes.next(), Some((Ordering::Less, 2)));
        assert_eq!(changes.next(), Some((Ordering::Less, 1)));
        assert_eq!(changes.next(), None);

        changes = report.lines[1].changes();
        assert_eq!(changes.next(), Some((Ordering::Greater, 1)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 5)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 1)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 1)));
        assert_eq!(changes.next(), None);

        changes = report.lines[2].changes();
        assert_eq!(changes.next(), Some((Ordering::Less, 2)));
        assert_eq!(changes.next(), Some((Ordering::Less, 1)));
        assert_eq!(changes.next(), Some((Ordering::Less, 4)));
        assert_eq!(changes.next(), Some((Ordering::Less, 1)));
        assert_eq!(changes.next(), None);

        changes = report.lines[3].changes();
        assert_eq!(changes.next(), Some((Ordering::Greater, 2)));
        assert_eq!(changes.next(), Some((Ordering::Less, 1)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 2)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 1)));
        assert_eq!(changes.next(), None);

        changes = report.lines[4].changes();
        assert_eq!(changes.next(), Some((Ordering::Less, 2)));
        assert_eq!(changes.next(), Some((Ordering::Less, 2)));
        assert_eq!(changes.next(), Some((Ordering::Equal, 0)));
        assert_eq!(changes.next(), Some((Ordering::Less, 3)));
        assert_eq!(changes.next(), None);

        changes = report.lines[5].changes();
        assert_eq!(changes.next(), Some((Ordering::Greater, 2)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 3)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 1)));
        assert_eq!(changes.next(), Some((Ordering::Greater, 2)));
        assert_eq!(changes.next(), None);
    }

    #[test]
    fn test_is_safe() {
        let report = example_report();
        assert_eq!(report.lines[0].is_safe(), true);
        assert_eq!(report.lines[1].is_safe(), false);
        assert_eq!(report.lines[2].is_safe(), false);
        assert_eq!(report.lines[3].is_safe(), false);
        assert_eq!(report.lines[4].is_safe(), false);
        assert_eq!(report.lines[5].is_safe(), true);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_report()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_is_safe_tolerating() {
        let report = example_report();
        assert_eq!(report.lines[0].is_safe_tolerating(), true);
        assert_eq!(report.lines[1].is_safe_tolerating(), false);
        assert_eq!(report.lines[2].is_safe_tolerating(), false);
        assert_eq!(report.lines[3].is_safe_tolerating(), true);
        assert_eq!(report.lines[4].is_safe_tolerating(), true);
        assert_eq!(report.lines[5].is_safe_tolerating(), true);
    }

    #[test]
    fn test_is_safe_tolerating_becomes_flat() {
        let becomes_flat = LevelReportLine(vec![2, 3, 2, 2]);
        assert_eq!(becomes_flat.is_safe_tolerating(), false);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
