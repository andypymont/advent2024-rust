advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
struct Operand {
    value: Option<u32>,
}

impl Operand {
    fn new() -> Self {
        Self { value: None }
    }

    fn add(&mut self, digit: u32) {
        self.value = Some(match self.value {
            None => digit,
            Some(existing) => (10 * existing) + digit,
        })
    }

    fn clear(&mut self) {
        self.value = None;
    }
}

#[derive(Debug, PartialEq)]
enum ParserState {
    Blank,
    FirstOperand,
    SecondOperand,
}

#[derive(Debug, PartialEq)]
struct InputParser {
    state: ParserState,
    buffer: [char; 4],
    first_operand: Operand,
    second_operand: Operand,
    instructions: Vec<(u32, u32)>,
}

impl InputParser {
    fn new() -> Self {
        Self {
            state: ParserState::Blank,
            buffer: [' ', ' ', ' ', ' '],
            first_operand: Operand::new(),
            second_operand: Operand::new(),
            instructions: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.state = ParserState::Blank;
        self.first_operand.clear();
        self.second_operand.clear();
    }

    fn record_and_clear(&mut self) {
        let Some(first) = self.first_operand.value else {
            return self.clear();
        };
        let Some(second) = self.second_operand.value else {
            return self.clear();
        };
        self.instructions.push((first, second));
        self.clear();
    }

    fn read_char(&mut self, input: char) {
        self.buffer = [self.buffer[1], self.buffer[2], self.buffer[3], input];
        match self.state {
            ParserState::Blank => {
                if self.buffer == ['m', 'u', 'l', '('] {
                    self.state = ParserState::FirstOperand;
                }
            }
            ParserState::FirstOperand => {
                if let Some(digit) = input.to_digit(10) {
                    self.first_operand.add(digit);
                } else if input == ',' {
                    if self.first_operand.value.is_some() {
                        self.state = ParserState::SecondOperand;
                    } else {
                        self.clear();
                    }
                } else {
                    self.clear();
                }
            }
            ParserState::SecondOperand => {
                if let Some(digit) = input.to_digit(10) {
                    self.second_operand.add(digit);
                } else if input == ')' {
                    if self.second_operand.value.is_some() {
                        self.record_and_clear();
                    } else {
                        self.clear();
                    }
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

    fn total_value(&self) -> u32 {
        self.instructions.iter().map(|(a, b)| a * b).sum()
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut parser = InputParser::new();
    parser.read_input(input);
    Some(parser.total_value())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_collection() {
        let mut operand = Operand::new();
        assert_eq!(operand.value, None);

        operand.add(4);
        assert_eq!(operand.value, Some(4));

        operand.add(2);
        assert_eq!(operand.value, Some(42));

        operand.add(1);
        assert_eq!(operand.value, Some(421));

        operand.clear();
        assert_eq!(operand.value, None);
    }

    #[test]
    fn test_parse_first_instruction() {
        let mut parser = InputParser::new();
        assert_eq!(parser.state, ParserState::Blank);

        parser.read_char('m');
        parser.read_char('u');
        parser.read_char('l');
        parser.read_char('(');
        assert_eq!(parser.state, ParserState::FirstOperand);
        parser.read_char('2');
        assert_eq!(parser.state, ParserState::FirstOperand);
        parser.read_char(',');
        assert_eq!(parser.state, ParserState::SecondOperand);
        parser.read_char('4');
        assert_eq!(parser.state, ParserState::SecondOperand);
        parser.read_char(')');
        assert_eq!(parser.state, ParserState::Blank);
        assert_eq!(parser.instructions.get(0), Some((2, 4)).as_ref());
    }

    #[test]
    fn test_read_input() {
        let expected = InputParser {
            state: ParserState::Blank,
            buffer: [',', '5', ')', ')'],
            first_operand: Operand { value: None },
            second_operand: Operand { value: None },
            instructions: vec![(2, 4), (5, 5), (11, 8), (8, 5)],
        };

        let mut parser = InputParser::new();
        parser.read_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(parser, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
