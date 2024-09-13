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

fn part1(instructions: &[String]) -> String {
    let keypad = [
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    solve(&keypad, instructions)
}
fn part2(instructions: &[String]) -> String {
    let keypad = [
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];
    solve(&keypad, instructions)
}

fn solve(keypad: &[Vec<char>], instructions: &[String]) -> String {
    let keypad_max_index = keypad.len() as i32 - 1;
    let mut current = keypad
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == '5')
                .map(|(x, _)| (y as i32, x as i32))
        })
        .unwrap();
    let mut combination = Vec::with_capacity(instructions.len());
    for instruction in instructions {
        for c in instruction.chars() {
            let prev = current;
            match c {
                'U' => current.0 = (current.0 - 1).max(0),
                'D' => current.0 = (current.0 + 1).min(keypad_max_index),
                'L' => current.1 = (current.1 - 1).max(0),
                'R' => current.1 = (current.1 + 1).min(keypad_max_index),
                _ => panic!("Invalid instruction"),
            }
            if keypad[current.0 as usize][current.1 as usize] == ' ' {
                current = prev;
            }
        }
        combination.push(current);
    }
    combination
        .iter()
        .map(|(y, x)| keypad[*y as usize][*x as usize])
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input = ["ULL", "RRDDD", "LURDL", "UUUUD"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), "1985");
        assert_eq!(part2(&input), "5DB3");
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day2/input")
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), "24862");
        assert_eq!(part2(&input), "46C91");
    }
}
