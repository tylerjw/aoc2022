use std::collections::BTreeSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();
    let elves: BTreeSet<i32> = data
        .split("\n\n")
        .map(|elf| -> i32 { elf.split('\n').flat_map(|item| item.parse::<i32>()).sum() })
        .collect();

    let top_elf = elves.last().unwrap();
    let top_three: i32 = elves.iter().rev().take(3).sum();

    println!("top_elf: {}", top_elf);
    println!("top_three: {}", top_three);
}
