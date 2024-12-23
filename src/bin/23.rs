use std::str::FromStr;

advent_of_code::solution!(23);

const MAX_COMPUTERS: usize = 676;

#[derive(Debug, PartialEq)]
struct ComputerSet {
    computers: Vec<bool>,
}

impl ComputerSet {
    fn new() -> Self {
        Self {
            computers: vec![false; MAX_COMPUTERS],
        }
    }

    fn insert(&mut self, computer: usize) -> bool {
        let present = self.computers[computer];
        self.computers[computer] = true;
        !present
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.computers
            .iter()
            .enumerate()
            .filter_map(|(ix, present)| if *present { Some(ix) } else { None })
    }
}

#[derive(Debug, PartialEq)]
struct Connections {
    connections: Vec<bool>,
}

impl Connections {
    fn new() -> Self {
        Self {
            connections: vec![false; MAX_COMPUTERS * MAX_COMPUTERS],
        }
    }

    fn contains(&self, first: usize, second: usize) -> bool {
        self.connections[(first * MAX_COMPUTERS) + second]
    }

    fn insert(&mut self, first: usize, second: usize) -> bool {
        let present = self.contains(first, second);
        self.connections[(first * MAX_COMPUTERS) + second] = true;
        self.connections[(second * MAX_COMPUTERS) + first] = true;
        !present
    }
}

#[derive(Debug, PartialEq)]
struct Network {
    computers: ComputerSet,
    connections: Connections,
}

impl Network {
    fn new() -> Self {
        Self {
            computers: ComputerSet::new(),
            connections: Connections::new(),
        }
    }

    fn connected_trios(&self) -> impl Iterator<Item = ComputerSet> + '_ {
        self.computers.iter().flat_map(move |a| {
            self.computers.iter().flat_map(move |b| {
                self.computers.iter().filter_map(move |c| {
                    if a >= b
                        || a >= c
                        || b >= c
                        || !self.connections.contains(a, b)
                        || !self.connections.contains(a, c)
                        || !self.connections.contains(b, c)
                    {
                        None
                    } else {
                        let mut trio = ComputerSet::new();
                        trio.insert(a);
                        trio.insert(b);
                        trio.insert(c);
                        Some(trio)
                    }
                })
            })
        })
    }
}

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

fn parse_digit(digit: char) -> Result<usize, ParseNetworkError> {
    let digit_u32 = digit
        .to_digit(36)
        .and_then(|d| d.checked_sub(10))
        .ok_or(ParseNetworkError)?;
    digit_u32.try_into().map_err(|_| ParseNetworkError)
}

fn parse_computer(computer: &str) -> Result<usize, ParseNetworkError> {
    let mut chars = computer.chars();
    let first = chars.next().ok_or(ParseNetworkError)?;
    let second = chars.next().ok_or(ParseNetworkError)?;
    if chars.next().is_some() {
        return Err(ParseNetworkError);
    }

    let first = parse_digit(first)?;
    let second = parse_digit(second)?;
    Ok((first * 26) + second)
}

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut network = Self::new();

        for line in input.lines() {
            let Some((first, second)) = line.split_once('-') else {
                return Err(ParseNetworkError);
            };
            let first = parse_computer(first)?;
            let second = parse_computer(second)?;

            network.computers.insert(first);
            network.computers.insert(second);
            network.connections.insert(first, second);
        }

        Ok(network)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Network::from_str(input).map_or(None, |network| {
        Some(
            network
                .connected_trios()
                .filter(|trio| trio.iter().any(|computer| computer / 26 == 19))
                .count(),
        )
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

    fn example_network() -> Network {
        let mut computers = ComputerSet::new();
        let mut connections = Connections::new();

        let aq = 16;
        let cg = 58;
        let co = 66;
        let de = 82;
        let ka = 260;
        let kh = 267;
        let qp = 431;
        let ta = 494;
        let tb = 495;
        let tc = 496;
        let td = 497;
        let ub = 521;
        let vc = 548;
        let wh = 579;
        let wq = 588;
        let yn = 637;

        computers.insert(aq);
        computers.insert(cg);
        computers.insert(co);
        computers.insert(de);
        computers.insert(ka);
        computers.insert(kh);
        computers.insert(qp);
        computers.insert(ta);
        computers.insert(tb);
        computers.insert(tc);
        computers.insert(td);
        computers.insert(ub);
        computers.insert(vc);
        computers.insert(wh);
        computers.insert(wq);
        computers.insert(yn);

        connections.insert(kh, tc);
        connections.insert(qp, kh);
        connections.insert(de, cg);
        connections.insert(ka, co);
        connections.insert(yn, aq);
        connections.insert(qp, ub);
        connections.insert(cg, tb);
        connections.insert(vc, aq);
        connections.insert(tb, ka);
        connections.insert(wh, tc);
        connections.insert(yn, cg);
        connections.insert(kh, ub);
        connections.insert(ta, co);
        connections.insert(de, co);
        connections.insert(tc, td);
        connections.insert(tb, wq);
        connections.insert(wh, td);
        connections.insert(ta, ka);
        connections.insert(td, qp);
        connections.insert(aq, cg);
        connections.insert(wq, ub);
        connections.insert(ub, vc);
        connections.insert(de, ta);
        connections.insert(wq, aq);
        connections.insert(wq, vc);
        connections.insert(wh, yn);
        connections.insert(ka, de);
        connections.insert(kh, ta);
        connections.insert(co, tc);
        connections.insert(wh, qp);
        connections.insert(tb, vc);
        connections.insert(td, yn);

        Network {
            computers,
            connections,
        }
    }

    #[test]
    fn test_network_from_str() {
        assert_eq!(
            Network::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_network()),
        );
    }

    #[test]
    fn test_network_connected_trios() {
        let aq = 16;
        let cg = 58;
        let co = 66;
        let de = 82;
        let ka = 260;
        let kh = 267;
        let qp = 431;
        let ta = 494;
        let tb = 495;
        let tc = 496;
        let td = 497;
        let ub = 521;
        let vc = 548;
        let wh = 579;
        let wq = 588;
        let yn = 637;

        let network = example_network();
        let mut trios = network.connected_trios();

        let mut trio = ComputerSet::new();
        trio.insert(aq);
        trio.insert(cg);
        trio.insert(yn);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(aq);
        trio.insert(vc);
        trio.insert(wq);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(co);
        trio.insert(de);
        trio.insert(ka);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(co);
        trio.insert(de);
        trio.insert(ta);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(co);
        trio.insert(ka);
        trio.insert(ta);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(de);
        trio.insert(ka);
        trio.insert(ta);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(kh);
        trio.insert(qp);
        trio.insert(ub);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(qp);
        trio.insert(td);
        trio.insert(wh);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(tb);
        trio.insert(vc);
        trio.insert(wq);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(tc);
        trio.insert(td);
        trio.insert(wh);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(td);
        trio.insert(wh);
        trio.insert(yn);
        assert_eq!(trios.next(), Some(trio));

        let mut trio = ComputerSet::new();
        trio.insert(ub);
        trio.insert(vc);
        trio.insert(wq);
        assert_eq!(trios.next(), Some(trio));

        assert_eq!(trios.next(), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
