use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Pos {
    count: i32,
    from: Dir,
    x: usize,
    y: usize,
}

fn main() {
    let path = "day10-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn process(input: String) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    let start_loc: Vec<(usize, usize, char)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|&(_, _, c)| c == 'S')
        .collect();
    let start = (start_loc[0].0, start_loc[0].1);
    // Start from pipe below S because my input allows it
    let mut pos = Pos {
        count: 1,
        from: Dir::North,
        y: start.1 + 1,
        x: start.0,
    };
    while map[pos.y][pos.x] != 'S' {
        if pos.from == Dir::North {
            match map[pos.y][pos.x] {
                '|' => pos.y += 1,
                'L' => {
                    pos.x += 1;
                    pos.from = Dir::West
                }
                'J' => {
                    pos.x -= 1;
                    pos.from = Dir::East
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::East {
            match map[pos.y][pos.x] {
                '-' => pos.x -= 1,
                'L' => {
                    pos.y -= 1;
                    pos.from = Dir::South
                }
                'F' => {
                    pos.y += 1;
                    pos.from = Dir::North
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::South {
            match map[pos.y][pos.x] {
                '|' => pos.y -= 1,
                '7' => {
                    pos.x -= 1;
                    pos.from = Dir::East
                }
                'F' => {
                    pos.x += 1;
                    pos.from = Dir::West
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::West {
            match map[pos.y][pos.x] {
                '-' => pos.x += 1,
                'J' => {
                    pos.y -= 1;
                    pos.from = Dir::South
                }
                '7' => {
                    pos.y += 1;
                    pos.from = Dir::North
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
    }
    return pos.count / 2;
}
