use crate::day08::Direction::{Left, Right};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::{max, Itertools};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Location(String);

impl Deref for Location {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Instruction {
    from: Location,
    left: Location,
    right: Location,
}

pub struct Map {
    directions: Vec<Direction>,
    instructions: Vec<Instruction>,
    hash_map: HashMap<Location, Instruction>,
}

impl Map {
    fn new(directions: Vec<Direction>, instructions: Vec<Instruction>) -> Self {
        Self {
            directions,
            instructions: instructions.clone(),
            hash_map: instructions
                .into_iter()
                .map(|i| (i.clone().from, i))
                .collect(),
        }
    }

    fn transition(&self, location: &Location, direction: &Direction) -> &Location {
        let instruction = &self.hash_map[location];

        match direction {
            Left => &instruction.left,
            Right => &instruction.right,
        }
    }

    fn get(&self, location: &Location) -> &Instruction {
        &self.hash_map[location]
    }
}

peg::parser! {
  grammar input_parser() for str {
    pub rule character() -> char = c:(['A'..='Z'|'0'..='9'])

    pub rule direction() -> Direction = c:(character()) { Direction::from(c) }

    pub rule location() -> Location = c:(character()*<3>) {Location(c.iter().join(""))}

    pub rule instruction() -> Instruction = from:(location()) " = (" left:(location()) ", " right:(location()) ")" {Instruction{from, left, right}}

    pub rule map() -> Map = directions:(direction()*) "\n\n" instructions:(instruction() ** "\n") {Map::new(directions, instructions)}
  }
}

#[aoc_generator(day8)]
fn parse_day8(input: &str) -> Map {
    input_parser::map(input).unwrap()
}

#[aoc(day8, part1)]
fn day8_part1(map: &Map) -> usize {
    let location = Location("AAA".to_string());
    let destination = Location("ZZZ".to_string());

    map.directions
        .iter()
        .cycle()
        .fold_while((0, &location), |(count, location), dir| match location {
            l if l.eq(&destination) => Done((count, l)),
            _ => Continue((count + 1, map.transition(location, dir))),
        })
        .into_inner()
        .0
}

#[aoc(day8, part2)]
fn day8_part2(map: &Map) -> usize {
    map.instructions
        .iter()
        .filter(|i| i.from.ends_with('A'))
        .map(|ins| {
            map.directions
                .iter()
                .cycle()
                .fold_while((0, &ins.from), |(count, location), dir| match location {
                    l if l.ends_with('Z') => Done((count, l)),
                    _ => Continue((count + 1, map.transition(location, dir))),
                })
                .into_inner()
                .0
        })
        .fold(1, lcm)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .join("\n");
        assert_eq!(day8_part1(&parse_day8(&input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .join("\n");
        assert_eq!(day8_part2(&parse_day8(&input)), 6);
    }
}
