use std::collections::HashSet;
use std::collections::HashMap;
use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 10)));
}

fn replace_start(pos: (isize, isize), map: &HashMap<(isize, isize), char>) -> char {
    let mut answer: HashSet<char> = HashSet::from_iter(vec!['|', '-', '7', 'F', 'L', 'J']);
    if map.contains_key(&(pos.0 + 1, pos.1)) && vec!['|', 'L', 'J'].contains(map.get(&(pos.0 + 1, pos.1)).unwrap()) {
        answer = answer.intersection(&HashSet::from_iter(vec!['|', '7', 'F']))
                       .map(|i| *i).collect();
    }
    if map.contains_key(&(pos.0, pos.1 + 1)) && vec!['-', '7', 'J'].contains(map.get(&(pos.0, pos.1 + 1)).unwrap()) {
        answer = answer.intersection(&HashSet::from_iter(vec!['-', 'F', 'L']))
                       .map(|i| *i).collect();
    }
    if map.contains_key(&(pos.0 - 1, pos.1)) && vec!['|', '7', 'F'].contains(map.get(&(pos.0 - 1, pos.1)).unwrap()) {
        answer = answer.intersection(&HashSet::from_iter(vec!['|', 'L', 'J']))
                       .map(|i| *i).collect();
    }
    if map.contains_key(&(pos.0, pos.1 - 1)) && vec!['-', 'L', 'F'].contains(map.get(&(pos.0, pos.1 - 1)).unwrap()) {
        answer = answer.intersection(&HashSet::from_iter(vec!['-', '7', 'J']))
                       .map(|i| *i).collect();
    }
    *answer.iter().next().unwrap()
}

fn next_moves(pos: (isize, isize), map: &HashMap<(isize, isize), char>, visited: &HashSet<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut next: Vec<(isize, isize)> = Vec::new();
    match map.get(&pos).unwrap() {
        '|' => next.extend_from_slice(&[(pos.0 + 1, pos.1), (pos.0 - 1, pos.1)]),
        '-' => next.extend_from_slice(&[(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)]),
        'L' => next.extend_from_slice(&[(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)]),
        'J' => next.extend_from_slice(&[(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)]),
        '7' => next.extend_from_slice(&[(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)]),
        'F' => next.extend_from_slice(&[(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)]),
        _ => todo!(),
    }
    return next.into_iter().filter(|x| !visited.contains(x)).collect();
}

fn aoc(input: &str) -> (isize, isize) {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut todo: Vec<(isize, isize)> = Vec::new();
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut start = (-1, -1);
    for (line_nbr, line) in split_input(input).iter().enumerate() {
        for (col_nbr, symbol) in line.chars().enumerate() {
            if symbol == 'S' {
                start = (line_nbr as isize, col_nbr as isize);
                todo.push(start);
            }
            if symbol != '.' {
                map.insert((line_nbr as isize, col_nbr as isize), symbol);
            }
        }
    }

    map.insert(start, replace_start(start, &map));
    
    let mut step = 0;
    while todo.len() > 0 {
        visited.extend::<&HashSet<(isize, isize)>>(&HashSet::from_iter(todo.clone()));
        todo = todo.iter().map(|x| next_moves(*x, &map, &visited)).flatten().collect();
        step += 1;
    }
    let boundaries = (
        map.keys().map(|x| x.0).max().unwrap(),
        map.keys().map(|x| x.1).max().unwrap(),
    );
    let mut contains = 0;
    for l in 0..=boundaries.0 {
        let mut inside = false;
        for c in 0..=boundaries.1 {
            let pos = (l, c);
            if visited.contains(&pos) {
                if vec!['|', '7', 'F'].contains(map.get(&pos).unwrap()) {
                    inside = !inside;
                }
                print!("{}", map.get(&pos).unwrap())
            }
            else if inside {
                contains += 1;
                print!("\x1b[1;34mI\x1b[0;37m");
            }
            else {
                print!("\x1b[0;31mO\x1b[0;37m");
            }
        }
        print!("\n");
    }
    return (step - 1, contains);
}