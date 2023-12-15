use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32, space1};
use nom::multi::{many_till, separated_list0};
use nom::IResult;
use std::fs;

fn main() {
    let path = "day12-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn parse(line: &str) -> IResult<&str, (Vec<char>, &str)> {
    many_till(anychar, space1)(line)
}

fn parse_counts(nums: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), i32)(nums)
}

fn recursion(springs: &[char], counts: &Vec<i32>, mut result: usize) -> usize {
    if springs.is_empty() {
        if counts.is_empty() {
            result += 1;
            return result;
        } else {
            return result;
        }
    }
    if counts.is_empty() && springs.contains(&'#') {
        return result;
    }
    match springs[0] {
        '.' => {
            result = recursion(&springs[1..], counts, result);
            return result;
        }
        '#' => {
            let grouplen = counts[0] as usize;
            if grouplen > springs.len() {
                return result;
            }
            let group = springs.get(..grouplen).unwrap();
            if group.contains(&'.') {
                return result;
            }
            if grouplen <= springs.len() - 1 && springs[grouplen] == '#' {
                return result;
            }
            if grouplen == springs.len() {
                if counts.len() == 1 {
                    result += 1;
                }
                return result;
            }
            result = recursion(&springs[grouplen + 1..], &counts[1..].to_vec(), result);
            return result;
        }
        '?' => {
            result = recursion(&springs[1..], counts, result);
            result = recursion(&[&['#'], &springs[1..]].concat(), counts, result);
            return result;
        }
        _ => unreachable!("character needs to be ., # or ?"),
    }
}

fn process(input: String) -> usize {
    let mut result: usize = 0;
    for line in input.lines() {
        let parsed = parse(line).unwrap();
        let counts = parse_counts(parsed.0).unwrap().1;
        let springs = parsed.1 .0;

        result += recursion(&springs, &counts, 0);
    }
    return result;
}
