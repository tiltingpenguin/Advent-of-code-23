use std::fs;

fn main() {
    let path = "day6-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn plot(time: u32, target_dist: u32) -> u32 {
    let mut dists: u32 = 0;
    for hold_time in 0..time {
        let dist = (time - hold_time) * hold_time;
        if dist > target_dist {
            dists += 1;
        }
    }
    return dists;
}

pub fn process(input: String) -> u32 {
    let mut result = 1;
    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<u32> = lines[0]
                            .split(":").nth(1).unwrap()
                            .split_whitespace()
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect();
    let dist: Vec<u32> = lines[1]
                            .split(":").nth(1).unwrap()
                            .split_whitespace()
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect();
    for race in 0..times.len() {
        let win = plot(times[race], dist[race]);
        if win > 0 {
            result *= win;
        }
    }
    result
}