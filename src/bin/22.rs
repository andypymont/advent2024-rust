advent_of_code::solution!(22);

const MIX: u64 = 16_777_216;

fn secret_number(initial: u64, number: u64) -> u64 {
    let mut secret = initial;
    for _ in 0..number {
        let mult = secret * 64;
        secret = (secret ^ mult) % MIX;
        let div = secret / 32;
        secret = (secret ^ div) % MIX;
        let mult2 = secret * 2048;
        secret = (secret ^ mult2) % MIX;
    }
    secret
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| line.parse().map_or(None, |x| Some(secret_number(x, 2000))))
            .sum(),
    )
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        assert_eq!(secret_number(123, 1), 15887950);
        assert_eq!(secret_number(123, 2), 16495136);
        assert_eq!(secret_number(123, 3), 527345);
        assert_eq!(secret_number(1, 2000), 8685429);
        assert_eq!(secret_number(10, 2000), 4700978);
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
