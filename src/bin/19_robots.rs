use std::{collections::VecDeque, env};

type Resources = [u16; 4];
type Blueprint = [Resources; 4];

#[derive(Debug, Clone, Copy)]
struct GameState {
    bank: Resources,
    bots: Resources,
    elapsed: u16,
}

fn max_geodes(blueprint: &Blueprint, max_time: u16) -> u16 {
    let mut max_geodes = 0;

    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
    }

    let mut q = VecDeque::new();
    q.push_back(GameState {
        bank: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(GameState {
        bank,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        // for each of the bots we could buy,
        for i in 0..blueprint.len() {
            // if we already have neough of this bot type, skip
            if bots[i] == max_robots[i] {
                continue;
            }

            let costs = &blueprint[i];
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource
                        // inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= bank[idx] => 0,
                        // no target bot type made yet
                        _ if bots[idx] == 0 => max_time + 1,
                        // remaining cost
                        // (total cost - what we have in bank + what we produce this round)
                        //      / number of bots we have producing this
                        cost => (cost - bank[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit to be exceeded
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            // calculate the new inventory
            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = bank[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            let mut new_bots = bots;
            new_bots[i] += 1;

            q.push_back(GameState {
                bank: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = bank[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }
    max_geodes
}

pub fn part_1(filepath: &str) -> usize {
    let blueprints = parse(filepath);

    blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, 24))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * usize::from(geodes))
        .sum()
}

pub fn part_2(filepath: &str) -> usize {
    let blueprints = parse(filepath);

    blueprints
        .iter()
        .take(3)
        .map(|blueprint| usize::from(max_geodes(blueprint, 32)))
        .product()
}

fn parse(filepath: &str) -> Vec<[[u16; 4]; 4]> {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        // ore bots cost ore
        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        // clay bots cost ore
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        // obsidian bots cost ore and clay
        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];
        // geode bots cost ore and obsidian
        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }

    blueprints
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let p1 = part_1(&args[1]);
    dbg!(p1);
    let p2 = part_2(&args[1]);
    dbg!(p2);
}
