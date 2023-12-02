fn part1(input: &str) -> i32 {}

fn part2(input: &str) -> i32 {}

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
