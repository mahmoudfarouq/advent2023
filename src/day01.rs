use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
fn day1_part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.chars()
                .find_or_first(|c| c.is_numeric())
                .unwrap()
                .to_string()
                + &l.chars()
                    .rev()
                    .find_or_first(|c| c.is_numeric())
                    .unwrap()
                    .to_string()
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum()
}

#[aoc(day1, part2)]
fn day1_part2(input: &str) -> usize {
    input
        .lines()
        .map(splitter)
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum()
}

fn splitter(s: &str) -> Vec<usize> {
    const NAMES: [(usize, &str); 20] = [
        (0, "0"),
        (0, "zero"),
        (1, "1"),
        (1, "one"),
        (2, "2"),
        (2, "two"),
        (3, "3"),
        (3, "three"),
        (4, "4"),
        (4, "four"),
        (5, "5"),
        (5, "five"),
        (6, "6"),
        (6, "six"),
        (7, "7"),
        (7, "seven"),
        (8, "8"),
        (8, "eight"),
        (9, "9"),
        (9, "nine"),
    ];

    NAMES
        .iter()
        .flat_map(|(i, n)| s.match_indices(n).map(move |(idx, _)| (idx, i)))
        .sorted_by_key(|(idx, _)| *idx)
        .map(|(_, n)| *n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].join("\n");
        assert_eq!(day1_part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .join("\n");
        assert_eq!(day1_part2(&input), 281);
    }
}
