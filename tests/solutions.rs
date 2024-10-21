use paste::paste;
macro_rules! solution_day {
    ($day:ident,$part1:expr,$part2:expr) => {
        paste! {
            #[test]
            fn [<solution_ $day>]() {
            assert_cmd::Command::cargo_bin(stringify!($day))
                .unwrap()
                .pipe_stdin(format!("inputs/{}/input", stringify!($day)))
                .unwrap()
                .assert()
                .success()
                .stdout(concat!($part1, "\n", $part2, "\n"));
            }
        }
    };
}

solution_day!(day1, "298", "158");
solution_day!(day2, "24862", "46C91");
solution_day!(day3, "982", "1826");
solution_day!(day4, "158835", "993");
solution_day!(day5, "4543c154", "1050cbbd");
solution_day!(day6, "wkbvmikb", "evakwaga");
solution_day!(day7, "115", "231");
solution_day!(
    day8,
    "128",
    "\
####..##...##..###...##..###..#..#.#...#.##...##..
#....#..#.#..#.#..#.#..#.#..#.#..#.#...##..#.#..#.
###..#..#.#..#.#..#.#....#..#.####..#.#.#..#.#..#.
#....#..#.####.###..#.##.###..#..#...#..####.#..#.
#....#..#.#..#.#.#..#..#.#....#..#...#..#..#.#..#.
####..##..#..#.#..#..###.#....#..#...#..#..#..##.."
);
solution_day!(day9, "110346", "10774309173");
solution_day!(day10, "181", "12567");
