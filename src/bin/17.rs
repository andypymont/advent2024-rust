use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(17);

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Debug, PartialEq)]
struct Program {
    registers: [usize; 3],
    instructions: Vec<usize>,
}

impl Program {
    fn run(&self, substitute_a: Option<usize>) -> Vec<usize> {
        let mut output = Vec::new();
        let mut ip = 0;
        let mut registers = self.registers;
        if let Some(a) = substitute_a {
            registers[A] = a;
        }

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
                    let denominator = 1 << combo;
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

            ip = adjust_ip.unwrap_or(ip + 2);
        }

        output
    }

    fn find_self_producing_program(&self) -> Option<usize> {
        // The program in my input does this:
        // loop {
        //   b = a % 8;                 collect last 3 digits of a, store in b
        //   b ^= 7;                    flip the 3 digits of b (in place)
        //   c = a / 2.pow(b);          remove b digits from a, store in c
        //   a = a / 8;                 remove 3 digits from a (in place)
        //   b &= c;                    ???
        //   b ^= 7;                    flip the 3 digits of b (in place)
        //   output(b % 8);             output last 3 digits of b
        //   if a == 0 { break; }       finish if a is now fully consumed
        // }
        //
        // Therefore I want to construct a value in three-binary-digit blocks. Some values can
        // affect subsequent ones, so need to consider multiple possibilities - therefore using
        // BFS.

        let mut queue = VecDeque::new();
        queue.push_back((1, 0));

        while let Some((output, a)) = queue.pop_front() {
            if output <= self.instructions.len() {
                (0..8).for_each(|candidate| {
                    let candidate = (a << 3) + candidate;
                    let result = self.run(Some(candidate));
                    if let Some(result) = result.first() {
                        if result == &self.instructions[self.instructions.len() - output] {
                            queue.push_back((output + 1, candidate));
                        }
                    }
                });
            } else if self.run(Some(a)) == self.instructions {
                return Some(a);
            }
        }

        None
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
        for out in program.run(None) {
            if !output.is_empty() {
                output.push(',');
            }
            output.push_str(&out.to_string());
        }
        Some(output)
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Program::from_str(input).map_or(None, |program| program.find_self_producing_program())
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
        assert_eq!(program.run(None), vec![0, 1, 2]);

        let program = Program {
            registers: [2024, 1, 2],
            instructions: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(program.run(None), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }
}
