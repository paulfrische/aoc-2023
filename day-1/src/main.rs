fn part1(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().into_iter().for_each(|l| {
        let mut first = None;
        let mut last = None;
        l.chars().for_each(|c| {
            if c.is_digit(10) {
                if first == None {
                    first = Some(c);
                }
                last = Some(c);
            }
        });
        sum += str::parse::<i32>(&format!("{}{}", first.unwrap(), last.unwrap())).unwrap();
    });

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().into_iter().for_each(|mut l| {
        let mut first = None;
        let mut last = None;

        while l.len() > 0 {
            let mut digit = None;
            if l.starts_with("zero") || l.starts_with("0") {
            } else if l.starts_with("one") || l.starts_with("1") {
                digit = Some(1)
            } else if l.starts_with("two") || l.starts_with("2") {
                digit = Some(2)
            } else if l.starts_with("three") || l.starts_with("3") {
                digit = Some(3)
            } else if l.starts_with("four") || l.starts_with("4") {
                digit = Some(4)
            } else if l.starts_with("five") || l.starts_with("5") {
                digit = Some(5)
            } else if l.starts_with("six") || l.starts_with("6") {
                digit = Some(6)
            } else if l.starts_with("seven") || l.starts_with("7") {
                digit = Some(7)
            } else if l.starts_with("eight") || l.starts_with("8") {
                digit = Some(8)
            } else if l.starts_with("nine") || l.starts_with("9") {
                digit = Some(9)
            }

            if first == None {
                first = digit;
            }

            if let Some(d) = digit {
                last = Some(d);
            }

            (_, l) = l.split_at(1);
        }

        sum += str::parse::<i32>(&format!("{}{}", first.unwrap(), last.unwrap())).unwrap();
    });

    sum
}

fn main() {
    let input1 = include_str!("./input1.txt");
    let input2 = include_str!("./input1.txt");
    let example1 = include_str!("./example1.txt");
    let example2 = include_str!("./example2.txt");

    println!("example part1: {}", part1(example1));
    println!("solution part1: {}", part1(input1));

    println!("example part2: {}", part2(example2));
    println!("solution part2: {}", part2(input2));
}
