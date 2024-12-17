use std::str::FromStr;

advent_of_code::solution!(17);

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Debug, PartialEq)]
struct Program {
    registers: [u32; 3],
    instructions: Vec<u32>,
}

impl Program {
    fn run(&self) -> Vec<u32> {
        let mut output = Vec::new();
        let mut ip = 0;
        let mut registers = self.registers;

        loop {
            let Some(opcode) = self.instructions.get(ip) else {
                break;
            };
            let Some(operand) = self.instructions.get(ip + 1) else {
                break;
            };
            let combo = match operand {
                4 => registers[A],
                5 => registers[B],
                6 => registers[C],
                _ => *operand,
            };

            let mut adjust_ip = None;
            match opcode {
                0 | 6 | 7 => {
                    // ADV / BDV / CDV
                    let numerator = registers[A];
                    let denominator = 2_u32.pow(combo);
                    let target = match opcode {
                        0 => A,
                        6 => B,
                        _ => C,
                    };
                    registers[target] = numerator / denominator;
                }
                1 => {
                    // BXL
                    registers[B] ^= operand;
                }
                2 => {
                    // BST
                    registers[B] = combo % 8;
                }
                3 => {
                    // JNX
                    if registers[A] != 0 {
                        adjust_ip = Some(*operand);
                    }
                }
                4 => {
                    // BXC
                    registers[B] ^= registers[C];
                }
                5 => {
                    // OUT
                    output.push(combo % 8);
                }
                _ => (),
            }

            ip = adjust_ip.map_or(ip + 2, |ip| usize::try_from(ip).unwrap_or(0));
        }

        output
    }
}

#[derive(Debug, PartialEq)]
struct ParseProgramError;

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (registers_str, instructions_str) =
            input.split_once("\n\n").ok_or(ParseProgramError)?;

        let mut lines = registers_str.lines();
        let mut registers = [0, 0, 0];
        for reg in &mut registers {
            let value = lines.next().ok_or(ParseProgramError)?;
            let value = value[12..].parse().map_err(|_| ParseProgramError)?;
            *reg = value;
        }

        let instructions_str = instructions_str
            .trim()
            .strip_prefix("Program: ")
            .ok_or(ParseProgramError)?;
        let mut instructions = Vec::new();
        for instruction in instructions_str.split(',') {
            let instruction = instruction.parse().map_err(|_| ParseProgramError)?;
            instructions.push(instruction);
        }

        Ok(Self {
            registers,
            instructions,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<String> {
    Program::from_str(input).map_or(None, |program| {
        let mut output = String::new();
        for out in program.run() {
            if !output.is_empty() {
                output.push(',');
            }
            output.push_str(&out.to_string());
        }
        Some(output)
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

    fn example_program() -> Program {
        Program {
            registers: [729, 0, 0],
            instructions: vec![0, 1, 5, 4, 3, 0],
        }
    }

    #[test]
    fn test_parse_program() {
        assert_eq!(
            Program::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_program()),
        );
    }

    #[test]
    fn test_run_program() {
        let program = Program {
            registers: [10, 0, 0],
            instructions: vec![5, 0, 5, 1, 5, 4],
        };
        assert_eq!(program.run(), vec![0, 1, 2]);

        let program = Program {
            registers: [2024, 1, 2],
            instructions: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(program.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
