use std::str::FromStr;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    const fn process(&self, first: bool, second: bool) -> bool {
        match self {
            Self::And => first && second,
            Self::Or => first || second,
            Self::Xor => first ^ second,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Gate {
    operation: Operation,
    inputs: [usize; 2],
    output: usize,
}

#[derive(Debug, PartialEq)]
struct System {
    wires: Vec<Option<bool>>,
    gates: Vec<Gate>,
}

impl System {
    fn calculate(mut self) -> u32 {
        loop {
            let mut changed = false;

            for gate in &self.gates {
                if self.wires[gate.output].is_some() {
                    continue;
                }
                let Some(first) = self.wires[gate.inputs[0]] else {
                    continue;
                };
                let Some(second) = self.wires[gate.inputs[1]] else {
                    continue;
                };
                self.wires[gate.output] = Some(gate.operation.process(first, second));
                changed = true;
            }

            if !changed {
                break;
            }
        }

        self.result()
    }

    fn get_result_digit(&self, digit: usize) -> u32 {
        let tens = digit / 10;
        let ones = digit % 10;
        let key = (35 * 36 * 36) + (tens * 36) + ones;
        u32::from(self.wires[key].unwrap_or(false))
    }

    fn result(&self) -> u32 {
        (0..32).map(|x| self.get_result_digit(x) << x).sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseSystemError;

fn parse_wire(wire: &str) -> Result<usize, ParseSystemError> {
    let mut value = 0;

    for ch in wire.chars() {
        let digit = ch.to_digit(36).ok_or(ParseSystemError)?;
        value = (value * 36) + digit;
    }

    usize::try_from(value).map_err(|_| ParseSystemError)
}

impl FromStr for Operation {
    type Err = ParseSystemError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(ParseSystemError),
        }
    }
}

impl FromStr for Gate {
    type Err = ParseSystemError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (input, output) = input.split_once(" -> ").ok_or(ParseSystemError)?;
        let output = parse_wire(output)?;

        let mut parts = input.split_whitespace();
        let first = parts.next().ok_or(ParseSystemError).and_then(parse_wire)?;
        let operation = parts
            .next()
            .ok_or(ParseSystemError)
            .and_then(Operation::from_str)?;
        let second = parts.next().ok_or(ParseSystemError).and_then(parse_wire)?;

        Ok(Self {
            operation,
            inputs: [first, second],
            output,
        })
    }
}

impl FromStr for System {
    type Err = ParseSystemError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut wires = vec![None; 36 * 36 * 36];
        let mut gates = Vec::new();

        let (wires_str, gates_str) = input.split_once("\n\n").ok_or(ParseSystemError)?;

        for line in wires_str.lines() {
            let (wire, value) = line.split_once(": ").ok_or(ParseSystemError)?;
            let wire = parse_wire(wire)?;
            let value = match value.chars().next() {
                Some('1') => true,
                Some('0') => false,
                _ => return Err(ParseSystemError),
            };
            wires[wire] = Some(value);
        }

        for line in gates_str.lines() {
            let gate = Gate::from_str(line)?;
            gates.push(gate);
        }

        Ok(Self { wires, gates })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    System::from_str(input).ok().map(System::calculate)
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_system() {
        let mut wires = vec![None; 36 * 36 * 36];
        wires[42768] = Some(true);
        wires[42769] = Some(false);
        wires[42770] = Some(true);
        wires[42771] = Some(true);
        wires[42772] = Some(false);
        wires[44064] = Some(true);
        wires[44065] = Some(true);
        wires[44066] = Some(true);
        wires[44067] = Some(true);
        wires[44068] = Some(true);

        let system = System {
            wires,
            gates: vec![
                Gate {
                    operation: Operation::Xor,
                    inputs: [30868, 20044],
                    output: 29207,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [44066, 42769],
                    output: 38444,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [27098, 26839],
                    output: 45365,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [42768, 42771],
                    output: 20477,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [38173, 36124],
                    output: 45361,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [40673, 38444],
                    output: 14828,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [14828, 20431],
                    output: 45396,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [19997, 30793],
                    output: 15212,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [44064, 44067],
                    output: 17554,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [44067, 44064],
                    output: 33425,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [15212, 20431],
                    output: 45368,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [38444, 20477],
                    output: 20431,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [21583, 38173],
                    output: 45397,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [14828, 29207],
                    output: 45360,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [42771, 42768],
                    output: 40673,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [21583, 42383],
                    output: 45362,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [42772, 44064],
                    output: 26616,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [17554, 32818],
                    output: 34340,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [30793, 40673],
                    output: 23206,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [26616, 20477],
                    output: 36124,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [44068, 44066],
                    output: 20044,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [44065, 42770],
                    output: 32818,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [30868, 26616],
                    output: 27098,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [33425, 20044],
                    output: 38173,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [34340, 38173],
                    output: 45369,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [32818, 17554],
                    output: 26839,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [42771, 44067],
                    output: 19997,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [42768, 44068],
                    output: 30868,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [14828, 15212],
                    output: 45366,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [30793, 20044],
                    output: 42383,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [20431, 34340],
                    output: 45364,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [15212, 20431],
                    output: 45367,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [44067, 42769],
                    output: 30793,
                },
                Gate {
                    operation: Operation::And,
                    inputs: [23206, 15212],
                    output: 45363,
                },
                Gate {
                    operation: Operation::Xor,
                    inputs: [38173, 36124],
                    output: 45398,
                },
                Gate {
                    operation: Operation::Or,
                    inputs: [38444, 32818],
                    output: 21583,
                },
            ],
        };
        assert_eq!(
            System::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(system),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
