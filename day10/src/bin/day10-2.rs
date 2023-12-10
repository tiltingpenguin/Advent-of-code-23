use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Position {
    count: i32,
    from: Direction,
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
    let mut mark_map = map.clone();

    let start_loc: Vec<(usize, usize, char)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|&(_, _, c)| c == 'S')
        .collect();
    let start = (start_loc[0].0, start_loc[0].1);
    // Start from pipe below S because my input allows it
    let mut pos = Position {
        count: 1,
        from: Direction::North,
        y: start.1 + 1,
        x: start.0,
    };
    while map[pos.y][pos.x] != 'S' {
        mark_map[pos.y][pos.x] = 'x';
        if pos.from == Direction::North {
            match map[pos.y][pos.x] {
                '|' => pos.y += 1,
                'L' => {
                    pos.x += 1;
                    pos.from = Direction::West
                }
                'J' => {
                    pos.x -= 1;
                    pos.from = Direction::East
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Direction::East {
            match map[pos.y][pos.x] {
                '-' => pos.x -= 1,
                'L' => {
                    pos.y -= 1;
                    pos.from = Direction::South
                }
                'F' => {
                    pos.y += 1;
                    pos.from = Direction::North
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Direction::South {
            match map[pos.y][pos.x] {
                '|' => pos.y -= 1,
                '7' => {
                    pos.x -= 1;
                    pos.from = Direction::East
                }
                'F' => {
                    pos.x += 1;
                    pos.from = Direction::West
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Direction::West {
            match map[pos.y][pos.x] {
                '-' => pos.x += 1,
                'J' => {
                    pos.y -= 1;
                    pos.from = Direction::South
                }
                '7' => {
                    pos.y += 1;
                    pos.from = Direction::North
                }
                _ => unreachable!("Should not stray from path"),
            }
            pos.count += 1;
            continue;
        }
    }
    // non general solution that only works because the input will never contain enclosed spaces in the first or last quarter of each dimension
    let mut result = 0;
    let sqx = map[0].len() / 4;
    let sqy = map.len() / 4;
    for y in mark_map[sqy..mark_map.len() - sqy].to_vec() {
        for x in y[sqx..mark_map.len() - sqx].to_vec() {
            if x != 'x' {
                result += 1;
            }
        }
    }
    return result;
}
