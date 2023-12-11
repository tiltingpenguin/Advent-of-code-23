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

fn find_expand(map: &[Vec<char>]) -> Vec<i64> {
    let mut expands: Vec<i64> = Vec::new();
    for (index, line) in map.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            expands.push(index as i64);
        }
    }
    expands
}

fn process(input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let scale: i64 = 1000000 - 1;
    let horizontal_lines = find_expand(&map);
    map = transpose(map);
    let verticals = find_expand(&map);
    map = transpose(map);
    let mut positions: Vec<(i64, i64)> = Vec::new();
    for (i, y) in map.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            if *x == '#' {
                positions.push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }
    let mut distance = 0;
    for (index, start) in positions.iter().enumerate() {
        distance += positions[index..]
            .iter()
            .map(|(y, x)| {
                // jank code to find out how many lines got crossed that need to be expanded
                // y and x are swapped but I'm too scared to touch anything to fix
                let yrange: std::ops::Range<i64> = if start.0 > *y {
                    *y..start.0
                } else {
                    start.0..*y
                };
                let xrange: std::ops::Range<i64> = if start.1 > *x {
                    *x..start.1
                } else {
                    start.1..*x
                };
                let hor_line_count = horizontal_lines.iter().filter(|&a| yrange.contains(a)).count() as i64;
                let vert_line_count = verticals.iter().filter(|&a| xrange.contains(a)).count() as i64;
                let dist = (y - start.0).abs() + (x - start.1).abs();
                dist + hor_line_count * scale + vert_line_count * scale
            })
            .sum::<i64>();
    }
    return distance;
}
