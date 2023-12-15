use nom::character::complete::{i32, space1, newline};
use nom::{IResult, Parser};
use nom_supreme::multi::collect_separated_terminated;
use std::fs;

fn main() {
    let path = "day9-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{:?}", result);
}

fn parser(line: &str) -> Vec<i32> {
    let mut parser = collect_separated_terminated(i32, space1, newline);
    let parsed: IResult<&str, Vec<i32>> = parser.parse(line);
    parsed.unwrap().1
}

fn process(input: String) -> i32 {
    let mut result = 0;
    let lines = input.lines().map(|x| format!("{}\n", x)).collect::<Vec<String>>();
    let mut input_arrs: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let nums = parser(&line);
        input_arrs.push(nums);
    }

    for nums in input_arrs.iter_mut() {
        *nums = nums.iter().rev().copied().collect();
        let mut tmp: Vec<i32> = nums.windows(2).map(|x| x.to_vec()).map(|x| x[1] - x[0]).collect();
        let mut last_vals: Vec<i32> = vec![nums[nums.len()-1]];
        while !tmp.iter().all(|&x| x == 0) {
            last_vals.push(tmp[tmp.len()-1]);
            tmp = tmp.windows(2).map(|x| x.to_vec()).map(|x| x[1] - x[0]).collect();
        }
        dbg!(&last_vals);
        result += last_vals.iter().sum::<i32>();
    }
    return result;
}