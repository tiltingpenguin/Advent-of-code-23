use std::{fs, collections::HashMap};
use nom::bytes::complete::{tag, take_till, is_a};

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    node: String,
    state: Res
}

impl Part {
    fn sum(&self) -> i32 {
        return self.x + self.m + self.a + self.s;
    }
}

#[derive(Debug)]
struct Condition {
    dim: char,
    comp: char,
    bound: i32,
    dest: String
}

#[derive(Debug, PartialEq, Eq)]
enum Res {
    U,
    A,
    R
}

fn main() {
    let path = "day19-test.txt";
    let input = fs::read_to_string(path).expect("fail");
    let result = process(input);
    println!("{}", result);
}

fn make_graph(mut nodes: HashMap<String, (Vec<Condition>, String)>, line: String) -> HashMap<String, (Vec<Condition>, String)> {
    let ob: Result<(&str, &str), nom::Err<()>> = take_till(|c| c == '{')(&line);
    let x = ob.unwrap();
    let name = x.1.to_string();
    let mut c = x.0.to_string();
    c.pop();
    c.remove(0);
    let mut conds: Vec<String> = c.split(',').map(|s| s.to_string()).collect();
    let default = conds.remove(conds.len()-1);
    let mut y: Vec<Condition> = vec![];
    for con in conds {
        let a: char = con.chars().nth(0).unwrap();
        let b = con.chars().nth(1).unwrap();
        let c = con[2..].split(":").nth(0).unwrap().parse::<i32>().unwrap();
        let d = con.split(":").nth(1).unwrap();
        y.push(Condition{dim: a, comp: b, bound: c, dest: d.to_string()});
    }
    nodes.insert(name.to_string(), (y, default));
    nodes
}

fn get_parts(mut line: String) -> Part {
    line.pop();
    line.remove(0);
    let parts: Vec<&str> = line.split(',').collect();
    let x = parts[0].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let m = parts[1].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let a = parts[2].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let s = parts[3].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    Part{x, m, a, s, node: "in".to_string(), state: Res::U}
}

fn process(input: String) -> i32 {
    let mut result = 0;
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut nodes: HashMap<String, (Vec<Condition>, String)> = HashMap::new();
    let n = sections[0];
    let parts = sections[1];
    for line in n.lines() {
        nodes = make_graph(nodes, line.to_string());
    }
    for line in parts.lines() {
        let mut part = get_parts(line.to_string());
        while part.state == Res::U {
            for condition in &nodes[&part.node].0 {
                match condition.dim {
                    'x' => {
                        match condition.comp {
                            '<' => {if part.x < condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            '>' => {if part.x > condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            _ => unreachable!("not < or >")
                        }
                    },
                    'm' => {
                        match condition.comp {
                            '<' => {if part.m < condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            '>' => {if part.m > condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            _ => unreachable!("not < or >")
                        }
                    },
                    'a' => {
                        match condition.comp {
                            '<' => {if part.a < condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            '>' => {if part.a > condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            _ => unreachable!("not < or >")
                        }
                    },
                    's' => {
                        match condition.comp {
                            '<' => {if part.s < condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            '>' => {if part.s > condition.bound {match condition.dest.as_str() {
                                "A" => {part.state = Res::A; break;},
                                "R" => {part.state = Res::R; break;},
                                _ => part.node = condition.dest.clone(),
                            }} else {continue;}},
                            _ => unreachable!("not < or >")
                        }
                    },
                    _ => unreachable!("parameter should be x, m, a, or s")
                }
            };
            let default_dest = nodes[&part.node].1.clone();
            match default_dest.as_str() {
                "A" => {part.state = Res::A; break;},
                "R" => {part.state = Res::R; break;},
                _ =>  part.node = default_dest.clone()
            }
        }
        dbg!(&part);
        if part.state == Res::A {
            result += part.sum();
        }
    }
    return result;
}