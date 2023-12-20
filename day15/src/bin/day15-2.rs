use std::{fs, collections::HashMap};
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
    let mut hash_list: Vec<usize> = vec![];
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    for value in input_list {
        let mut result = 0;
        for c in value.chars() {
            result += c as u32;
            result = result * 17 % 256;
        }
        //hash_list.push(result as usize);
        if value.contains('=') {
            let focal = value.split("=").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
            if boxes.contains_key(&result) {
                boxes[&result].push((value, focal))
            }
            boxes.insert(result, vec![(value, focal)]);
        }
        if value.contains('-') {
            let pos = boxes[&result].iter().position(|&x| x.0 == value);
            match pos {
                Some(pos) => _ = boxes[&result].remove(pos),
                None => continue,
            }
        }
    }
    return 0;
}