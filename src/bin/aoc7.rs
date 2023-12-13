use std::cmp::Ordering;
use itertools::Itertools;
use aoc::{input, split_input, display};

fn main() {
    display(aoc(&input(2023, 7)));
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    joker: bool,
}

impl Hand {
    fn new(line: &str, joker: bool) -> Self {
        let input = line.trim().split(" ").collect::<Vec<_>>();
        Self {
            cards: input[0].chars().map(|c| {
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => if joker { 1 } else { 11 },
                    'T' => 10,
                     d  => d.to_digit(10).unwrap() as usize,

                }
            }).collect::<Vec<usize>>(),
            bid: input[1].parse().unwrap(),
            joker: joker,
        }
    }

    fn max_group(&self) -> (usize, usize) {
        let joker_nb = self.cards.iter().filter(|x| **x == 1 && self.joker).count();
        let mut size = self.cards.iter()
                                 .filter(|x| **x != 1 || !self.joker)
                                 .sorted()
                                 .group_by(|c| *c).into_iter()
                                 .map(|(_, g)| g.count())
                                 .sorted().collect::<Vec<_>>();
        if size.len() == 0 {
            return (5, 0);
        }
        if size.len() == 1 {
            return (size[0] + joker_nb, 0);
        }
        return (size.pop().unwrap() + joker_nb, size.pop().unwrap());
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.max_group().cmp(&other.max_group()) != Ordering::Equal {
            return self.max_group().cmp(&other.max_group());
        }
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a.cmp(&b) != Ordering::Equal {
                return a.cmp(&b);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn aoc(input: &str) -> (usize, usize) {
    let mut hands: Vec<Hand> = Vec::new();
    let mut hands_joker: Vec<Hand> = Vec::new();
    for line in split_input(input) {
        hands.push(Hand::new(&line, false));
        hands_joker.push(Hand::new(&line, true));
    }
    (
        hands.iter().sorted().enumerate().map(|(i, hand)| (i + 1) * hand.bid).sum(),
        hands_joker.iter().sorted().enumerate().map(|(i, hand)| (i + 1) * hand.bid).sum()
    )
}