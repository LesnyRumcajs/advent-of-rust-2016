use std::io::BufRead;

// https://stackoverflow.com/a/64499219/4658000
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[String]) -> String {
    solve(input, Mode::Max)
}
fn part2(input: &[String]) -> String {
    solve(input, Mode::Min)
}

enum Mode {
    Min,
    Max,
}

fn solve(input: &[String], mode: Mode) -> String {
    let transposed = input
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    transpose(transposed)
        .iter()
        .map(|v| {
            let mut freq = [0; 26];
            for c in v {
                freq[(*c as u8 - b'a') as usize] += 1;
            }
            let v = match mode {
                Mode::Min => freq.iter().filter(|&&x| x > 0).min().unwrap(),
                Mode::Max => freq.iter().max().unwrap(),
            };
            let idx = freq.iter().position(|&x| x == *v).unwrap();
            (idx as u8 + b'a') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part1(&input), "easter");
        assert_eq!(part2(&input), "advent");
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day6/input")
            .expect("Can't read input")
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(part1(&input), "wkbvmikb");
        assert_eq!(part2(&input), "evakwaga");
    }
}
