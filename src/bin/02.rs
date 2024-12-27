use std::cmp::Ordering;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct LevelReportLine(Vec<u8>);

impl LevelReportLine {
    fn is_safe(&self, skip: Option<usize>) -> bool {
        let mut direction: Option<Ordering> = None;

        for (ix, value) in self.0.iter().enumerate() {
            let offset = if skip == Some(ix) {
                continue;
            } else if skip == Some(ix + 1) {
                2
            } else {
                1
            };
            let Some(next) = self.0.get(ix + offset) else {
                break;
            };

            if value.abs_diff(*next) > 3 {
                return false;
            }

            let cmp = next.cmp(value);
            if cmp == Ordering::Equal {
                return false;
            }
            if let Some(dir) = direction {
                if cmp != dir {
                    return false;
                }
            }
            direction = Some(cmp);
        }

        true
    }

    fn is_safe_default(&self) -> bool {
        self.is_safe(None)
    }

    fn is_safe_tolerating(&self) -> bool {
        (0..self.0.len()).any(|ix| self.is_safe(Some(ix)))
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
    LevelReport::from_str(input).ok().map(|report| {
        report
            .lines
            .iter()
            .filter(|line| line.is_safe_default())
            .count()
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    LevelReport::from_str(input).ok().map(|report| {
        report
            .lines
            .iter()
            .filter(|line| line.is_safe_tolerating())
            .count()
    })
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
    fn test_is_safe_default() {
        let report = example_report();
        assert_eq!(report.lines[0].is_safe_default(), true);
        assert_eq!(report.lines[1].is_safe_default(), false);
        assert_eq!(report.lines[2].is_safe_default(), false);
        assert_eq!(report.lines[3].is_safe_default(), false);
        assert_eq!(report.lines[4].is_safe_default(), false);
        assert_eq!(report.lines[5].is_safe_default(), true);
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
