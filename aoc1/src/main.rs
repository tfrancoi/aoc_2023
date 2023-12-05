use std::fs::File;
use std::io::Read;

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
    let mut sum: u32 = 0;
    let mut sum_second: u32 = 0;
    for line in input.split("\n").map(|x| x.trim()) {
        sum += get_number(line);
        sum_second += get_number(&replace(line));
    }
    return (sum, sum_second);
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
