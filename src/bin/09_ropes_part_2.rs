use std::cmp::max;
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

fn compute_distance(from: Pos, to: Pos) -> u32 {
    let delta_x = (to.x - from.x).unsigned_abs();
    let delta_y = (to.y - from.y).unsigned_abs();
    max(delta_x, delta_y)
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

    let mut snake = Vec::new();

    for _ in 0..10 {
        snake.push(Pos { x: 0, y: 0 });
    }

    let mut tail_visited: HashSet<Pos> = HashSet::default();
    tail_visited.insert(Pos { x: 0, y: 0 });

    for m in &moves {
        for _ in 0..m.n {
            // move the head
            match m.dir.as_str() {
                "R" => {
                    snake[0].x += 1;
                }
                "U" => {
                    snake[0].y += 1;
                }
                "L" => {
                    snake[0].x -= 1;
                }
                "D" => {
                    snake[0].y -= 1;
                }
                _ => panic!("nooooo"),
            };

            for lead in 0..9 {
                let follow = lead + 1;

                if compute_distance(snake[lead], snake[follow]) > 1 {
                    let mut delta_x: i32 = snake[lead].x - snake[follow].x;
                    let mut delta_y: i32 = snake[lead].y - snake[follow].y;
                    if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
                        delta_x = delta_x.clamp(-1, 1);
                        delta_y = delta_y.clamp(-1, 1);
                    } else if delta_x.abs() == 2 && delta_y == 0 {
                        delta_x = delta_x.clamp(-1, 1);
                    } else if delta_x == 0 && delta_y.abs() == 2 {
                        delta_y = delta_y.clamp(-1, 1);
                    }

                    snake[follow].x += delta_x;
                    snake[follow].y += delta_y;
                }
            }

            tail_visited.insert(snake[9]);
        }
    }

    dbg!(tail_visited.len());
}
