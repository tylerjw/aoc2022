#![feature(iter_array_chunks)]
use std::collections::hash_set::HashSet;
use std::env;
use std::fs;

fn score(items: &str) -> i32 {
    items
        .chars()
        .map(|item| {
            match item as i32 {
                n @ 97..=122 => n - 96, // lower case
                n @ 65..=90 => n - 38,  // upper case
                _ => panic!("nooooo"),
            }
        })
        .sum::<i32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();
    let rucksack = data.split('\n').collect::<Vec<_>>();
    let duplicate_items = rucksack
        .iter()
        .map(|sack| sack.split_at(sack.len() / 2))
        .map(|sacks| {
            let a = sacks.0.chars().collect::<HashSet<_>>();
            let b = sacks.1.chars().collect::<HashSet<_>>();
            a.intersection(&b).collect::<String>()
        })
        .collect::<String>();

    let priority_sum = score(&duplicate_items);

    dbg!(duplicate_items);
    dbg!(priority_sum);

    let group_badges = rucksack
        .iter()
        .array_chunks::<3>()
        .map(|group| {
            group[0]
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&group[1].chars().collect::<HashSet<_>>())
                .collect::<String>()
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&group[2].chars().collect::<HashSet<_>>())
                .collect::<String>()
        })
        .collect::<String>();

    let group_score = score(&group_badges);

    dbg!(group_badges);
    dbg!(group_score);
}
