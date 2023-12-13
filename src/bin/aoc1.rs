use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 1)));
}

fn replace(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn get_number(line: &str) -> u32 {
    let digits = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    digits[0] * 10 + digits[digits.len() - 1]
}

fn aoc(input: &str) -> (u32, u32) {
    split_input(input).into_iter().fold((0, 0), |(a, b), line| (a + get_number(&line), b + get_number(&replace(&line))))
}
