use std::iter::successors;
use std::str::FromStr;

advent_of_code::solution!(22);

const MIX: u64 = 16_777_216;

#[derive(Debug, PartialEq)]
struct Buyer {
    secret: u64,
}

impl Buyer {
    const fn next_secret_number(secret: u64) -> u64 {
        let secret = (secret ^ (secret * 64)) % MIX;
        let secret = (secret ^ (secret / 32)) % MIX;
        (secret ^ (secret * 2048)) % MIX
    }

    fn secret_numbers(&self) -> impl Iterator<Item = u64> {
        successors(Some(self.secret), |n| Some(Self::next_secret_number(*n))).take(2001)
    }
}

#[derive(Debug, PartialEq)]
struct Market {
    buyers: Vec<Buyer>,
}

impl Market {
    fn total_final_secret_numbers(&self) -> u64 {
        self.buyers
            .iter()
            .map(|buyer| buyer.secret_numbers().last().unwrap_or(0))
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseMarketError;

impl FromStr for Market {
    type Err = ParseMarketError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut buyers = Vec::new();

        for line in input.lines() {
            let secret = line.parse().map_err(|_| ParseMarketError)?;
            buyers.push(Buyer { secret });
        }

        Ok(Self { buyers })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Market::from_str(input).map_or(None, |market| Some(market.total_final_secret_numbers()))
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_market() -> Market {
        Market {
            buyers: vec![
                Buyer { secret: 1 },
                Buyer { secret: 10 },
                Buyer { secret: 100 },
                Buyer { secret: 2024 },
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Market::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_market()),
        );
    }

    #[test]
    fn test_secret_numbers() {
        let buyer = Buyer { secret: 123 };
        let mut secrets = buyer.secret_numbers();
        assert_eq!(secrets.next(), Some(123));
        assert_eq!(secrets.next(), Some(15887950));
        assert_eq!(secrets.next(), Some(16495136));
        assert_eq!(secrets.next(), Some(527345));

        let buyer = Buyer { secret: 1 };
        assert_eq!(buyer.secret_numbers().last(), Some(8685429));

        let buyer = Buyer { secret: 10 };
        assert_eq!(buyer.secret_numbers().last(), Some(4700978));

        let buyer = Buyer { secret: 100 };
        assert_eq!(buyer.secret_numbers().last(), Some(15273692));

        let buyer = Buyer { secret: 2024 };
        assert_eq!(buyer.secret_numbers().last(), Some(8667524));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
