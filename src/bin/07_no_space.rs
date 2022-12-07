use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    // find the width of the stacks
    let data = data.split('\n').collect::<Vec<_>>();
    let mut directories: HashMap<String, usize> = HashMap::new();

    let mut current_dir: Vec<&str> = Vec::new();

    for line in data {
        if line.starts_with("$ cd") {
            let dir = line.split_whitespace().last().unwrap();

            if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir);
                directories.insert(current_dir.join(""), 0);
            }
        }

        if line.chars().next().unwrap().is_numeric() {
            let current_dir_str = current_dir.join("");
            let file_size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            directories
                .clone()
                .keys()
                .filter(|key| current_dir_str.starts_with(*key))
                .for_each(|key| {
                    if let Some(current_size) = directories.get_mut(key) {
                        *current_size += file_size;
                    }
                });
        }
    }

    let total_used_space = directories.get("/").unwrap();
    dbg!(total_used_space);

    // sum of small directoires
    let mut small_directories = directories.clone();
    small_directories.retain(|_, size| *size < 100_000);
    let sum_of_small = small_directories.values().sum::<usize>();
    dbg!(sum_of_small);

    let total_space: usize = 70_000_000;
    let unused_space = total_space - total_used_space;
    dbg!(unused_space);

    let need_to_delete = 30_000_000 - unused_space;
    dbg!(need_to_delete);

    let mut smallest_big = directories.clone();
    smallest_big.retain(|_, size| *size >= need_to_delete);

    let by_size = smallest_big
        .iter()
        .map(|(key, value)| (*value, key.clone()))
        .collect::<BTreeMap<usize, String>>();
    let smallest_single_dir = by_size.iter().next().unwrap();
    dbg!(smallest_single_dir);
}
