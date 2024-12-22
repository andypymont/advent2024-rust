use std::cmp::Ordering;
use std::iter::successors;
use std::str::FromStr;

advent_of_code::solution!(22);

const MIX: usize = 16_777_216;

#[derive(Debug, PartialEq)]
struct Buyer {
    secret: usize,
}

impl Buyer {
    const fn next_secret_number(secret: usize) -> usize {
        let secret = (secret ^ (secret * 64)) % MIX;
        let secret = (secret ^ (secret / 32)) % MIX;
        (secret ^ (secret * 2048)) % MIX
    }

    fn prices(&self) -> impl Iterator<Item = usize> {
        self.secret_numbers().map(|x| x % 10)
    }

    fn secret_numbers(&self) -> impl Iterator<Item = usize> {
        successors(Some(self.secret), |n| Some(Self::next_secret_number(*n))).take(2001)
    }
}

#[derive(Debug, PartialEq)]
struct RecentPriceChanges {
    prev: usize,
    recent: [Option<usize>; 4],
    seen: Vec<bool>,
}

impl RecentPriceChanges {
    fn new(prev: usize) -> Self {
        Self {
            prev,
            recent: [None; 4],
            seen: vec![false; 19 * 19 * 19 * 19],
        }
    }

    fn changes(&self) -> Option<usize> {
        let a = self.recent[0]?;
        let b = self.recent[1]?;
        let c = self.recent[2]?;
        let d = self.recent[3]?;
        Some((a * 19 * 19 * 19) + (b * 19 * 19) + (c * 19) + d)
    }

    fn push(&mut self, price: usize) -> Option<usize> {
        let change = match price.cmp(&self.prev) {
            Ordering::Equal => 0,
            Ordering::Less => price.abs_diff(self.prev),
            Ordering::Greater => price.abs_diff(self.prev) + 9,
        };

        self.prev = price;
        self.recent = [self.recent[1], self.recent[2], self.recent[3], Some(change)];

        self.changes().and_then(|changes| {
            if self.seen[changes] {
                None
            } else {
                self.seen[changes] = true;
                Some(changes)
            }
        })
    }
}

#[derive(Debug, PartialEq)]
struct Market {
    buyers: Vec<Buyer>,
}

impl Market {
    fn most_bananas_buyable(&self) -> Option<usize> {
        let mut bananas = vec![0; 19 * 19 * 19 * 19];

        for buyer in &self.buyers {
            let mut prices = buyer.prices();
            let mut recent = RecentPriceChanges::new(prices.next().unwrap_or(0));
            for price in prices {
                if let Some(changes) = recent.push(price) {
                    bananas[changes] += price;
                }
            }
        }

        bananas.into_iter().max()
    }

    fn total_final_secret_numbers(&self) -> usize {
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
pub fn part_one(input: &str) -> Option<usize> {
    Market::from_str(input).map_or(None, |market| Some(market.total_final_secret_numbers()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Market::from_str(input).map_or(None, |market| market.most_bananas_buyable())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_market() -> Market {
        Market {
            buyers: vec![
                Buyer { secret: 1 },
                Buyer { secret: 2 },
                Buyer { secret: 3 },
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
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_prices() {
        let buyer = Buyer { secret: 123 };
        let mut prices = buyer.prices();
        assert_eq!(prices.next(), Some(3));
        assert_eq!(prices.next(), Some(0));
        assert_eq!(prices.next(), Some(6));
        assert_eq!(prices.next(), Some(5));
        assert_eq!(prices.next(), Some(4));
        assert_eq!(prices.next(), Some(4));
        assert_eq!(prices.next(), Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
