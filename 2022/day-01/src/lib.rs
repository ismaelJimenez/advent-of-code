pub fn process_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn process_part2(input: &str) -> u32 {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum()
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
        let result = process_part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part_2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 45000);
    }
}
