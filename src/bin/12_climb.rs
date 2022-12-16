use grid::Grid;
use pathfinding::prelude::astar;
use std::env;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Pos(usize, usize);

impl Pos {
    fn neighboors(&self) -> Vec<Pos> {
        let mut ret = vec![Pos(self.0 + 1, self.1), Pos(self.0, self.1 + 1)];
        if self.0 != 0 {
            ret.push(Pos(self.0 - 1, self.1));
        }
        if self.1 != 0 {
            ret.push(Pos(self.0, self.1 - 1));
        }
        ret
    }

    fn distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn from_idx(idx: usize, col_len: usize) -> Pos {
        let row = idx / col_len;
        let col = idx - (row * col_len);
        Pos(row, col)
    }
}

fn find_first(data: &[char], value: char, col_len: usize) -> Option<Pos> {
    data.iter()
        .enumerate()
        .find(|(_, c)| **c == value)
        .map(|(idx, _)| Pos::from_idx(idx, col_len))
}

fn find_path_length(map: &Grid<char>, start: Pos, goal: Pos) -> Option<usize> {
    astar(
        &start,
        |pos| {
            let height = *map.get(pos.0, pos.1).unwrap() as u8;
            pos.neighboors()
                .iter()
                .flat_map(|next| {
                    map.get(next.0, next.1)
                        .map(|next_height| (next, *next_height as u8))
                })
                .filter(|(_, next_height)| *next_height <= height + 1)
                .map(|(pos, _)| (*pos, 1))
                .collect::<Vec<_>>()
        },
        |pos| pos.distance(&goal),
        |pos| *pos == goal,
    )
    .map(|r| r.1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    let col_len = data.lines().next().unwrap().len();
    let data = data.replace('\n', "").chars().collect::<Vec<_>>();
    let start = find_first(&data, 'S', col_len).unwrap();
    let goal = find_first(&data, 'E', col_len).unwrap();

    let map = Grid::from_vec(
        data.iter()
            .map(|v| match v {
                'S' => 'a',
                'E' => 'z',
                x => *x,
            })
            .collect::<Vec<_>>(),
        col_len,
    );

    let part_1 = find_path_length(&map, start, goal).unwrap();
    dbg!(part_1);

    let best_start = map
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'a')
        .map(|(i, _)| Pos::from_idx(i, col_len))
        .flat_map(|pos| find_path_length(&map, pos, goal))
        .min()
        .unwrap();

    dbg!(best_start);
}
