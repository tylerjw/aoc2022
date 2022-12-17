use rayon::prelude::*;
use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Sensor {
    loc: Pos,
    dist: i32,
}

impl Sensor {
    fn in_range(&self, pos: Pos) -> bool {
        self.loc.dist(pos) <= self.dist
    }
}

fn circle(center: Pos, radius: i32) -> Vec<Pos> {
    (0..radius)
        .flat_map(|dx| {
            let dy = radius - dx;
            vec![
                Pos {
                    x: center.x + dx,
                    y: center.y + dy,
                },
                Pos {
                    x: center.x + dx,
                    y: center.y - dy,
                },
                Pos {
                    x: center.x - dx,
                    y: center.y + dy,
                },
                Pos {
                    x: center.x - dx,
                    y: center.y - dy,
                },
            ]
        })
        .collect::<Vec<_>>()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let data = fs::read_to_string(&args[1]).unwrap();

    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let points = data
        .lines()
        .map(|l| {
            re.captures_iter(l)
                .map(|c| Pos {
                    x: c[1].parse().unwrap(),
                    y: c[2].parse().unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sensors = points
        .iter()
        .map(|p| Sensor {
            loc: p[0],
            dist: p[0].dist(p[1]),
        })
        .collect::<Vec<_>>();

    let min_x = sensors.iter().map(|s| s.loc.x - s.dist).min().unwrap();
    let max_x = sensors.iter().map(|s| s.loc.x + s.dist).max().unwrap();
    dbg!((min_x, max_x));

    let y = 2000000;
    let count = (min_x..=max_x)
        .filter(|x| {
            sensors
                .iter()
                .filter(|s| s.in_range(Pos { x: *x, y }))
                .count()
                > 0
        })
        .count();

    let beacons_in_row = points
        .iter()
        .filter(|pair| pair[1].y == y)
        .map(|p| p[1])
        .collect::<HashSet<_>>();

    dbg!(count);
    dbg!(count - beacons_in_row.len());

    let limit = 4000000;
    let sensor_ref: &Vec<Sensor> = sensors.as_ref();

    let edge_points = sensor_ref
        .into_par_iter()
        .flat_map(|sensor| {
            circle(sensor.loc, sensor.dist + 1)
                .into_par_iter()
                .filter(|pos| pos.x >= 0 && pos.x <= limit && pos.y >= 0 && pos.y <= limit)
        })
        .collect::<Vec<_>>();

    let hole = edge_points
        .into_par_iter()
        .find_first(|pos| !sensor_ref.into_par_iter().any(|s| s.in_range(*pos)))
        .unwrap();

    let freq: i64 = hole.x as i64 * 4000000 + hole.y as i64;
    dbg!(freq);
}

#[test]
fn test_pos() {
    let a = Pos { x: 2, y: 18 };
    let b = Pos { x: -2, y: 15 };

    assert_eq!(a.dist(b), 7);

    let input = "Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    dbg!(re.captures(input));

    for cap in re.captures_iter(input) {
        dbg!(cap);
    }

    assert!(re.is_match(input));
}
