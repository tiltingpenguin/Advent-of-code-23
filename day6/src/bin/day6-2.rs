use std::fs;

fn main() {
    let path = "day6-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn plot(time: u64, target_dist: u64) -> u64 {
    let mut dists: u64 = 0;
    for hold_time in 0..time {
        let dist = (time - hold_time) * hold_time;
        if dist > target_dist {
            dists += 1;
        }
    }
    dists
}

pub fn process(input: String) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut time: String = lines[0]
                            .split(":").nth(1).unwrap()
                            .to_string();
    let mut dist: String = lines[1]
                            .split(":").nth(1).unwrap()
                            .to_string();
                            //.retain(|s| s != ' ');
                            //.parse::<u64>;
    time.retain(|c| c != ' ');
    dist.retain(|c| c != ' ');
    let time_num = time.parse::<u64>().unwrap();
    let dist_num = dist.parse::<u64>().unwrap();
    dbg!(time_num);
    dbg!(dist_num);

    return plot(time_num, dist_num);
}