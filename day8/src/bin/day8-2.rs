use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, space0};
use nom::{IResult, Parser};
use nom_supreme::{multi::collect_separated_terminated, parser_ext::ParserExt};
use std::{collections::HashMap, fs};

fn main() {
    let path = "day8-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{:?}", result);
}

fn process(input: String) -> i64 {
    let way = input.lines().next().unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines = lines[2..].to_owned();
    let mut paths = vec![];

    for line in lines {
        let parts = parse_line(line);
        let node = parts[0];
        let left = parts[1];
        let right = parts[2];
        map.insert(node, (left, right));
    }

    let mut nodes = map
        .keys()
        .filter(|c| c.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();
    for node in nodes.iter_mut() {
        let mut count = 0;
        for direction in way.chars().cycle() {
            if !node.ends_with('Z') {
                match direction {
                    'L' => *node = map[node].0,
                    'R' => *node = map[node].1,
                    _ => panic!(),
                }
                count += 1;
            } else {
                paths.push(count);
                break;
            }
        }
    }
    let mut last = lcm(paths[0], paths[1]);
    for path in paths {
        last = lcm(last, path);
    }
    last
}

fn parse_line(line: &str) -> Vec<&str> {
    let mut parsed = collect_separated_terminated(
        alphanumeric1.terminated(space0),
        tag("= (").or(tag(", ")),
        char(')'),
    );
    let values: IResult<&str, Vec<&str>> = parsed.parse(line);
    return values.unwrap().1;
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * (b / gcd(a, b));
}
