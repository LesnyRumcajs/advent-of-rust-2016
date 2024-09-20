use std::io::BufRead as _;

fn main() {
    let input = std::io::stdin().lock().lines().next().unwrap().unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut index = 0;
    while password.len() < 8 {
        index += 1;
        let hash = md5::compute(format!("{input}{index}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            password
                .push(std::char::from_digit((hash[2].overflowing_shr(8).0) as u32, 16).unwrap());
        }
    }
    password
}
fn part2(input: &str) -> String {
    let mut password = vec!['_'; 8];
    let mut index = 0;
    loop {
        index += 1;
        let hash = md5::compute(format!("{input}{index}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 8 {
            let hex = format!("{hash:x}");
            let position = hex.chars().nth(5).unwrap();
            let position = if position.is_ascii_digit() {
                position.to_digit(10).unwrap()
            } else {
                continue;
            };
            let character = hex.chars().nth(6).unwrap();
            if position < 8 && password[position as usize] == '_' {
                password[position as usize] = character;
                if !password.contains(&'_') {
                    break;
                }
            }
        }
    }
    password.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "ojvtpuvg";
        assert_eq!(part1(input), "4543c154");
        assert_eq!(part2(input), "1050cbbd");
    }
}
