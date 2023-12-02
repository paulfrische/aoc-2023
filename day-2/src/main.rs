use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take},
        streaming::take_until,
    },
    character::complete::char,
    multi::{many0, many_till},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug, Copy, Clone)]
struct Set {
    r: i32,
    g: i32,
    b: i32,
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    let (r, id) = parse_id(line).unwrap();
    let (r, _) = take::<usize, &str, nom::error::Error<&str>>(2usize)(r).unwrap();
    let (_, s) = many0(parse_set)(r).unwrap();

    Ok(("", Game { id, sets: s }))
}

fn parse_id(input: &str) -> IResult<&str, i32> {
    let (r, _) = take(5usize)(input)?;
    let (r, id) = take_until(":")(r)?;
    Ok((r, id.parse().unwrap()))
}

fn parse_color(input: &str) -> IResult<&str, (&str, i32)> {
    let mut color_name = alt((tag("red"), tag("green"), tag("blue")));
    let count = take_until(" ");

    let (r, _) = many0(char(' '))(input)?;
    let (r, n) = count(r)?;
    let (r, _) = take(1usize)(r)?;
    let (r, c) = color_name(r)?;

    let n = n.parse().unwrap();

    if r.len() > 0 && r.chars().next().unwrap() == ',' {
        let (r, _) = take(2usize)(r)?;
        return Ok((r, (c, n)));
    }

    Ok((r, (c, n)))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let mut set = Set { r: 0, g: 0, b: 0 };

    let (r, (sets, _)) = many_till(parse_color, tag(";"))(input).map_err(|_| {
        nom::Err::Error(nom::error::Error {
            input: "",
            code: nom::error::ErrorKind::Eof,
        })
    })?;
    for s in sets.iter() {
        match s.0 {
            "red" => set.r = s.1,
            "green" => set.g = s.1,
            "blue" => set.b = s.1,
            _ => unimplemented!("unknown color"),
        }
    }

    Ok((r, set))
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().for_each(|l| {
        // dbg!(l);
        let (_, g) = parse_game(&format!("{l};")).unwrap();
        // dbg!(&g);
        sum += g.id;
        for s in g.sets.iter() {
            if s.r > 12 || s.g > 13 || s.b > 14 {
                sum -= g.id;
                break;
            }
        }
    });
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().for_each(|l| {
        let (_, g) = parse_game(&format!("{l};")).unwrap();
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        for s in g.sets.iter() {
            if s.r > min_r {
                min_r = s.r;
            }
            if s.g > min_g {
                min_g = s.g;
            }
            if s.b > min_b {
                min_b = s.b;
            }
        }
        sum += min_r * min_g * min_b;
    });
    sum
}

fn main() {
    let input1 = include_str!("./input1.txt");
    let input2 = include_str!("./input1.txt");
    let example1 = include_str!("./example1.txt");
    let example2 = include_str!("./example2.txt");

    println!("DAY-2");
    println!("example part1: {}", part1(example1));
    println!("solution part1: {}", part1(input1));
    println!("example part2: {}", part2(example2));
    println!("solution part2: {}", part2(input2));
}
