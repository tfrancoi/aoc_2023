use std::collections::HashMap;
use itertools::Itertools;
use aoc::{input, split_input, display, test_input};

fn main() {
    display(aoc(&aoc::test_input("input_12.txt")));
}

fn generate_solution_rec(group: usize, solution: Vec<char>, remaining_size: isize, g_num: usize) -> Vec<(Vec<char>, isize)> {
    let mut res:Vec<(Vec<char>, isize)> = Vec::new();
    println!("{g_num}");
    for offset in 0..remaining_size+1 {
        let mut s = solution.clone();
        let r = remaining_size - offset;
        if g_num > 0 {
            s.push('.');
        }
        for _ in 0..offset {
            s.push('.');
        }
        for _ in 0..group {
            s.push('#');
        }
        println!("{}", s.iter().collect::<String>());

        res.push((s, r));
    }
    res
}

fn filter_solution_partial(constrains: &str, solutions: Vec<(Vec<char>, isize)>) -> Vec<(Vec<char>, isize)> {
    return solutions;
    // let c = constrains.chars().collect::<Vec<_>>();
    // solutions.into_iter().filter(|s| (0..s.0.len()).all(|i| c[i] == '?' || c[i] == s.0[i])).collect()
}

fn generate_solution(size: usize, groups: &Vec<usize>, constrains: &str) -> Vec<Vec<char>> {
    let groups_size = groups.iter().sum::<usize>() + groups.len() - 1;
    let remaining_size = size - groups_size;
    let mut res:Vec<(Vec<char>, isize)> = vec![(vec![], remaining_size as isize)];
    let mut step = 0;
    for (g_num, g_size) in groups.iter().enumerate() {
        let mut new_res: Vec<(Vec<char>, isize)> = Vec::new();
        for r in res.iter() {
            step += 1;
            let so = generate_solution_rec(*g_size, r.0.clone(), r.1, g_num);
            new_res.extend_from_slice(&filter_solution_partial(constrains, so));
        }
        res = new_res;
    }
    // println!("STEP {step}");
    res.into_iter().map(|x| x.0).collect()
}

fn filter_solution(solutions: Vec<Vec<char>>, constrains: &str) -> usize {
    let mut res = 0;
    let l = constrains.len();
    let c = constrains.chars().collect::<Vec<_>>();
    for solution in solutions {
        let mut s = solution.clone(); 
        for _ in solution.len()..l {
            s.push('.');
        }
        if (0..l).all(|i| c[i] == '?' || c[i] == s[i]) {
            res += 1;
            println!("{}", s.iter().collect::<String>());
            println!("{}\n", constrains);
            return res;
        }
    }
    println!("Res {res}");
    res
}

fn aoc(input: &str) -> (usize, usize) {
    let mut total = 0;
    let mut total2 = 0;
    for line in aoc::split_input(input) {
        println!("{line}");
        let data = line.split(" ").collect::<Vec<_>>();
        let group_str = (0..5).map(|x| data[1].to_string()).collect::<Vec<String>>().join(",");
        let springs = data[0];
        let springs2 = (0..5).map(|x| springs.to_string()).collect::<Vec<String>>().join("?");
        // println!("{springs2}");
        // println!("{group_str}");
        let groups = data[1].split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let groups2 = group_str.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let solutions = generate_solution(springs.len(), &groups, springs);
        // let solutions2 = generate_solution(springs2.len(), &groups2, &springs2);
        println!("---- Solutions --- ");
        total += filter_solution(solutions, springs);
        // total2 += filter_solution(solutions2, &springs2);
        
    }

    (total, total2)
}


fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>, // Cache ??
    s: &[u8], //string remaining
    within: Option<usize>, 
    remaining: &[usize]
) -> usize {
    println!("{:?}", cache);
    println!("{:?}", s);
    println!("{:?}", within);
    println!("{:?}\n", remaining);
    if s.is_empty() {
        return match (within, remaining.len()) {
        (None, 0) => 1,
        (Some(x), 1) if x == remaining[0] => 1,
        _ => 0
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        (b'.', Some(x)) if x != remaining[0] => 0,
        (b'.', Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
        (b'.', None)    => possible_ways(cache, &s[1..], None, remaining),
        (b'#', Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x+1), remaining),
        (b'#', None)    => possible_ways(cache, &s[1..], Some(1), remaining),
        (b'?', Some(x)) => {
        let mut ans = possible_ways(cache, &s[1..], within.map(|x| x+1), remaining);
        if x == remaining[0] {
            ans += possible_ways(cache, &s[1..], None, &remaining[1..])
        }
        ans
        }
        (b'?', None) =>
        possible_ways(cache, &s[1..], Some(1), remaining) +
        possible_ways(cache, &s[1..], None, remaining),
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}


fn aocalt(input: &str) -> (usize, usize) {
    let mut cache = HashMap::new();
    input.split('\n').map(|l| {
        let (vents, rest) = l.split_once(' ').unwrap();
        let nums = rest.split(',').map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let new_vents = (0..5).map(|_| vents).join("?");
        let new_nums = (0..5).flat_map(|_| &nums).copied().collect::<Vec<_>>();
        cache.clear();
        println!("{vents}");
        let p1 = possible_ways(&mut cache, vents.as_bytes(), None, &nums);
        cache.clear();
        // let p2 = possible_ways(&mut cache, new_vents.as_bytes(), None, &new_nums);
        (p1,0)
    }).fold((0,0), |(p1,p2), (a,b)| (p1+a, p2+b))
}