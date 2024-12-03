use std::ops::{Add, AddAssign};

advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParserState {
    Blank,
    FirstOperand(Option<u32>),
    SecondOperand(u32, Option<u32>),
}

impl Add<u32> for ParserState {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Self::Blank => Self::Blank,
            Self::FirstOperand(None) => Self::FirstOperand(Some(rhs)),
            Self::FirstOperand(Some(existing)) => Self::FirstOperand(Some((existing * 10) + rhs)),
            Self::SecondOperand(first, None) => Self::SecondOperand(first, Some(rhs)),
            Self::SecondOperand(first, Some(existing)) => {
                Self::SecondOperand(first, Some((existing * 10) + rhs))
            }
        }
    }
}

impl AddAssign<u32> for ParserState {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

#[derive(Debug, PartialEq)]
enum ParserActivity {
    Ignore,
    Active,
    Inactive,
}

#[derive(Debug, PartialEq)]
struct InputParser {
    active: ParserActivity,
    state: ParserState,
    buffer: [char; 7],
    total: u32,
}

impl InputParser {
    fn new(togglable: bool) -> Self {
        let active = if togglable {
            ParserActivity::Active
        } else {
            ParserActivity::Ignore
        };
        Self {
            active,
            state: ParserState::Blank,
            buffer: [' '; 7],
            total: 0,
        }
    }

    fn activate(&mut self) {
        if self.active != ParserActivity::Ignore {
            self.active = ParserActivity::Active;
        }
    }

    fn deactivate(&mut self) {
        if self.active != ParserActivity::Ignore {
            self.active = ParserActivity::Inactive;
        }
    }

    fn clear(&mut self) {
        self.state = ParserState::Blank;
    }

    fn read_char(&mut self, input: char) {
        self.buffer = [
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
            self.buffer[4],
            self.buffer[5],
            self.buffer[6],
            input,
        ];

        if self.buffer == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            self.deactivate();
        } else if self.buffer[3..7] == ['d', 'o', '(', ')'] {
            self.activate();
        }

        match self.state {
            ParserState::Blank => {
                if self.buffer[3..7] == ['m', 'u', 'l', '('] {
                    self.state = ParserState::FirstOperand(None);
                }
            }
            ParserState::FirstOperand(first) => {
                if let Some(digit) = input.to_digit(10) {
                    self.state += digit;
                } else if input == ',' {
                    if let Some(f) = first {
                        self.state = ParserState::SecondOperand(f, None);
                    } else {
                        self.clear();
                    }
                } else {
                    self.clear();
                }
            }
            ParserState::SecondOperand(first, second) => {
                if let Some(digit) = input.to_digit(10) {
                    self.state += digit;
                } else if input == ')' {
                    let value = match (&self.active, second) {
                        (ParserActivity::Inactive, _) | (_, None) => 0,
                        (_, Some(s)) => first * s,
                    };
                    self.total += value;
                    self.clear();
                } else {
                    self.clear();
                }
            }
        }
    }

    fn read_input(&mut self, input: &str) {
        for ch in input.chars() {
            self.read_char(ch);
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut parser = InputParser::new(false);
    parser.read_input(input);
    Some(parser.total)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut parser = InputParser::new(true);
    parser.read_input(input);
    Some(parser.total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_first_instruction() {
        let mut parser = InputParser::new(false);
        assert_eq!(parser.state, ParserState::Blank);
        assert_eq!(parser.total, 0);

        parser.read_char('m');
        parser.read_char('u');
        parser.read_char('l');
        parser.read_char('(');
        assert_eq!(parser.state, ParserState::FirstOperand(None));
        parser.read_char('2');
        assert_eq!(parser.state, ParserState::FirstOperand(Some(2)));
        parser.read_char(',');
        assert_eq!(parser.state, ParserState::SecondOperand(2, None));
        parser.read_char('4');
        assert_eq!(parser.state, ParserState::SecondOperand(2, Some(4)));
        parser.read_char(')');
        assert_eq!(parser.state, ParserState::Blank);
        assert_eq!(parser.total, 8);
    }

    #[test]
    fn test_read_input() {
        let expected = InputParser {
            active: ParserActivity::Ignore,
            state: ParserState::Blank,
            buffer: ['l', '(', '8', ',', '5', ')', ')'],
            total: 161,
        };

        let mut parser = InputParser::new(false);
        parser.read_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(parser, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_parser_activity() {
        let mut parser = InputParser::new(true);
        assert_eq!(parser.active, ParserActivity::Active);

        parser.read_input("don't()");
        assert_eq!(parser.active, ParserActivity::Inactive);

        parser.read_input("do()");
        assert_eq!(parser.active, ParserActivity::Active);
    }

    #[test]
    fn test_read_input_togglable() {
        let expected = InputParser {
            active: ParserActivity::Active,
            state: ParserState::Blank,
            buffer: ['l', '(', '8', ',', '5', ')', ')'],
            total: 48,
        };

        let mut parser = InputParser::new(true);
        parser.read_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(parser, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
