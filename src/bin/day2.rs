use std::io::BufRead as _;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();
    println!("Day 2, part 1: {}", part1(&input));
    println!("Day 2, part 2: {}", part2(&input));
}

fn part1(instructions: &[String]) -> i32 {
    let mut buttons = Vec::with_capacity(instructions.len());
    let mut current = (1, 1);
    for instruction in instructions {
        for c in instruction.chars() {
            match c {
                'U' => current.0 = (current.0 - 1).max(0),
                'D' => current.0 = (current.0 + 1).min(2),
                'L' => current.1 = (current.1 - 1).max(0),
                'R' => current.1 = (current.1 + 1).min(2),
                _ => panic!("Invalid instruction"),
            }
        }
        buttons.push(current);
    }
    buttons
        .iter()
        .fold(0, |acc, (y, x)| acc * 10 + y * 3 + x + 1)
}
fn part2(instructions: &[String]) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    let mut current = (3i32, 0);
    let mut buttons = Vec::with_capacity(instructions.len());
    for instruction in instructions {
        for c in instruction.chars() {
            let prev = current;
            match c {
                'U' => current.0 = (current.0 - 1).max(0),
                'D' => current.0 = (current.0 + 1).min(4),
                'L' => current.1 = (current.1 - 1).max(0),
                'R' => current.1 = (current.1 + 1).min(4),
                _ => panic!("Invalid instruction"),
            }
            if keypad[current.0 as usize][current.1 as usize] == ' ' {
                current = prev;
            }
        }
        buttons.push(current);
    }
    buttons
        .iter()
        .map(|(y, x)| keypad[*y as usize][*x as usize])
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = ["ULL", "RRDDD", "LURDL", "UUUUD"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), 1985);
        assert_eq!(part2(&input), "5DB3");
    }
}
