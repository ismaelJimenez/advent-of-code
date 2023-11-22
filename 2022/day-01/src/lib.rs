use itertools::Itertools;

trait TopN<T> {
    fn top_n(self, n: usize) -> Vec<T>;
}

impl<T: PartialOrd, U: Iterator<Item = T>> TopN<T> for U {
    fn top_n(self, n: usize) -> Vec<T> {
        let mut top = Vec::with_capacity(n);
        
        for value in self {
            for i in 0..n {
                if let Some(top_value) = top.get(i) {
                    if value > *top_value {
                        top[i..].rotate_right(1);
                        top[i] = value;
                        break;
                    }
                } else {
                    top.push(value);
                    break;
                }
            }
        }
        top
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

pub fn part2_itertools(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

pub fn part2_no_sort(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top_n(3)
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part_2_itertools_works() {
        let result = part2_itertools(INPUT);
        assert_eq!(result, 45000);
    }

    #[test]
    fn part_2_no_sort_works() {
        let result = part2_no_sort(INPUT);
        assert_eq!(result, 45000);
    }
}
