use std::fs;
use nom::multi::separated_list0;
use nom::bytes::complete::{tag, take_till};
use nom::IResult;

fn main() {
    let path = "day15-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn parse(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(","), take_till(|c| c == ','))(s)
}

fn process(input: String) -> usize {
    let input_list = parse(&input).unwrap().1;
    let mut sum: usize = 0;
    for value in input_list {
        let mut result = 0;
        for c in value.chars() {
            result += c as u32;
            result = result * 17 % 256;
        }
        sum += result as usize;
    }
    return sum;
}