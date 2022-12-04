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
        .map(|sack| {
            let sacks = sack.split_at(sack.len() / 2);
            vec![sacks.0, sacks.1]
        })
        .map(|sacks| {
            sacks
                .iter()
                .map(|i| i.chars().collect::<HashSet<_>>())
                .reduce(|acc, set| acc.intersection(&set).cloned().collect())
                .unwrap()
                .iter()
                .collect::<String>()
        })
        .collect::<String>();

    let priority_sum = score(&duplicate_items);

    dbg!(duplicate_items);
    dbg!(priority_sum);

    let group_badges = rucksack
        .iter()
        .array_chunks::<3>()
        .map(|group| {
            group
                .iter()
                .map(|i| i.chars().collect::<HashSet<_>>())
                .reduce(|acc, set| acc.intersection(&set).cloned().collect())
                .unwrap()
                .iter()
                .collect::<String>()
        })
        .collect::<String>();

    let group_score = score(&group_badges);

    dbg!(group_badges);
    dbg!(group_score);
}
