use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 2)));
}

fn max_color(game: &str, color: &str) -> u32 {
    game.split(";").map(
        |x| x.trim().split(",")
             .filter(|x| x.contains(color))
             .map(|x| x.replace(color, "")
                       .trim().parse::<u32>().unwrap()
            ).collect::<Vec<_>>()
    ).filter(|x| x.len() == 1).map(|x| x[0])
     .max().unwrap()
}

fn aoc(input: &str) -> (u32, u32) {
    let mut sum: u32 = 0;
    let mut sum_second: u32 = 0;
    for line in split_input(input) {
        let game = line.split(':').collect::<Vec<_>>();
        let id: u32 = game[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let red = max_color(game[1], "red"); //12
        let green = max_color(game[1], "green"); //13
        let blue = max_color(game[1], "blue"); //14
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += id;
        }
        sum_second += red * green * blue; 
    }
    return (sum, sum_second);
}


