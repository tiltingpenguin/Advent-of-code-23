use std::fs;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

struct Point {
    x: u32,
    y: u32,
}

struct Number {
    num: u32,
    coords: Vec<Point>,
    found: bool,
}

fn main() {
    let path = "day3-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(&input);
    //println!("{}", result);
}

pub fn process(input: &str) {


    for dx in [0,1,2] {
        for dy in [0,1,2] {
            let cx = x+dx-1;
            let cy = y+dy-1;

            for num in found.iter_mut() {
                if (num.coord.y == cy as u32) {
                    let possible_x = num.coord.x ..num.coord.x + num.len as u32;

                    if possible_x.contains(&(cx as u32)) && !num.found {
                        num.found = true;
                        sum += num.num;
                        println!("Found {}", num.num)
                    }
                }
            }
        }
    }
}