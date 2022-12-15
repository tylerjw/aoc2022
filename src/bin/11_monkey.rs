use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: usize,
}

fn get_test_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([79, 98]),
            operation: Box::new(|x| x * 19),
            test_divisor: 23,
            true_monkey: 2,
            false_monkey: 3,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: Box::new(|x| x + 6),
            test_divisor: 19,
            true_monkey: 2,
            false_monkey: 0,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: Box::new(|x| x * x),
            test_divisor: 13,
            true_monkey: 1,
            false_monkey: 3,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([74]),
            operation: Box::new(|x| x + 3),
            test_divisor: 17,
            true_monkey: 0,
            false_monkey: 1,
            inspect_count: 0,
        },
    ]
}

fn get_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([54, 98, 50, 94, 69, 62, 53, 85]),
            operation: Box::new(|x| x * 13),
            test_divisor: 3,
            true_monkey: 2,
            false_monkey: 1,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([71, 55, 82]),
            operation: Box::new(|x| x + 2),
            test_divisor: 13,
            true_monkey: 7,
            false_monkey: 2,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([77, 73, 86, 72, 87]),
            operation: Box::new(|x| x + 8),
            test_divisor: 19,
            true_monkey: 4,
            false_monkey: 7,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([97, 91]),
            operation: Box::new(|x| x + 1),
            test_divisor: 17,
            true_monkey: 6,
            false_monkey: 5,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([78, 97, 51, 85, 66, 63, 62]),
            operation: Box::new(|x| x * 17),
            test_divisor: 5,
            true_monkey: 6,
            false_monkey: 3,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([88]),
            operation: Box::new(|x| x + 3),
            test_divisor: 7,
            true_monkey: 1,
            false_monkey: 0,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([87, 57, 63, 86, 87, 53]),
            operation: Box::new(|x| x * x),
            test_divisor: 11,
            true_monkey: 5,
            false_monkey: 0,
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from([73, 59, 82, 65]),
            operation: Box::new(|x| x + 6),
            test_divisor: 2,
            true_monkey: 4,
            false_monkey: 3,
            inspect_count: 0,
        },
    ]
}

fn play_round_part_1(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while let Some(worry) = monkeys[i].items.pop_front() {
            let worry = (monkeys[i].operation)(worry);
            let worry = worry / 3;
            let res = worry % monkeys[i].test_divisor == 0;
            let throw = if res {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };
            monkeys[throw].items.push_back(worry);
            monkeys[i].inspect_count += 1;
        }
    }
}

fn calc_modulo(monkeys: &[Monkey]) -> u64 {
    monkeys.iter().map(|m| m.test_divisor).product()
}

fn play_round_part_2(monkeys: &mut Vec<Monkey>, modulo: u64) {
    for i in 0..monkeys.len() {
        while let Some(worry) = monkeys[i].items.pop_front() {
            let worry = (monkeys[i].operation)(worry);
            let worry = worry % modulo;
            let res = worry % monkeys[i].test_divisor == 0;
            let throw = if res {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };
            monkeys[throw].items.push_back(worry);
            monkeys[i].inspect_count += 1;
        }
    }
}

fn calc_monkey_business(monkeys: &[Monkey]) -> usize {
    let mut counts = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    counts.sort();
    counts.last().unwrap() * counts.len().checked_sub(2).map(|i| counts[i]).unwrap()
}

#[test]
fn test_monkey() {
    let mut monkeys = get_test_input();

    // rounds
    for _ in 1..=20 {
        play_round_part_1(&mut monkeys);
    }

    // calculate monkey business
    let monkey_business = calc_monkey_business(&monkeys);

    assert_eq!(monkey_business, 10605);
}

fn main() {
    let mut monkeys = get_input();

    let modulo = calc_modulo(&monkeys);

    // rounds
    for round in 1..=10000 {
        play_round_part_2(&mut monkeys, modulo);

        println!("\nRound: {}", round);
        monkeys.iter().enumerate().for_each(|(i, monkey)| {
            println!("Monkey {}: items: {:?}", i, monkey.items);
        });
    }

    // calculate monkey business
    dbg!(calc_monkey_business(&monkeys));
}
