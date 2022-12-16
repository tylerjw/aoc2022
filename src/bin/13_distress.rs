use serde_json::to_string;
use serde_json::Number;
use serde_json::Value;
use std::cmp::Ordering;
use std::env;
use std::fs;

fn parse_line(line: &str) -> Vec<Value> {
    let res: Vec<Value> = serde_json::from_str(line).unwrap();
    res
}

fn compare_numbers(left: &Number, right: &Number) -> Ordering {
    let left = left.as_i64().unwrap();
    let right = right.as_i64().unwrap();
    if left == right {
        return Ordering::Equal;
    }
    if left < right {
        return Ordering::Less;
    }
    Ordering::Greater
}

fn compare_lists(left: &[Value], right: &[Value]) -> Ordering {
    let mut first_iter = left.iter();
    let mut second_iter = right.iter();

    loop {
        let left = first_iter.next();
        let right = second_iter.next();

        match (left, right) {
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (None, Some(_)) => {
                return Ordering::Less;
            }
            (None, None) => {
                return Ordering::Equal;
            }
            _ => (),
        };

        let left = left.unwrap();
        let right = right.unwrap();

        let out = match (left, right) {
            (Value::Number(left), Value::Number(right)) => compare_numbers(left, right),
            (Value::Array(left), Value::Array(right)) => compare_lists(left, right),
            (Value::Array(left), Value::Number(right)) => {
                let right: Vec<Value> = vec![Value::Number(right.clone())];
                compare_lists(left, &right)
            }
            (Value::Number(left), Value::Array(right)) => {
                let left: Vec<Value> = vec![Value::Number(left.clone())];
                compare_lists(&left, right)
            }
            _ => Ordering::Greater,
        };

        if matches!(out, Ordering::Greater) || matches!(out, Ordering::Less) {
            return out;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();
    let pairs = data.split("\n\n").collect::<Vec<_>>();

    let index_sum = pairs
        .iter()
        .map(|pair| {
            let left = parse_line(pair.lines().next().unwrap());
            let right = parse_line(pair.lines().nth(1).unwrap());
            compare_lists(&left, &right)
        })
        .enumerate()
        .filter(|(_, o)| matches!(o, Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    dbg!(index_sum);

    let control = vec!["[[2]]", "[[6]]"];
    let mut packets = pairs
        .iter()
        .flat_map(|pair| pair.lines())
        .chain(control)
        .map(parse_line)
        .collect::<Vec<_>>();
    packets.sort_by(|l, r| compare_lists(l, r));

    let packets = packets
        .iter()
        .map(|v| {
            format!("{:?}", v.iter().flat_map(to_string).collect::<Vec<_>>()).replace('\"', "")
        })
        .collect::<Vec<_>>();

    let control_a = packets.iter().position(|line| line == "[[2]]").unwrap() + 1;
    let control_b = packets.iter().position(|line| line == "[[6]]").unwrap() + 1;
    let decoder_key = control_a * control_b;

    dbg!(packets);
    dbg!(decoder_key);
}
