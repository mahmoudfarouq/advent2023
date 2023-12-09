use crate::day08::Direction::{Left, Right};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::{max, Itertools};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Eq, PartialEq, Debug)]
pub struct History(Vec<isize>);

impl Deref for History {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl History {
    fn predict_next(&self) -> isize {
        if self.all_zeroes() {
            0
        } else {
            self.last().unwrap() + self.new_history().predict_next()
        }
    }

    fn predict_previous(&self) -> isize {
        if self.all_zeroes() {
            0
        } else {
            self.first().unwrap() - self.new_history().predict_previous()
        }
    }

    fn new_history(&self) -> History {
        History(
            self.iter()
                .zip(self.iter().skip(1))
                .map(|(f, s)| s - f)
                .collect::<Vec<_>>(),
        )
    }

    fn all_zeroes(&self) -> bool {
        self.iter().all(|n| n == &0)
    }
}

peg::parser! {
  grammar input_parser() for str {
    pub rule number() -> isize = n:$("-"?['0'..='9']+) {? n.parse().or(Err("isize")) }
    pub rule history() -> History = series:(number() ** " ") {History(series)}
    pub rule report() -> Vec<History> = histories:(history()**"\n")
  }
}

#[aoc_generator(day9)]
fn parse_day9(input: &str) -> Vec<History> {
    input_parser::report(input).unwrap()
}

#[aoc(day9, part1)]
fn day9_part1(report: &[History]) -> isize {
    report.iter().map(|h| h.predict_next()).sum()
}

#[aoc(day9, part2)]
fn day9_part2(report: &[History]) -> isize {
    report.iter().map(|h| h.predict_previous()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ["0 -3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"].join("\n");
        assert_eq!(
            parse_day9(&input),
            vec![
                History(vec![0, -3, 6, 9, 12, 15]),
                History(vec![1, 3, 6, 10, 15, 21]),
                History(vec![10, 13, 16, 21, 30, 45]),
            ]
        );
    }

    #[test]
    fn test_predict_next() {
        let h = History(vec![0, 3, 6, 9, 12, 15]);

        assert_eq!(h.predict_next(), 18)
    }

    #[test]
    fn test_part1() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"].join("\n");
        assert_eq!(day9_part1(&parse_day9(&input)), 114);
    }

    #[test]
    fn test_part2() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"].join("\n");
        assert_eq!(day9_part2(&parse_day9(&input)), 2);
    }
}
