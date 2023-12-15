use std::fs;

fn main() {
    let path = "day4-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> i32 {
    let mut result = 0;
    let lines = input
        .lines()
        .map(|x| format!("{}", x.to_string()))
        .collect::<Vec<String>>();
    for line in lines {
        let game = line.split(":").nth(1).unwrap().to_string();
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
        if matches > 0 {
            result += i32::pow(2, matches - 1);
        }
    }
    result
}
