use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(21);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CodeKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

#[derive(Debug, PartialEq)]
struct Code {
    number: usize,
    keys: Vec<CodeKey>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum DirectionKey {
    Up,
    Right,
    Down,
    Left,
    A,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectionSequence {
    length: usize,
    sequence: u64,
}

impl DirectionSequence {
    const fn new() -> Self {
        Self {
            length: 0,
            sequence: 0,
        }
    }

    const fn extended_with(&self, direction: DirectionKey) -> Self {
        let value = match direction {
            DirectionKey::Up => 1,
            DirectionKey::Right => 2,
            DirectionKey::Down => 3,
            DirectionKey::Left => 4,
            DirectionKey::A => 5,
        };
        Self {
            length: self.length + 1,
            sequence: self.sequence | (value << (3 * self.length)),
        }
    }
}

impl Iterator for DirectionSequence {
    type Item = DirectionKey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }

        let next = self.sequence % 8;
        self.sequence >>= 3;
        self.length -= 1;
        match next {
            1 => Some(DirectionKey::Up),
            2 => Some(DirectionKey::Right),
            3 => Some(DirectionKey::Down),
            4 => Some(DirectionKey::Left),
            5 => Some(DirectionKey::A),
            _ => None,
        }
    }
}

trait Key: Copy + PartialEq {
    const FORBIDDEN_POSITION: (u8, u8);

    fn get_position(&self) -> (u8, u8);

    fn shortest_paths(from: Self, to: Self) -> Vec<DirectionSequence> {
        let mut paths = Vec::new();

        if from == to {
            paths.push(DirectionSequence {
                length: 1,
                sequence: 5,
            });
            return paths;
        }

        let mut queue = VecDeque::new();
        let mut best = usize::MAX;
        let target = to.get_position();

        queue.push_back((from.get_position(), DirectionSequence::new()));

        while let Some((position, sequence)) = queue.pop_front() {
            if sequence.length > best {
                break;
            }
            if position == target {
                best = sequence.length;
                paths.push(sequence.extended_with(DirectionKey::A));
                continue;
            }
            if position == Self::FORBIDDEN_POSITION {
                continue;
            }
            match position.0.cmp(&target.0) {
                Ordering::Less => queue.push_back((
                    (position.0 + 1, position.1),
                    sequence.extended_with(DirectionKey::Up),
                )),
                Ordering::Greater => queue.push_back((
                    (position.0 - 1, position.1),
                    sequence.extended_with(DirectionKey::Down),
                )),
                Ordering::Equal => (),
            }
            match position.1.cmp(&target.1) {
                Ordering::Less => queue.push_back((
                    (position.0, position.1 + 1),
                    sequence.extended_with(DirectionKey::Right),
                )),
                Ordering::Greater => queue.push_back((
                    (position.0, position.1 - 1),
                    sequence.extended_with(DirectionKey::Left),
                )),
                Ordering::Equal => (),
            }
        }

        paths
    }
}

impl Key for CodeKey {
    const FORBIDDEN_POSITION: (u8, u8) = (0, 0);

    fn get_position(&self) -> (u8, u8) {
        match self {
            Self::Zero => (0, 1),
            Self::One => (1, 0),
            Self::Two => (1, 1),
            Self::Three => (1, 2),
            Self::Four => (2, 0),
            Self::Five => (2, 1),
            Self::Six => (2, 2),
            Self::Seven => (3, 0),
            Self::Eight => (3, 1),
            Self::Nine => (3, 2),
            Self::A => (0, 2),
        }
    }
}

impl Key for DirectionKey {
    const FORBIDDEN_POSITION: (u8, u8) = (1, 0);

    fn get_position(&self) -> (u8, u8) {
        match self {
            Self::Up => (1, 1),
            Self::Right => (0, 2),
            Self::Down => (0, 1),
            Self::Left => (0, 0),
            Self::A => (1, 2),
        }
    }
}

struct DirectionPadStack {
    height: usize,
}

impl DirectionPadStack {
    const fn new(height: usize) -> Self {
        Self { height }
    }

    fn shortest_path_for_code(&self, code: &Code) -> usize {
        let mut total = 0;

        for (ix, second) in code.keys.iter().enumerate() {
            let first = if ix == 0 {
                CodeKey::A
            } else {
                code.keys[ix - 1]
            };
            let paths = CodeKey::shortest_paths(first, *second);
            total += paths
                .into_iter()
                .map(|path| self.shortest_path_stacked(self.height, &path))
                .min()
                .unwrap_or(0);
        }

        total
    }

    #[allow(clippy::only_used_in_recursion)]
    fn shortest_path_stacked(&self, level: usize, path: &DirectionSequence) -> usize {
        let mut length = 0;
        let mut first = DirectionKey::A;

        for second in path.clone() {
            let paths = DirectionKey::shortest_paths(first, second);
            if level == 1 {
                length += paths.into_iter().map(|path| path.length).min().unwrap_or(0);
            } else {
                length += paths
                    .into_iter()
                    .map(|path| self.shortest_path_stacked(level - 1, &path))
                    .min()
                    .unwrap_or(0);
            }
            first = second;
        }

        length
    }
}

