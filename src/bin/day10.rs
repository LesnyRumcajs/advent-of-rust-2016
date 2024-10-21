use std::{collections::HashMap, io::BufRead, str::FromStr, string::ParseError};

use regex::Regex;

fn main() {
    let instructions = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Instruction>>();
    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

enum Instruction {
    ValueTo((i32, String)),
    BotTo((String, String, String)),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('v') {
            let value_re = Regex::new(r"^value (\d+) goes to (bot \d+)$").unwrap();
            let (_, [value, sink]) = value_re.captures(s).unwrap().extract();
            Ok(Instruction::ValueTo((
                value.parse().unwrap(),
                sink.to_string(),
            )))
        } else {
            let gives_re =
                Regex::new(r"^(bot \d+) gives low to (\w+ \d+) and high to (\w+ \d+)$").unwrap();

            let (_, [bot_source, low_sink, high_sink]) = gives_re.captures(s).unwrap().extract();
            Ok(Instruction::BotTo((
                bot_source.to_string(),
                low_sink.to_string(),
                high_sink.to_string(),
            )))
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Node {
    low: i32,
    high: i32,
    low_sink: Option<String>,
    high_sink: Option<String>,
}

fn part1(instructions: &[Instruction]) -> i32 {
    let tree = solve(instructions);
    tree.iter()
        .find(|&(_k, v)| v.low == 17 && v.high == 61)
        .unwrap()
        .0
        .split_once(' ')
        .unwrap()
        .1
        .parse()
        .unwrap()
}
fn part2(instructions: &[Instruction]) -> i32 {
    let tree = solve(instructions);
    ["output 0", "output 1", "output 2"]
        .map(|k| tree[k].low)
        .iter()
        .product()
}

fn solve(instructions: &[Instruction]) -> HashMap<String, Node> {
    let mut tree: HashMap<String, Node> = HashMap::new();
    let mut queue: Vec<String> = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::ValueTo((v, to)) => {
                tree.entry(to.to_string())
                    .and_modify(|node| {
                        // this signifies that a node got two signals and is ripe for continuing the
                        // signal
                        if node.low > 0 {
                            queue.push(to.to_string());
                            if node.low > *v {
                                node.high = node.low;
                                node.low = *v;
                            } else {
                                node.high = *v;
                            }
                        } else {
                            node.low = *v;
                        }
                    })
                    .or_insert_with(|| Node {
                        low: *v,
                        ..Default::default()
                    });
            }
            Instruction::BotTo((source, sink_low, sink_high)) => {
                tree.entry(source.to_string())
                    .and_modify(|node| {
                        node.low_sink = Some(sink_low.to_string());
                        node.high_sink = Some(sink_high.to_string());
                    })
                    .or_insert_with(|| Node {
                        low_sink: Some(sink_low.to_string()),
                        high_sink: Some(sink_high.to_string()),
                        ..Default::default()
                    });
            }
        };
    }

    while let Some(item) = queue.pop() {
        let node = tree[&item].clone();
        let high_sink = node.high_sink.unwrap();
        let low_sink = node.low_sink.unwrap();

        tree.entry(high_sink.clone())
            .and_modify(|sink| {
                if sink.low > 0 {
                    queue.push(high_sink);
                    if sink.low > node.high {
                        sink.high = sink.low;
                        sink.low = node.high;
                    } else {
                        sink.high = node.high
                    }
                } else {
                    sink.low = node.high;
                }
            })
            .or_insert_with(|| Node {
                low: node.high,
                ..Default::default()
            });

        tree.entry(low_sink.clone())
            .and_modify(|sink| {
                if sink.low > 0 {
                    queue.push(low_sink);
                    if sink.low > node.low {
                        sink.high = sink.low;
                        sink.low = node.low;
                    } else {
                        sink.high = node.low
                    }
                } else {
                    sink.low = node.low;
                }
            })
            .or_insert_with(|| Node {
                low: node.low,
                ..Default::default()
            });
    }
    tree
}
