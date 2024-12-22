use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
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

    fn prices(&self) -> impl Iterator<Item = u64> {
        self.secret_numbers().map(|x| x % 10)
    }

    fn secret_numbers(&self) -> impl Iterator<Item = u64> {
        successors(Some(self.secret), |n| Some(Self::next_secret_number(*n))).take(2001)
    }
}

type PriceChange = (Ordering, u64);

#[derive(Debug, PartialEq)]
struct RecentPriceChanges {
    a: Option<PriceChange>,
    b: Option<PriceChange>,
    c: Option<PriceChange>,
    d: Option<PriceChange>,
}

impl RecentPriceChanges {
    const fn new() -> Self {
        Self {
            a: None,
            b: None,
            c: None,
            d: None,
        }
    }

    fn changes(&self) -> Option<(PriceChange, PriceChange, PriceChange, PriceChange)> {
        let a = self.a?;
        let b = self.b?;
        let c = self.c?;
        let d = self.d?;
        Some((a, b, c, d))
    }

    fn push(&mut self, change: PriceChange) {
        self.a = self.b;
        self.b = self.c;
        self.c = self.d;
        self.d = Some(change);
    }
}

#[derive(Debug, PartialEq)]
struct Market {
    buyers: Vec<Buyer>,
}

impl Market {
    fn most_bananas_buyable(&self) -> Option<u64> {
        let mut bananas = BTreeMap::new();

        for buyer in &self.buyers {
            let mut prices = buyer.prices();
            let mut seen = BTreeSet::new();
            let mut prev = prices.next().unwrap_or(0);
            let mut recent = RecentPriceChanges::new();
            for price in prices {
                recent.push((price.cmp(&prev), price.abs_diff(prev)));
                prev = price;

                if let Some(changes) = recent.changes() {
                    if seen.insert(changes) {
                        bananas
                            .entry(changes)
                            .and_modify(|x| *x += price)
                            .or_insert(price);
                    }
                }
            }
        }

        bananas.values().max().copied()
    }

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

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
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
