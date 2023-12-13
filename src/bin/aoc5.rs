use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use aoc::{input, display};

fn main() {
    display(aoc(&input(2023, 5)));
}

#[derive(Debug,PartialEq,Clone)]
struct Mapping {
    mapping: Vec<(isize, isize, isize)>,
}

impl Mapping {
    fn new() -> Mapping {
        let mapping: Vec<(isize, isize, isize)> = Vec::new();
        Mapping {
            mapping: mapping,
        }
    }

    fn add(&mut self, input: &str) {
        let range = input.split(" ").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
        self.mapping.push((range[1], range[1] + range[2] - 1, range[0] - range[1]));
    }

    fn get(&self, key: isize) -> isize{
        for (begin, end, offset) in self.mapping.iter() {
            if key >= *begin && key <= *end {
                return key + offset;
            }
        }
        return key;
    }
}

fn find_min(seeds: Vec<isize>, mappings: Arc<Vec<Mapping>>) -> isize {
    let mut res: Vec<isize> = Vec::new();
    for seed in seeds {
        let mut loc = seed;
        for map in mappings.iter() {
            loc = map.get(loc)
        }
        res.push(loc);
    }
    return *res.iter().min().unwrap();
}

fn aoc(input: &str) -> (isize, isize) {
    let mut lines = input.split("\n\n").map(|x| x.trim()).collect::<Vec<_>>();
    let mut mappings: Vec<Mapping> = Vec::new();
    let seeds = lines[0].split(":").collect::<Vec<_>>()[1].trim().split(" ").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    lines.remove(0);
    for line in lines {
        let mut mapping = Mapping::new();
        let maps = line.split("\n").filter(|x| !x.contains(":")).collect::<Vec<_>>();
        for range in maps {
            mapping.add(range);
        }
        mappings.push(mapping);
    }

    let new_mapping = Arc::new(mappings.clone());
    let mut handles: Vec<JoinHandle<_>> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let mut new_seeds: Vec<isize> = Vec::new();
        for (c, j) in (seeds[i]..(seeds[i] + seeds[i+1])).enumerate() {
            new_seeds.push(j);
            if c % 5000000 == 0 || j == (seeds[i] + seeds[i+1] -1) {
                let new_mapping = Arc::clone(&new_mapping);
                handles.push(thread::spawn(move || find_min(new_seeds, new_mapping)));
                new_seeds = Vec::new();
            }
        }
        println!("{} threads", handles.len());
    }

    return (find_min(seeds, Arc::new(mappings.clone())), handles.into_iter().map(|x| x.join().unwrap()).min().unwrap());
}