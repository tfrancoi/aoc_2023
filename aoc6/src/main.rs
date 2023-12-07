fn aoc(input: Vec<(isize, isize)>) -> i64 {
    let mut res:i64 = 1;
    let mut res2:i64 = 1;
    for (time, distance) in input {
        let ftime = time as f64;
        let fdistance = distance as f64;
        let mut sol1 = (ftime - f64::sqrt((ftime * ftime) - (4.0 * fdistance))) / 2.0;
        let mut sol2 = (ftime + f64::sqrt((ftime * ftime) - (4.0 * fdistance))) / 2.0;
        res *= (sol2.ceil() - (sol1 + 1.0).floor()) as i64;
        res2 *= (0..=time).map(|t| t * (time - t)).filter(|d| *d > distance).count() as i64;
    }
    println!("Res 2 {res2}");
    res
}


fn main() {
    let input = vec![
        (7, 9),
        (15, 40),
        (30, 200),
    ];
    let input = vec![
        (44, 208),
        (80, 1581),
        (65, 1050),
        (72, 1102),
    ];
    let input2 = vec![(44806572, 208158110501102)];
    println!("{}    {}", aoc(input), aoc(input2));
}
