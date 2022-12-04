use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_assignments(code: &str) -> HashSet<i32> {
    let numbers: Vec<_> = code.split('-').flat_map(|n| n.parse::<i32>()).collect();
    (numbers[0]..=numbers[1]).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();
    let pairs = data
        .split('\n')
        .flat_map(|elves| {
            let assignments = elves.split(',').map(parse_assignments).collect::<Vec<_>>();
            if assignments[0].is_subset(&assignments[1])
                || assignments[1].is_subset(&assignments[0])
            {
                return Some(1);
            }
            None
        })
        .collect::<Vec<_>>();

    dbg!(pairs.len());

    let some_overlap = data
        .split('\n')
        .flat_map(|elves| {
            let assignments = elves.split(',').map(parse_assignments).collect::<Vec<_>>();
            if !assignments[0]
                .intersection(&assignments[1])
                .cloned()
                .collect::<Vec<_>>()
                .is_empty()
                || !assignments[1]
                    .intersection(&assignments[0])
                    .cloned()
                    .collect::<Vec<_>>()
                    .is_empty()
            {
                return Some(1);
            }
            None
        })
        .collect::<Vec<_>>();

    dbg!(some_overlap.len());
}
