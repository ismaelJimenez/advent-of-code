use day_01::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/input.txt").unwrap();
    println!("{}", process_part1(&file));
}