use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::Hash;

#[derive(Copy, Debug, PartialEq, Eq, Clone, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Move {
    dir: String,
    n: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();
    let moves = data
        .split('\n')
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<_>>();
            Move {
                dir: String::from(line[0]),
                n: line[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    dbg!(&moves);

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut tail_visited: HashSet<Pos> = HashSet::default();
    tail_visited.insert(tail);

    for m in &moves {
        for _ in 0..m.n {
            // move the head
            match m.dir.as_str() {
                "R" => {
                    head.x += 1;
                }
                "U" => {
                    head.y -= 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "D" => {
                    head.y += 1;
                }
                _ => panic!("nooooo"),
            };

            // move the tail to keep up
            if head.x == tail.x {
                if head.y - tail.y == 2 {
                    tail.y += 1;
                } else if tail.y - head.y == 2 {
                    tail.y -= 1;
                }
            } else if head.y == tail.y {
                if head.x - tail.x == 2 {
                    tail.x += 1;
                } else if tail.x - head.x == 2 {
                    tail.x -= 1;
                }
            } else if (head.x - tail.x).abs() + (head.y - tail.y).abs() > 2 {
                match m.dir.as_str() {
                    "R" => {
                        tail = head;
                        tail.x -= 1;
                    }
                    "U" => {
                        tail = head;
                        tail.y += 1;
                    }
                    "L" => {
                        tail = head;
                        tail.x += 1;
                    }
                    "D" => {
                        tail = head;
                        tail.y -= 1;
                    }
                    _ => panic!("nooooo"),
                };
            }

            tail_visited.insert(tail);
        }
    }

    dbg!(tail_visited.len());
}
