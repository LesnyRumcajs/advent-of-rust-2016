use std::io::BufRead;

use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[String]) -> usize {
    let expanded = expand(input);
    expanded.iter().map(|x| x.len()).sum::<usize>()
}
fn part2(input: &[String]) -> usize {
    expand_recurse_size_only(input)
}

fn expand(input: &[String]) -> Vec<String> {
    input.iter().map(|x| expand_single(x)).collect()
}

fn expand_recurse_size_only(input: &[String]) -> usize {
    input
        .iter()
        .par_bridge()
        .map(|x| {
            let mut x = x.clone();
            while x.contains('(') {
                x = expand_single(&x);
            }
            x.len()
        })
        .sum()
}

fn expand_single(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();

    let mut marker: Option<(usize, usize)> = None;
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker_temp = String::new();
            for c in chars.by_ref() {
                if c == ')' {
                    marker = marker_temp
                        .split('x')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple();
                    break;
                }
                marker_temp.push(c);
            }
        }

        if let Some((len, times)) = marker {
            let base = (&mut chars).take(len).collect::<String>();
            for _ in 0..times {
                output.push_str(&base);
            }
            marker = None;
        } else {
            output.push(c);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let cases = [
            ("ADVENT", "ADVENT"),
            ("A(1x5)BC", "ABBBBBC"),
            ("(3x3)XYZ", "XYZXYZXYZ"),
            ("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG"),
            ("(6x1)(1x3)A", "(1x3)A"),
            ("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY"),
        ];

        for case in cases {
            assert_eq!(expand_single(case.0), case.1);
        }
    }
}