#[derive(Debug, PartialEq)]
struct ParseCodeError;

impl TryFrom<char> for CodeKey {
    type Error = ParseCodeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'A' => Ok(Self::A),
            _ => Err(ParseCodeError),
        }
    }
}

impl FromStr for Code {
    type Err = ParseCodeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut keys = Vec::new();
        let mut number = 0;
        for ch in line.trim().chars() {
            if let Some(digit) = ch.to_digit(10) {
                number = (number * 10) + digit;
            }

            let key = ch.try_into()?;
            keys.push(key);
        }
        let number = number.try_into().map_err(|_| ParseCodeError)?;

        Ok(Self { number, keys })
    }
}

impl Code {
    fn vec_from_str(input: &str) -> Result<Vec<Self>, ParseCodeError> {
        let mut codes = Vec::new();

        for line in input.lines() {
            let code = Self::from_str(line)?;
            codes.push(code);
        }

        Ok(codes)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Code::vec_from_str(input).ok().map(|codes| {
        let dpad = DirectionPadStack::new(2);
        codes
            .iter()
            .map(|code| dpad.shortest_path_for_code(code) * code.number)
            .sum()
    })
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_codes() -> Vec<Code> {
        vec![
            Code {
                number: 29,
                keys: vec![CodeKey::Zero, CodeKey::Two, CodeKey::Nine, CodeKey::A],
            },
            Code {
                number: 980,
                keys: vec![CodeKey::Nine, CodeKey::Eight, CodeKey::Zero, CodeKey::A],
            },
            Code {
                number: 179,
                keys: vec![CodeKey::One, CodeKey::Seven, CodeKey::Nine, CodeKey::A],
            },
            Code {
                number: 456,
                keys: vec![CodeKey::Four, CodeKey::Five, CodeKey::Six, CodeKey::A],
            },
            Code {
                number: 379,
                keys: vec![CodeKey::Three, CodeKey::Seven, CodeKey::Nine, CodeKey::A],
            },
        ]
    }

    #[test]
    fn test_parse_codes() {
        assert_eq!(
            Code::vec_from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_codes()),
        );
    }

    #[test]
    fn test_code_key_shortest_paths() {
        assert_eq!(
            CodeKey::shortest_paths(CodeKey::Zero, CodeKey::Zero),
            vec![DirectionSequence::new().extended_with(DirectionKey::A)],
        );
        assert_eq!(
            CodeKey::shortest_paths(CodeKey::Zero, CodeKey::Two),
            vec![DirectionSequence::new()
                .extended_with(DirectionKey::Up)
                .extended_with(DirectionKey::A)],
        );
        assert_eq!(
            CodeKey::shortest_paths(CodeKey::Four, CodeKey::Two),
            vec![
                DirectionSequence::new()
                    .extended_with(DirectionKey::Down)
                    .extended_with(DirectionKey::Right)
                    .extended_with(DirectionKey::A),
                DirectionSequence::new()
                    .extended_with(DirectionKey::Right)
                    .extended_with(DirectionKey::Down)
                    .extended_with(DirectionKey::A),
            ],
        );
    }

    #[test]
    fn test_direction_key_shortest_paths() {
        assert_eq!(
            DirectionKey::shortest_paths(DirectionKey::Up, DirectionKey::Up),
            vec![DirectionSequence::new().extended_with(DirectionKey::A)],
        );
        assert_eq!(
            DirectionKey::shortest_paths(DirectionKey::Up, DirectionKey::Left),
            vec![DirectionSequence::new()
                .extended_with(DirectionKey::Down)
                .extended_with(DirectionKey::Left)
                .extended_with(DirectionKey::A)],
        );
        assert_eq!(
            DirectionKey::shortest_paths(DirectionKey::A, DirectionKey::Down),
            vec![
                DirectionSequence::new()
                    .extended_with(DirectionKey::Down)
                    .extended_with(DirectionKey::Left)
                    .extended_with(DirectionKey::A),
                DirectionSequence::new()
                    .extended_with(DirectionKey::Left)
                    .extended_with(DirectionKey::Down)
                    .extended_with(DirectionKey::A),
            ],
        );
    }

    #[test]
    fn test_directionpadstack_shortest_path_for_code() {
        let codes = example_codes();
        let dpad = DirectionPadStack::new(2);
        assert_eq!(dpad.shortest_path_for_code(&codes[0]), 68);
        assert_eq!(dpad.shortest_path_for_code(&codes[1]), 60);
        assert_eq!(dpad.shortest_path_for_code(&codes[2]), 68);
        assert_eq!(dpad.shortest_path_for_code(&codes[3]), 64);
        assert_eq!(dpad.shortest_path_for_code(&codes[4]), 64);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126_384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
