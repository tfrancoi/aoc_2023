use std::fs::File;
use std::io::Read;

#[derive(Debug,PartialEq)]
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

fn find_min(seeds: Vec<isize>, mappings: &Vec<Mapping>) -> isize {
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
    let mut min_second = std::u32::MAX as isize;

    for i in (0..seeds.len()).step_by(2) {
        println!("Start seed");
        let mut new_seeds: Vec<isize> = Vec::new();
        for j in seeds[i]..(seeds[i] + seeds[i+1]) {
            new_seeds.push(j);
        }
        let new_min = find_min(new_seeds, &mappings);
        if new_min < min_second {
            min_second = new_min;
        }
    }

    return (find_min(seeds, &mappings), min_second);
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    let result = aoc(&contents);
    println!("{}    {}", result.0, result.1);
}


//TODO split range 
// process range per range