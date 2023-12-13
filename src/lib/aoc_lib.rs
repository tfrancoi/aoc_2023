use ureq;
use std::fs::File;
use std::io::Read;


fn get_input(year: usize, day: usize) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
                    .set("COOKIE", "session=YOUR_SESSION_COOKIE")
                    .call()?
                    .into_string()?;
    
    Ok(body)
}

pub fn input(year: usize, day: usize) -> String {
    get_input(year, day).unwrap()
}

pub fn split_input(input: &str) -> Vec<String> {
    input.split("\n").map(|x| x.trim()).filter(|x| *x != "").map(|x| x.to_string()).collect()
}

pub fn test_input(file: &str) -> String {
    let mut file = File::open(file).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    contents.to_string()
}

pub fn display<T: std::fmt::Display>(result: (T, T)) {
    println!("Part 1: {}\nPart 2: {}", result.0, result.1);
}