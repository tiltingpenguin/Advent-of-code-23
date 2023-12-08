use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, space0};
use nom::{IResult, Parser};
use nom_supreme::{multi::collect_separated_terminated, parser_ext::ParserExt};
use std::{collections::HashMap, fs};

fn main() {
    let path = "day8-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn process(input: String) -> i32 {
    let way = input.lines().nth(0).unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut node = "PTA";
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines = lines[2..].to_owned();
    let mut result = 0;

    for line in lines {
        let parts = parse_line(line);
        let node = parts[0];
        let left = parts[1];
        let right = parts[2];
        map.insert(node, (left, right));
    }
    for direction in way.chars().cycle() {
        if !node.ends_with("Z") {
            match direction {
                'L' => node = map[node].0,
                'R' => node = map[node].1,
                _ => panic!(),
            }
            result += 1;
        } else {
            return result;
        }
    }
    result
}

fn parse_line(line: &str) -> Vec<&str> {
    let mut parsed = collect_separated_terminated(
        alpha1.terminated(space0),
        tag("= (").or(tag(", ")),
        char(')'),
    );
    let values: IResult<&str, Vec<&str>> = parsed.parse(line);
    return values.unwrap().1;
}
