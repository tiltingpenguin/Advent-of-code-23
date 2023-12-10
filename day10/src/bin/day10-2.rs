use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Top,
    Right,
    Down,
    Left,
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
    println!("{:?}", result);
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
    let mut pos = Pos {
        count: 1,
        from: Dir::Top,
        y: start.1 + 1,
        x: start.0,
    };
    while map[pos.y][pos.x] != 'S' {
        mark_map[pos.y][pos.x] = 'x';
        if pos.from == Dir::Top {
            match map[pos.y][pos.x] {
                '|' => pos.y += 1,
                'L' => {
                    pos.x += 1;
                    pos.from = Dir::Left
                }
                'J' => {
                    pos.x -= 1;
                    pos.from = Dir::Right
                }
                '.' => panic!(),
                _ => (),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::Right {
            match map[pos.y][pos.x] {
                '-' => pos.x -= 1,
                'L' => {
                    pos.y -= 1;
                    pos.from = Dir::Down
                }
                'F' => {
                    pos.y += 1;
                    pos.from = Dir::Top
                }
                '.' => panic!(),
                _ => (),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::Down {
            match map[pos.y][pos.x] {
                '|' => pos.y -= 1,
                '7' => {
                    pos.x -= 1;
                    pos.from = Dir::Right
                }
                'F' => {
                    pos.x += 1;
                    pos.from = Dir::Left
                }
                '.' => panic!(),
                _ => (),
            }
            pos.count += 1;
            continue;
        }
        if pos.from == Dir::Left {
            match map[pos.y][pos.x] {
                '-' => pos.x += 1,
                'J' => {
                    pos.y -= 1;
                    pos.from = Dir::Down
                }
                '7' => {
                    pos.y += 1;
                    pos.from = Dir::Top
                }
                '.' => panic!(),
                _ => (),
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
