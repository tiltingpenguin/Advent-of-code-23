// Brute force solution, might refactor later
// solution for my input is: 10834440

use std::collections::BTreeMap;
use std::fs;

fn main() {
    let path = "day5-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> i64 {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut name_order: Vec<&str> = Vec::new();
    let mut map: BTreeMap<String, Vec<Vec<i64>>> = BTreeMap::new();
    let seed_ranges: Vec<i64> = blocks[0]
        .to_string()
        .split(":")
        .nth(1)
        .unwrap()
        .to_string()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let iter = seed_ranges.chunks(2);
    let mut seeds: Vec<i64> = vec![];
    for group in iter {
        seeds.append(&mut (group[0]..group[0] + group[1]).collect::<Vec<i64>>())
    }
    for block in blocks {
        let parts: Vec<&str> = block.split(":").collect();
        name_order.push(parts[0]);
        let mut ivalues: Vec<Vec<i64>> = Vec::new();
        let mut values = parts[1]
            .to_string()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for val in values[1..].iter_mut() {
            let new_vec = val
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            ivalues.push(new_vec);
        }
        map.insert(parts[0].to_string(), ivalues);
    }
    for seed in seeds.iter_mut() {
        for ranges in name_order[1..].iter() {
            for tup in map[*ranges].iter() {
                if tup.is_empty() {
                    continue;
                }
                let dest = tup[0];
                let source = tup[1];
                let len = tup[2];

                if (source..source + len).contains(seed) {
                    *seed += dest - source;
                    break;
                }
            }
        }
    }
    *seeds.iter().min().unwrap()
}
