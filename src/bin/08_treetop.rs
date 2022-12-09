use std::env;
use std::fs;
use std::iter::successors;

fn parse_heightmap(input: &str) -> (Vec<u32>, usize, usize) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let heightmap = input
        .lines()
        .flat_map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    (heightmap, width, height)
}

fn idx(width: usize, x: usize, y: usize) -> usize {
    width * y + x
}

fn xy(idx: usize, width: usize) -> (usize, usize) {
    let y = idx / width;
    let x = idx - (width * y);
    (x, y)
}

fn get(map: &Vec<u32>, width: usize, x: usize, y: usize) -> u32 {
    map[idx(width, x, y)]
}

fn is_visible(map: &Vec<u32>, width: usize, height: usize, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return true;
    }
    let tree = get(map, width, x, y);
    map[width * y..idx(width, x, y)].iter().all(|t| *t < tree)
        || map[idx(width, x, y) + 1..width * (y + 1)]
            .iter()
            .all(|t| *t < tree)
        || (0..y).all(|py| map[idx(width, x, py)] < tree)
        || (y + 1..height).all(|py| map[idx(width, x, py)] < tree)
}

fn scenic_score(map: &Vec<u32>, width: usize, height: usize, x: usize, y: usize) -> usize {
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return 0;
    }
    let tree = get(map, width, x, y);

    let left = successors(Some(idx(width, x, y) - 1), |&idx| {
        if idx == width * y || map[idx] >= tree {
            return None;
        }
        let idx = idx - 1;
        Some(idx)
    })
    .count();

    let right = successors(Some(idx(width, x, y) + 1), |&idx| {
        if idx == width * (y + 1) - 1 || map[idx] >= tree {
            return None;
        }
        Some(idx + 1)
    })
    .count();

    let up = successors(Some(y - 1), |&py| {
        if py == 0 || map[idx(width, x, py)] >= tree {
            return None;
        }
        let py = py - 1;
        Some(py)
    })
    .count();

    let down = successors(Some(y + 1), |&py| {
        if py == height - 1 || map[idx(width, x, py)] >= tree {
            return None;
        }
        let py = py + 1;
        Some(py)
    })
    .count();

    left * right * up * down
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    let (heightmap, width, height) = parse_heightmap(&data);
    let n = (0..heightmap.len())
        .map(|idx| {
            let (x, y) = xy(idx, width);
            is_visible(&heightmap, width, height, x, y)
        })
        .filter(|x| *x)
        .count();

    dbg!(n);
    dbg!(scenic_score(&heightmap, width, height, 2, 3));

    let max_score = (0..heightmap.len())
        .map(|idx| xy(idx, width))
        .map(|(x, y)| scenic_score(&heightmap, width, height, x, y))
        .max();

    dbg!(max_score);
}
