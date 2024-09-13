use std::str::FromStr;

struct Input {
    a: i32,
    b: i32,
    c: i32,
}

impl FromStr for Input {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().map(|p| p.parse::<i32>().unwrap());
        Ok(Input {
            a: parts.next().unwrap(),
            b: parts.next().unwrap(),
            c: parts.next().unwrap(),
        })
    }
}

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Input>().unwrap())
        .collect::<Vec<_>>();
    println!("Day 3, part 1: {}", part1(&input));
    println!("Day 3, part 2: {}", part2(&input));
}

fn part1(triangles: &[Input]) -> i32 {
    solve(triangles)
}
fn part2(triangles: &[Input]) -> i32 {
    let transchunked = triangles
        .chunks(3)
        .flat_map(|c| {
            let t1 = &c[0];
            let t2 = &c[1];
            let t3 = &c[2];
            vec![
                Input {
                    a: t1.a,
                    b: t2.a,
                    c: t3.a,
                },
                Input {
                    a: t1.b,
                    b: t2.b,
                    c: t3.b,
                },
                Input {
                    a: t1.c,
                    b: t2.c,
                    c: t3.c,
                },
            ]
        })
        .collect::<Vec<_>>();
    solve(&transchunked)
}

fn solve(triangles: &[Input]) -> i32 {
    triangles.iter().fold(0, |acc, t| {
        if t.a + t.b > t.c && t.a + t.c > t.b && t.b + t.c > t.a {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ["5 10 25", "1 1 1"]
            .iter()
            .map(|l| l.parse::<Input>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part1(&input), 1);

        let input = [
            "101 301 501",
            "102 302 502",
            "103 303 503",
            "201 401 601",
            "202 402 602",
            "203 403 603",
        ]
        .iter()
        .map(|l| l.parse::<Input>().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day3/input")
            .unwrap()
            .lines()
            .map(|l| l.parse::<Input>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), 982);
        assert_eq!(part2(&input), 1826);
    }
}
