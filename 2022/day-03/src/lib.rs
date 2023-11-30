use std::collections::HashSet;
mod by3;

struct Rucksack<'a> {
    compartment_a: &'a str,
    compartment_b: &'a str,
}

impl<'a> Rucksack<'a> {
    fn new(s: &'a str) -> Self {
        let len = s.len() / 2;
        Self {
            compartment_a: &s[..len],
            compartment_b: &s[len..],
        }
    }

    fn common_item(&self) -> char {
        let compartment_1_items = self.compartment_a.chars().collect::<HashSet<_>>();

        self.compartment_b
            .chars()
            .find(|c| compartment_1_items.contains(c))
            .unwrap()
    }
}

fn priority(c: char) -> Result<u64, &'static str> {
    match c {
        'A'..='Z' => Ok(c as u64 - 38),
        'a'..='z' => Ok(c as u64 - 96),
        _ => Err("Unknown char {c}"),
    }
}

fn find_badge(a: &str, b: &str, c: &str) -> char {
    a.chars()
        .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
        .unwrap()
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|content| Rucksack::new(content).common_item())
        .map(|c| priority(c).unwrap())
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    by3::By3Iter::new(input.lines())
        .map(|(a, b, c)| find_badge(a, b, c))
        .map(|c| priority(c).unwrap())
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }
}
