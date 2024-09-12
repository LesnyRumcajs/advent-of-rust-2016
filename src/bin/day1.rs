use std::{collections::HashSet, f32::consts::PI, io::BufRead as _, ops::ControlFlow};

fn main() {
    let input = std::io::stdin().lock().lines().next().unwrap().unwrap();
    println!("Day 1, part 1: {}", part1(&input));
    println!("Day 1, part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    calculate_distance(input, false)
}
fn part2(input: &str) -> i32 {
    calculate_distance(input, true)
}

/// Calcuates the distance from beginning to the Easter Bunny Headquarters.
/// If short-circuit, it will stop at the first location that appears twice.
fn calculate_distance(input: &str, short_circuit: bool) -> i32 {
    let mut set: Option<HashSet<(i32, i32)>> = if short_circuit {
        Some(HashSet::new())
    } else {
        None
    };
    let coords = input
        .trim_end()
        .split(", ")
        .map(|cmd| {
            (
                cmd.chars().next().unwrap(),
                cmd[1..].parse::<i32>().unwrap(),
            )
        })
        .try_fold((PI / 2.0, 0, 0), |prev, (dir, steps)| {
            let dir = if dir == 'L' {
                prev.0 + PI / 2.0
            } else {
                prev.0 - PI / 2.0
            };

            let vert = f32::sin(dir).round() as i32;
            let hor = f32::cos(dir).round() as i32;

            let old_y = prev.1;
            let old_x = prev.2;

            let (new_y, new_x) = if let Some(set) = set.as_mut() {
                let mut y = old_y;
                let mut x = old_x;
                for _ in 0..steps {
                    y += vert;
                    x += hor;
                    if !set.insert((y, x)) {
                        return ControlFlow::Break((dir, y, x));
                    }
                }
                (y, x)
            } else {
                ((old_y + vert * steps), (old_x + hor * steps))
            };

            ControlFlow::Continue((dir, new_y, new_x))
        });

    let coords = match coords {
        ControlFlow::Continue(v) => (v.1, v.2),
        ControlFlow::Break(v) => (v.1, v.2),
    };
    coords.0.abs() + coords.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day1/input").unwrap();
        assert_eq!(298, calculate_distance(&input, false));
        assert_eq!(158, calculate_distance(&input, true));
    }

    #[test]
    fn test_examples() {
        assert_eq!(5, calculate_distance("R2, L3", false));
        assert_eq!(2, calculate_distance("R2, R2, R2", false));
        assert_eq!(12, calculate_distance("R5, L5, R5, R3", false));
        assert_eq!(4, calculate_distance("R8, R4, R4, R8", true));
    }
}
