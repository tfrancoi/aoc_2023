use std::collections::HashSet;
use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 3)));
}

#[derive(Debug,PartialEq)]
struct Number {
    pos: Vec<(isize, isize)>,
    n: String,
}

impl Number {
    fn new() -> Number {
        Number {
            pos: Vec::new(),
            n: String::new(),
        }
    }

    fn add(&mut self, point: (isize, isize), symbol: char) {
        self.pos.push(point);
        self.n.push_str(&symbol.to_string());
    }

    fn is_empty(&self) -> bool {
        self.pos.len() == 0
    }

    fn get_num(&self) -> u32 {
        self.n.parse().unwrap()
    }

    fn is_part(&self, symbols: &HashSet<(isize, isize)>) -> u32 {
        for (l_offset, c_offset) in Number::get_adj_offset().iter() {
            for (l, c) in self.pos.iter() {
                if symbols.contains(&(l + l_offset, c + c_offset)) {
                    return self.get_num();
                }
            }
        }
        return 0;
    }

    fn is_adjacent(&self, pos: &(isize, isize)) -> bool {
        for (l_offset, c_offset) in Number::get_adj_offset().iter() {
            for (l, c) in self.pos.iter() {
                if (l + l_offset, c + c_offset) == *pos{
                    return true;
                }
            }
        }
        return false;
    }

    fn get_adj_offset() -> Vec<(isize, isize)> {
        vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1), ( 0, 0), ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ]
    }
}

fn aoc(input: &str) -> (u32, u32) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols:HashSet<(isize, isize)> = HashSet::new();
    let mut gears:Vec<(isize, isize)> = Vec::new();
    for (line_nbr, line) in split_input(input).iter().enumerate() {
        let new_line = line.to_string() + "."; // add empty to make sure to add number a the end of the line
        let mut number = Number::new();
        for (col_nbr, symbol) in new_line.chars().enumerate() {
            let l = line_nbr as isize;
            let c = col_nbr as isize;
            match symbol.to_digit(10) {
                Some(_) => number.add((l, c), symbol),
                None => {
                    if !number.is_empty() {
                        numbers.push(number);
                        number = Number::new();
                    }
                    if symbol != '.' {
                        symbols.insert((l, c));
                    }
                    if symbol == '*' {
                        gears.push((l, c));
                    }
                },
            }
        }
    }

    let sum = numbers.iter().map(|x| x.is_part(&symbols)).sum();
    let sum_second = gears.iter()
                          .map(|g| numbers.iter().filter(|x| x.is_adjacent(&g))
                          .collect::<Vec<&Number>>())
                          .filter(|x| x.len() == 2)
                          .map(|x| x.iter()
                                    .map(|x| x.get_num())
                                    .fold(1, |acc, x| acc * x))
                          .sum();

    return (sum, sum_second);
}