use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn distance(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

type BBox = ((i32, i32), (i32, i32), (i32, i32));
fn count_exterior_area(start: (i32, i32, i32), bbox: &BBox, cubes: &[(i32, i32, i32)]) -> i32 {
    let mut queue = VecDeque::new();
    let mut used = HashSet::new();
    queue.push_front(start);
    used.insert(start);

    let mut result = 0;
    let &((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bbox;

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        let cube = (x, y, z);

        for other in cubes.iter() {
            if distance(other, &cube) == 1 {
                result += 1;
            }
        }

        for d in [-1, 1] {
            let x_cube = (x + d, y, z);
            if min_x <= x + d
                && max_x >= x + d
                && !used.contains(&x_cube)
                && !cubes.contains(&x_cube)
            {
                queue.push_back(x_cube);
                used.insert(x_cube);
            }

            let y_cube = (x, y + d, z);
            if min_y <= y + d
                && max_y >= y + d
                && !used.contains(&y_cube)
                && !cubes.contains(&y_cube)
            {
                queue.push_back(y_cube);
                used.insert(y_cube);
            }

            let z_cube = (x, y, z + d);
            if min_z <= z + d
                && max_z >= z + d
                && !used.contains(&z_cube)
                && !cubes.contains(&z_cube)
            {
                queue.push_back(z_cube);
                used.insert(z_cube);
            }
        }
    }
    result
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let data = fs::read_to_string(&args[1]).unwrap();

    let cubes = data
        .lines()
        .map(|line| line.split(',').flat_map(|n| n.parse::<i32>()).collect_vec())
        .map(|cube| (cube[0], cube[1], cube[2]))
        .collect_vec();

    let surface = cubes
        .iter()
        .map(|a| 6 - cubes.iter().filter(|&b| distance(a, b) == 1).count() as i32)
        .sum::<i32>();

    dbg!(surface);

    let bbox: BBox = (
        (
            cubes.iter().map(|cube| cube.0).min().unwrap() - 10,
            cubes.iter().map(|cube| cube.0).max().unwrap() + 10,
        ),
        (
            cubes.iter().map(|cube| cube.1).min().unwrap() - 10,
            cubes.iter().map(|cube| cube.1).max().unwrap() + 10,
        ),
        (
            cubes.iter().map(|cube| cube.2).min().unwrap() - 10,
            cubes.iter().map(|cube| cube.2).max().unwrap() + 10,
        ),
    );

    // start min of min
    let start = (bbox.0 .0, bbox.1 .0, bbox.2 .0);
    let exterior_area = count_exterior_area(start, &bbox, &cubes);

    dbg!(exterior_area);
}
