use std::cmp::Ordering;
use std::ops::Add;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Slope {
    Increasing,
    Decreasing,
    Undulating,
}

impl Add<Ordering> for Slope {
    type Output = Self;

    fn add(self, rhs: Ordering) -> Self::Output {
        match (self, rhs) {
            (Slope::Increasing, Ordering::Greater) => Slope::Increasing,
            (Slope::Decreasing, Ordering::Less) => Slope::Decreasing,
            _ => Slope::Undulating,
        }
    }
}

#[derive(Debug, PartialEq)]
struct LevelReportSummary {
    slope: Slope,
    max: u8,
}

#[derive(Debug, PartialEq)]
struct LevelReportLine(Vec<u8>);

#[derive(Debug, PartialEq)]
struct LevelReport {
    lines: Vec<LevelReportLine>,
}

impl LevelReportSummary {
    fn is_safe(&self) -> bool {
        !(self.slope == Slope::Undulating || self.max > 3)
    }
}

impl LevelReportLine {
    fn summary(&self) -> LevelReportSummary {
        let mut items = self.0.iter();

        let Some(mut prev) = items.next() else {
            return LevelReportSummary {
                slope: Slope::Undulating,
                max: 0,
            };
        };

        let mut max = 0;
        let mut slope: Option<Slope> = None;

        for item in items {
            let cmp = item.cmp(prev);
            slope = match slope {
                Some(existing_slope) => Some(existing_slope + cmp),
                None => match cmp {
                    Ordering::Greater => Some(Slope::Increasing),
                    Ordering::Equal => Some(Slope::Undulating),
                    Ordering::Less => Some(Slope::Decreasing),
                },
            };
            max = max.max(item.abs_diff(*prev));
            prev = item;
        }

        LevelReportSummary {
            slope: slope.unwrap_or(Slope::Undulating),
            max,
        }
    }
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
    Some(
        report
            .lines
            .iter()
            .filter(|line| line.summary().is_safe())
            .count(),
    )
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_report_summary() {
        let report = example_report();
        assert_eq!(
            report.lines[0].summary(),
            LevelReportSummary {
                slope: Slope::Decreasing,
                max: 2
            },
        );
        assert_eq!(
            report.lines[1].summary(),
            LevelReportSummary {
                slope: Slope::Increasing,
                max: 5
            },
        );
        assert_eq!(
            report.lines[2].summary(),
            LevelReportSummary {
                slope: Slope::Decreasing,
                max: 4
            },
        );
        assert_eq!(
            report.lines[3].summary(),
            LevelReportSummary {
                slope: Slope::Undulating,
                max: 2
            },
        );
        assert_eq!(
            report.lines[4].summary(),
            LevelReportSummary {
                slope: Slope::Undulating,
                max: 3
            },
        );
        assert_eq!(
            report.lines[5].summary(),
            LevelReportSummary {
                slope: Slope::Increasing,
                max: 3
            },
        );
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
