use std::collections::HashMap;
use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 8)));
}

#[derive(Clone)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: String, right: String) -> Self{
        Self {
            left: left,
            right: right,
        }
    }

    fn get(&self, instruction: char) -> String {
        match instruction {
            'L' => self.left.to_string(),
            'R' => self.right.to_string(),
             _  => todo!(),
        }
    }
}

fn gcd(first: usize, second: usize) -> usize {
    let mut a = first;
    let mut b = second;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn gcd_iter(iters: &Vec<usize>) -> usize {
    if iters.len() == 0 {
        panic!("Cannot find gcd for 0 numbers");
    }
    if iters.len() == 1 {
        return iters[0];
    }
    let mut res = gcd(iters[0], iters[1]);
    for num in &iters[2..] {
        res = gcd(res, *num);
    }
    res
}

fn aoc(input: &str) -> (usize, usize) {
    let lines = split_input(input);
    let instruction = lines[0].chars().collect::<Vec<_>>();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in &lines[1..] {
        nodes.insert(
            String::from(&line[..3]),
            Node::new(
                String::from(&line[7..10]),
                String::from(&line[12..15])
            )
        );
    }
    let mut current_node = String::from("AAA");
    let mut step: usize = 0;
    while current_node != "ZZZ" {
        let i = instruction[step % instruction.len()];
        current_node = nodes.get(&current_node).unwrap().get(i);
        step += 1;
    }

    let current_nodes: Vec<String> = nodes.clone().into_keys().filter(|x| x.chars().nth(2).unwrap() == 'A').collect::<Vec<String>>();
    let mut res: Vec<usize> = Vec::new();
    for current_node_it in current_nodes {
        let mut step_second: usize = 0;
        let mut current_node = current_node_it.to_string();
        while !(current_node.chars().nth(2).unwrap() == 'Z') {
            let i = instruction[step_second % instruction.len()];
            current_node = nodes.get(&current_node).unwrap().get(i);
            step_second += 1;
        }
        res.push(step_second);
    }
    let gcd = gcd_iter(&res);
    (step, res.iter().fold(gcd, |res, cur| res / gcd * cur))
}