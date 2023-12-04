use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let (_, card) = l.split_once(':').unwrap();
            let (winning, own) = card.split_once('|').unwrap();
            let winning = winning.split_whitespace().collect_vec();
            own.split_whitespace()
                .map(|n| if winning.contains(&n) { 2 } else { 1 })
                .fold(1, |a, b| a * b)
                / 2
        })
        .fold(0, |a, b| a + b)
}

#[derive(Copy, Clone, Debug)]
struct Card {
    id: i32,
    winning_numbers: i32,
}

fn part2(input: &str) -> i32 {
    let mut card_table = HashMap::new();
    let mut card_stack = VecDeque::new();
    let mut card_count = 0;
    input.lines().for_each(|l| {
        let (card, numbers) = l.split_once(':').unwrap();
        let (_, card_id) = card.split_once(' ').unwrap();
        let (winning, own) = numbers.split_once('|').unwrap();
        let winning = winning.split_whitespace().collect_vec();
        let own = own.split_whitespace().collect_vec();
        let id = card_id.trim().parse().unwrap();
        let num_count = own
            .iter()
            .filter(|n| winning.contains(&n))
            .collect_vec()
            .len() as i32;
        card_table.insert(id, num_count);
        card_stack.push_back(Card {
            id,
            winning_numbers: num_count,
        });
    });

    while !card_stack.is_empty() {
        card_count += 1;
        let card = card_stack.pop_front().unwrap();
        for i in ((card.id + 1) as usize)..((card.id + card.winning_numbers + 1) as usize) {
            card_stack.push_back(Card {
                id: i as i32,
                winning_numbers: *card_table.get(&(i as i32)).unwrap(),
            });
        }
    }

    card_count
}

fn main() {
    let input1 = include_str!("./input1.txt");
    let input2 = include_str!("./input1.txt");
    let example1 = include_str!("./example1.txt");
    let example2 = include_str!("./example2.txt");

    println!("DAY-1");
    println!("example part1: {}", part1(example1));
    println!("solution part1: {}", part1(input1));
    println!("example part2: {}", part2(example2));
    println!("solution part2: {}", part2(input2));
}
