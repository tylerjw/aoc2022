use std::env;
use std::fs;

// parse the stacks
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
//
// stacks are represented by vectors that are pushed into and popped from
// Indexes for the items are as follows
// .1...5...9..
// 0...4...8
// position = (vector * 4) + 1
fn parse_stacks(text: &str) -> Vec<Vec<char>> {
    let n_stacks = (text.split('\n').next().unwrap().len() - 2) / 4 + 1;
    let text = text.split('\n').collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..n_stacks {
        stacks.push(vec![]);
    }

    for line_idx in (0..text.len() - 1).rev() {
        let line = text[line_idx];

        for stack_idx in 0..n_stacks {
            let item = line.chars().nth((stack_idx * 4) + 1).unwrap();
            if item != ' ' {
                stacks[stack_idx].push(item);
            }
        }
    }
    stacks
}

fn last_items(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    // find the width of the stacks
    let data = data.split("\n\n").collect::<Vec<_>>();

    // Crane 9000 - Part 1
    let mut stacks_9000 = parse_stacks(data[0]);

    let re = regex::Regex::new(r"move | from | to ").unwrap();

    for line in data[1].split('\n') {
        let line = re.split(line).collect::<Vec<_>>();
        let n = line[1].parse::<usize>().unwrap();
        let from = line[2].parse::<usize>().unwrap() - 1;
        let to = line[3].parse::<usize>().unwrap() - 1;

        for _ in 0..n {
            let item = stacks_9000[from].pop().unwrap();
            stacks_9000[to].push(item);
        }
    }
    let answer = last_items(&stacks_9000);
    dbg!(answer);

    // Crane 9001 - Part 2
    let mut stacks_9001 = parse_stacks(data[0]);

    for line in data[1].split('\n') {
        let line = re.split(line).collect::<Vec<_>>();
        let n = line[1].parse::<usize>().unwrap();
        let from = line[2].parse::<usize>().unwrap() - 1;
        let to = line[3].parse::<usize>().unwrap() - 1;

        let split_point = stacks_9001[from].len() - n;
        let mut items = stacks_9001[from].split_off(split_point);
        stacks_9001[to].append(&mut items);
    }
    let answer = last_items(&stacks_9001);
    dbg!(answer);
}
