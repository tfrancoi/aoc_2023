use std::collections::HashSet;
use regex::Regex;
use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 4)));
}

fn aoc(input: &str) -> (u32, u32) {
    let mut sum:u32 = 0;
    let re = Regex::new(r":(.*)\|(.*)").unwrap();
    let base:i32 = 2;
    let mut cards:Vec<(usize, usize)> = Vec::new();
    for line in split_input(input) {
        let matches: Vec<regex::Captures<'_>> = re.captures_iter(&line).collect();
        let winning = &matches[0][1].trim().split(" ").filter(|x| *x != "").collect::<HashSet<_>>();
        let numbers = &matches[0][2].trim().split(" ").filter(|x| *x != "").collect::<HashSet<_>>();
        let l = numbers.intersection(winning).count();
        cards.push((1, l));
        if l > 0 {
            sum += base.pow((l - 1) as u32) as u32;
        }
    }

    for pos in 0..cards.len(){
        let card = cards[pos];
        for i in pos+1..=pos+card.1 {
            cards[i] = (cards[i].0 + card.0, cards[i].1);
        }
    }
    let sum_second = cards.iter().map(|x| x.0 as u32).sum();

    return (sum, sum_second);
}