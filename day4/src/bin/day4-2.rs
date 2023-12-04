use std::fs;

fn main() {
    let path = "day4-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> i32 {
    let mut result = 0;
    let lines = input.lines().map(|x| format!("{}", x.to_string())).collect::<Vec<String>>();
    let size = lines.len();
    let mut winnings: Vec<i32> = Vec::with_capacity(size);
    for _ in 0..size {
        winnings.push(1);
    }
    for line in lines {
        let game = line.split(":").nth(1).unwrap().to_string();
        let card = line.split(":").nth(0).unwrap().to_string().split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        let winners = game.split(" | ").nth(0).unwrap().to_string();
        let actuals = game.split(" | ").nth(1).unwrap().to_string();
        let winners_vec = winners.split_whitespace().collect::<Vec<&str>>();
        let actuals_vec = actuals.split_whitespace().collect::<Vec<&str>>();
        
        let mut matches = 0;
        for num in winners_vec {
            if actuals_vec.iter().any(|e| &num == e) {
                matches += 1;
            }
        }
        let cur = winnings[card-1];
        for elem in winnings[card..card+matches].iter_mut() {
            *elem += cur;
        }
    }
    result = winnings.iter().sum();
    result
}