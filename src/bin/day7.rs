use std::io::BufRead;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let re = Regex::new(r"(?:]|\A)(\w+)").unwrap();
    let re_hypernet = Regex::new(r"\[(\w+)\]").unwrap();
    input.iter().fold(0, |acc, entry| {
        let has_hypernet_abba = re_hypernet.captures_iter(entry).any(|cap| {
            cap.iter().skip(1).any(|m| {
                let m = m.unwrap();
                m.as_str()
                    .chars()
                    .tuple_windows()
                    .any(|(a, b, c, d)| a == d && b == c && a != b)
            })
        });
        let has_abba = re.captures_iter(entry).any(|cap| {
            cap.iter().skip(1).any(|m| {
                let m = m.unwrap();
                m.as_str()
                    .chars()
                    .tuple_windows()
                    .any(|(a, b, c, d)| a == d && b == c && a != b)
            })
        });
        (has_abba && !has_hypernet_abba) as i32 + acc
    })
}

fn part2(input: &[String]) -> i32 {
    let re = Regex::new(r"(?:]|\A)(\w+)").unwrap();
    let re_hypernet = Regex::new(r"\[(\w+)\]").unwrap();
    input.iter().fold(0, |acc, entry| {
        let mut hypernet_babs = Vec::new();
        re_hypernet.captures_iter(entry).for_each(|cap| {
            cap.iter().skip(1).for_each(|m| {
                let m = m.unwrap();
                m.as_str().chars().tuple_windows().for_each(|(a, b, c)| {
                    if a == c && b != c {
                        hypernet_babs.push((b, a));
                    }
                })
            })
        });
        let mut abas = Vec::new();
        re.captures_iter(entry).for_each(|cap| {
            cap.iter().skip(1).for_each(|m| {
                let m = m.unwrap();
                m.as_str().chars().tuple_windows().for_each(|(a, b, c)| {
                    if a == c && b != c {
                        abas.push((b, a));
                    }
                })
            })
        });
        acc + (abas
            .iter()
            .any(|aba| hypernet_babs.contains(&(aba.1, aba.0))) as i32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = [
            "abba[mnop]qrst",
            "abcd[bddb]xyyx",
            "aaaa[qwer]tyui",
            "ioxxoj[asdfgh]zxcvbn",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect_vec();

        assert_eq!(2, part1(&input));

        let input = ["aba[bab]xyz", "xyx[xyx]xyx", "aaa[kek]eke", "zazbz[bzb]cdb"]
            .iter()
            .map(|s| s.to_string())
            .collect_vec();

        assert_eq!(3, part2(&input));
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day7/input")
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect_vec();
        assert_eq!(115, part1(&input));
        assert_eq!(231, part2(&input));
    }
}
