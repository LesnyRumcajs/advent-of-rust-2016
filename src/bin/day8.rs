use std::{io::BufRead as _, str::FromStr};

use itertools::Itertools as _;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect_vec();
    let part1 = part1(Screen::new(50, 6), &input);
    println!("{}", part1.0);
    part2(part1.1);
}

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("rect") => {
                let (width, height) = parts
                    .next()
                    .unwrap()
                    .split('x')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Ok(Instruction::Rect(width, height))
            }
            Some("rotate") => {
                let dir = parts.next().unwrap();
                let idx = parts.next().unwrap().split('=').last().unwrap().parse()?;
                let by = parts.last().unwrap().parse()?;
                match dir {
                    "row" => Ok(Instruction::RotateRow(idx, by)),
                    "column" => Ok(Instruction::RotateColumn(idx, by)),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![false; width]; height],
        }
    }
}

fn part1(screen: Screen, input: &[String]) -> (i32, Screen) {
    let instructions = input
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect_vec();

    let mut screen = screen;
    for instruction in instructions {
        match instruction {
            Instruction::Rect(width, height) => {
                for y in 0..height {
                    for x in 0..width {
                        screen.pixels[y][x] = true;
                    }
                }
            }
            Instruction::RotateRow(start, count) => {
                screen.pixels[start].rotate_right(count);
            }
            Instruction::RotateColumn(start, count) => {
                let mut column = screen.pixels.iter().map(|row| row[start]).collect_vec();
                column.rotate_right(count);
                for (row, pixel) in screen.pixels.iter_mut().zip(column) {
                    row[start] = pixel;
                }
            }
        }
    }
    (
        screen.pixels.iter().flatten().copied().map(i32::from).sum(),
        screen,
    )
}
fn part2(screen: Screen) {
    for row in &screen.pixels {
        for pixel in row {
            print!("{}", if *pixel { '#' } else { '.' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_examples() {
        let input = [
            "rect 3x2",
            "rotate column x=1 by 1",
            "rotate row y=0 by 4",
            "rotate column x=1 by 1",
        ]
        .iter()
        .map(ToString::to_string)
        .collect_vec();

        let screen = Screen::new(7, 3);
        assert_eq!(6, part1(screen, &input).0);
    }
}
