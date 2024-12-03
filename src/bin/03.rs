advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParserState {
    Blank,
    FirstOperand(Option<u32>),
    SecondOperand(u32, Option<u32>),
}

#[derive(Debug, PartialEq)]
struct InputParser {
    active: Option<bool>,
    state: ParserState,
    buffer: [char; 7],
    total: u32,
}

impl InputParser {
    fn new(toggle: bool) -> Self {
        Self {
            active: if toggle { Some(true) } else { None },
            state: ParserState::Blank,
            buffer: [' '; 7],
            total: 0,
        }
    }

    fn activate(&mut self) {
        self.active = self.active.map(|_| true);
    }

    fn deactivate(&mut self) {
        self.active = self.active.map(|_| false);
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

        self.state = match self.state {
            ParserState::Blank => {
                if self.buffer[3..7] == ['m', 'u', 'l', '('] {
                    ParserState::FirstOperand(None)
                } else {
                    ParserState::Blank
                }
            }
            ParserState::FirstOperand(first) => match (input.to_digit(10), first, input == ',') {
                (Some(digit), None, _) => ParserState::FirstOperand(Some(digit)),
                (Some(digit), Some(f), _) => ParserState::FirstOperand(Some((f * 10) + digit)),
                (None, Some(f), true) => ParserState::SecondOperand(f, None),
                _ => ParserState::Blank,
            },
            ParserState::SecondOperand(first, second) => {
                match (
                    input.to_digit(10),
                    second,
                    input == ')',
                    self.active.unwrap_or(true),
                ) {
                    (Some(digit), None, _, _) => ParserState::SecondOperand(first, Some(digit)),
                    (Some(digit), Some(s), _, _) => {
                        ParserState::SecondOperand(first, Some((s * 10) + digit))
                    }
                    (None, Some(s), true, true) => {
                        self.total += first * s;
                        ParserState::Blank
                    }
                    _ => ParserState::Blank,
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
            active: None,
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
    fn test_read_input_togglable() {
        let expected = InputParser {
            active: Some(true),
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
