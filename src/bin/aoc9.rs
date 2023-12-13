use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 9)));
}

fn resolve_number(lasts: Vec<isize>, back: isize) -> isize {
    lasts.iter().rev().fold(0, |diff, cur| cur + diff * back)
}

fn get_next_number(suite: Vec<isize>) -> (isize, isize) {
    let mut last_number: Vec<isize> = Vec::new();
    let mut first_number: Vec<isize> = Vec::new();
    let mut new_suite: Vec<isize> = suite.clone();
    last_number.push(*new_suite.last().unwrap());
    first_number.push(*new_suite.first().unwrap());
    while !new_suite.iter().all(|x| *x == 0) {
        new_suite = new_suite.windows(2).map(|x| x[1] - x[0]).collect();
        last_number.push(*new_suite.last().unwrap());
        first_number.push(*new_suite.first().unwrap());
    }
    (resolve_number(last_number, 1), resolve_number(first_number, -1))
}

fn aoc(input: &str) -> (isize, isize) {
    let mut res = (0, 0);
    for line in split_input(input) {
        let suite = line.split(" ").map(|x| x.trim().parse::<isize>().unwrap()).collect::<Vec<_>>();
        let r = get_next_number(suite);
        res = (res.0 + r.0, res.1 + r.1);
    }
    res
}