use std::fs;

fn main() {
    let path = "day11-input.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn transpose(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn expand(map: Vec<Vec<char>>, scale: i32) -> Vec<Vec<char>> {
    let buf = vec!['.'; map[0].len()];
    let mut expanded = map.clone();
    let mut offset = 0;
    for (index, line) in map.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            for _ in 0..scale {
                expanded.insert(index + offset, buf.clone());
            }
            offset += scale as usize;
        }
    }
    expanded
}

fn process(input: String) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    map = expand(map, 1);
    map = transpose(map);
    map = expand(map, 1);
    map = transpose(map);
    let mut positions: Vec<(i32, i32)> = Vec::new();
    for (i, y) in map.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            if *x == '#' {
                positions.push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }
    let mut result = 0;
    for (index, start) in positions.iter().enumerate() {
        result += positions[index..]
            .iter()
            .map(|(y, x)| (y - start.0).abs() + (x - start.1).abs())
            .sum::<i32>();
    }
    return result;
}
