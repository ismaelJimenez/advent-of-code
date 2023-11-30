use std::ops::RangeInclusive;
use std::str::FromStr;

struct Ranges {
    first: RangeInclusive<u64>,
    second: RangeInclusive<u64>,
}

fn parse_range(s: &str) -> Result<RangeInclusive<u64>, &'static str> {
    if let Some((first, second)) = s.split_once('-') {
        return Ok(first.parse().unwrap()..=second.parse().unwrap());
    }

    Err("Invalid String")
}

impl FromStr for Ranges {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split_once(',') {
            return Ok(Self {
                first: parse_range(first).unwrap(),
                second: parse_range(second).unwrap(),
            });
        }

        Err("Wrong input")
    }
}

fn range_contains(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.contains(second.start()) && first.contains(second.end())
}

fn range_overlap(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.contains(second.start())
}

impl Ranges {
    fn contained(&self) -> bool {
        range_contains(&self.first, &self.second) || range_contains(&self.second, &self.first)
    }

    fn overlapping(&self) -> bool {
        range_overlap(&self.first, &self.second) || range_overlap(&self.second, &self.first)
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| line.parse::<Ranges>().unwrap().contained())
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| line.parse::<Ranges>().unwrap().overlapping())
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
