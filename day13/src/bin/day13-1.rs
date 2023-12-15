use std::fs;
use std::cmp::min;

fn main() {
    let path = "day13-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn transpose(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = map[0].len();
    let mut iters: Vec<_> = map.iter().map(|n| n.iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn find_mirror(block: &Vec<Vec<char>>) -> Option<usize> {
    let mut res = 0;
    let mut ind = 0;
    for (index, line) in block[..block.len()-1].iter().enumerate() {
        //println!("{:?}", line);
        //println!("{:?}", block[index+1]);
        if line == &block[index+1] {
            ind = index+1;
            res = index;
            break;
        }
    }
    if res == 0 || res == block.len() {return Some(ind);}

    let mut offset = 1;
    while offset <= res && res + offset + 1< block.len() {
        //if res < offset {break;}
        let left = res - offset;
        let right = res + offset +1;
        //if right == block.len() {break;}
        if block[left] != block[right] {
            println!("{:?}", block[left]);
            println!("{:?}", block[right]);
            ind = 0;
            break;
        }
        offset += 1;
    }
    if ind != 0 { return Some(ind); } else { return None; }
}

fn process(input: String) -> usize {
    let mut result = 0;
    let arr: Vec<Vec<Vec<char>>> = input.split("\n\n").collect::<Vec<&str>>().iter().map(|&b| b.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()).collect();
    for block in arr {
        //println!("Starting block");
        let mut mirror = find_mirror(&block);
        match mirror {
            Some(mirror) => result += mirror * 100,
            None => {
                mirror = find_mirror(&transpose(&block));
                match mirror {
                    Some(mirror) => result += mirror,
                    None =>  {println!("hi");}//{println!("{:?}", transpose(&block)); unreachable!("Mirror line should exist")}
                }
            }
        }
    }
    result += 1001;
    return result;
}