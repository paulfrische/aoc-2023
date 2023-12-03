#![allow(unused)]

use itertools::Itertools;

struct Parser<'a> {
    text: &'a str,
    current: Option<char>,
    pos: usize,
    numbers: Vec<(usize, usize)>,
    symbols: Vec<usize>,
    gears: Vec<usize>,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            current: text.chars().into_iter().next(),
            pos: 0,
            numbers: Vec::new(),
            symbols: Vec::new(),
            gears: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current = self.text.chars().collect_vec().get(self.pos).copied();
    }

    fn parse(&mut self) {
        while !self.current.is_none() {
            if let Some(c) = self.current {
                if c.is_ascii_digit() {
                    self.parse_number();
                } else if c != '.' && c != '\n' && c != '*' {
                    self.symbols.push(self.pos);
                    self.advance()
                } else if c == '*' {
                    self.symbols.push(self.pos);
                    self.gears.push(self.pos);
                    self.advance()
                } else {
                    self.advance()
                }
            }
        }
    }

    fn parse_number(&mut self) {
        let start = self.pos;
        while !self.current.is_none() && self.current.unwrap().is_ascii_digit() {
            self.advance();
        }
        let end = self.pos;
        self.numbers.push((start, end));
    }
}

#[derive(Debug)]
struct BluePrint<'a> {
    text: &'a str,
    width: usize,
    height: usize,
    numbers: Vec<(usize, usize)>,
    symbols: Vec<usize>,
    gears: Vec<usize>,
}

impl<'a> BluePrint<'a> {
    fn new(
        text: &'a str,
        numbers: Vec<(usize, usize)>,
        symbols: Vec<usize>,
        gears: Vec<usize>,
    ) -> Self {
        let width = text.lines().next().unwrap().len() + 1;
        let height = text.lines().collect_vec().len() + 1;

        Self {
            text,
            width,
            height,
            numbers,
            symbols,
            gears,
        }
    }

    fn gear_ratios(&self) -> Vec<i32> {
        let mut ratios = Vec::new();
        for g in self.gears.iter() {
            let mut numbers = Vec::new();
            let (gx, gy) = self.text_to_board(*g);
            'nums: for n in self.numbers.iter() {
                for di in (n.0)..(n.1) {
                    let (dx, dy) = self.text_to_board(di);
                    if dx.abs_diff(gx) <= 1 && dy.abs_diff(gy) <= 1 {
                        numbers.push(self.text.get((n.0)..(n.1)).unwrap().parse::<i32>().unwrap());
                        continue 'nums;
                    }
                }
            }

            if numbers.len() == 2 {
                ratios.push(numbers[0] * numbers[1]);
            }
        }

        ratios
    }

    fn numbers_with_symbol(&self) -> Vec<i32> {
        let is_symbol = |ch: Option<char>| {
            if let Some(c) = ch {
                return !c.is_ascii_digit() && c != '.' && c != '\n';
            } else {
                return false;
            }
        };

        let mut numbers = Vec::new();
        'num: for number in self.numbers.iter() {
            for digit_index in (number.0)..(number.1) {
                let (x, y) = self.text_to_board(digit_index);

                let mut sx = x;
                if x > 0 {
                    sx = x - 1;
                }
                let mut sy = y;
                if y > 0 {
                    sy = y - 1;
                }

                for cx in sx..x + 2 {
                    for cy in sy..y + 2 {
                        let c = self.text.chars().nth(self.board_to_text((cx, cy)));
                        if is_symbol(c) {
                            numbers.push(number);
                            continue 'num;
                        }
                    }
                }
            }
        }

        let numbers = numbers
            .iter()
            .map(|num| {
                self.text
                    .get((num.0)..(num.1))
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            })
            .collect_vec();

        numbers
    }

    fn text_to_board(&self, pos: usize) -> (usize, usize) {
        let y = pos / self.width;
        let x = pos % self.width;
        (x, y)
    }

    fn board_to_text(&self, pos: (usize, usize)) -> usize {
        self.width * pos.1 + pos.0
    }
}

fn part1(input: &str) -> i32 {
    let mut parser = Parser::new(input);
    parser.parse();
    let blueprint = BluePrint::new(parser.text, parser.numbers, parser.symbols, parser.gears);
    let nums = blueprint.numbers_with_symbol();
    let v = nums.iter().fold(0, |a, v| a + v);
    v
}

fn part2(input: &str) -> i32 {
    let mut parser = Parser::new(input);
    parser.parse();
    let blueprint = BluePrint::new(parser.text, parser.numbers, parser.symbols, parser.gears);
    let nums = blueprint.gear_ratios();
    let v = nums.iter().fold(0, |a, v| a + v);
    v
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
