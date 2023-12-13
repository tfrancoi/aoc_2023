use itertools::Itertools;
use std::collections::HashSet;
use std::cmp::{min, max};
use aoc::{input, split_input, display};

fn get_empty_line(map: &Vec<Vec<char>>) -> HashSet<usize> {
    map.iter().enumerate().filter(|(_, line)| line.iter().all(|x| *x == '.')).map(|(i, _)| i).collect::<HashSet<usize>>()
}

fn get_empty_col(map: &Vec<Vec<char>>) -> HashSet<usize> {
    (0..map[1].len()).filter(|i| map.iter().all(|x| x[*i] == '.')).collect::<HashSet<usize>>()
}

fn extract_pos(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    for (l, line) in map.iter().enumerate() {
        for (c, symbol) in line.iter().enumerate() {
            if *symbol == '#' {
                pos.push((l, c));
            }
        } 
    }
    return pos;
}

fn manathan_distance(pos1: (usize, usize), pos2: (usize, usize),
                     empty_line: &HashSet<usize>, empty_col: &HashSet<usize>,
                     factor: isize) -> isize {
    let count_x = (min(pos1.0, pos2.0)..max(pos1.0, pos2.0)).filter(|i| empty_line.contains(i)).count() as isize;
    let count_y = (min(pos1.1, pos2.1)..max(pos1.1, pos2.1)).filter(|i| empty_col.contains(i)).count() as isize;
    (pos1.0 as isize - pos2.0 as isize).abs() + count_x * factor + (pos1.1 as isize - pos2.1 as isize).abs() + count_y * factor
}

fn aoc(input: &str) -> (isize, isize) {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in split_input(input) {
        let mut vline: Vec<char> = Vec::new();
        for symbol in line.chars() {
            vline.push(symbol);
        }
        map.push(vline);
    }
    let empty_line = get_empty_line(&map);
    let empty_col = get_empty_col(&map);
    let pos =  extract_pos(&map);

    (
        pos.iter().combinations(2).map(|x| manathan_distance(*x[0], *x[1], &empty_line, &empty_col, 1)).sum(),
        pos.iter().combinations(2).map(|x| manathan_distance(*x[0], *x[1], &empty_line, &empty_col, 999999)).sum()
    )
}

fn main() {
    display(aoc(&input(2023, 11)));
}
