use std::{io::BufRead as _, str::FromStr, string::ParseError};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect_vec();
    println!("Day 4, part 1: {}", part1(&input));
    println!("Day 4, part 2: {}", part2());
}

fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|s| s.parse::<Room>().unwrap())
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum()
}
fn part2() -> i32 {
    unimplemented!();
}

#[derive(Debug, PartialEq)]
struct Room {
    name: Vec<String>,
    sector_id: i32,
    checksum: String,
}

impl FromStr for Room {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(['-', '[', ']'])
            .take_while(|s| !s.is_empty())
            .collect();
        let name = parts[..parts.len() - 2]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let sector_id = parts[parts.len() - 2].parse::<i32>().unwrap();
        let checksum = parts[parts.len() - 1].to_string();
        Ok(Room {
            name,
            sector_id,
            checksum,
        })
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let mut freq = std::collections::HashMap::new();
        for c in self.name.iter().flat_map(|s| s.chars()) {
            *freq.entry(c).or_insert(0) += 1;
        }
        let freq: Vec<(char, i32)> = freq
            .into_iter()
            .sorted_by(|lhs, rhs| rhs.1.cmp(&lhs.1).then_with(|| lhs.0.cmp(&rhs.0)))
            .collect_vec();

        let checksum: String = freq.iter().take(5).map(|(c, _)| *c).collect();
        checksum == self.checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room {
            name: ["aaaaa", "bbb", "z", "y", "x"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            sector_id: 123,
            checksum: "abxyz".to_string(),
        };
        assert_eq!(input.parse::<Room>().unwrap(), expected);
    }

    #[test]
    fn test_examples() {
        let input = [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ]
        .iter()
        .map(ToString::to_string)
        .collect_vec();

        assert_eq!(1514, part1(&input));
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day4/input").unwrap();
        let input: Vec<String> = input.lines().map(ToString::to_string).collect();
        assert_eq!(158835, part1(&input));
    }
}
