use crate::day02::Cube::{Blue, Green, Red};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Cube {
	Red,
	Green,
	Blue,
}

impl From<String> for Cube {
	fn from(value: String) -> Self {
		match value.as_str() {
			"red" => Red,
			"green" => Green,
			"blue" => Blue,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Set {
	groups: Vec<(Cube, usize)>,
}

impl Set {
	fn count(&self, cube: &Cube) -> usize {
		self.groups
			.iter()
			.filter_map(|(c, n)| if c == cube { Some(n) } else { None })
			.sum()
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
	id: usize,
	sets: Vec<Set>,
}

impl Game {
	fn is_possible(&self, values: &[(Cube, usize)]) -> bool {
		for (c, v) in values {
			for s in self.sets.iter() {
				if s.count(c) > *v {
					return false;
				}
			}
		}

		true
	}

	fn least_possible(&self) -> Set {
		let mut map = HashMap::new();
		for s in self.sets.iter() {
			for (g, s) in s.groups.iter() {
				let e = map.entry(*g).or_insert(0);

				if *e < *s {
					*e = *s
				}
			}
		}

		Set {
			groups: map.into_iter().collect(),
		}
	}
}

peg::parser! {
  grammar game_parser() for str {
    pub rule number() -> usize
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule identifier() -> String
      = id:$(['a'..='z' | 'A'..='Z' | '0'..='9']+) { id.to_string() }

    pub rule group() -> (Cube, usize)
      = n:(number()) " " name:(identifier())  {
      (Cube::from(name), n)
    }

    pub rule set() -> Set
      = groups:(group() ** ", ") { Set{ groups } }

    pub rule game() -> Game
      = "Game " id:(number()) ": "  sets:(set() ** "; ")  { Game{id, sets} }
  }
}

#[aoc_generator(day2)]
fn parse_day2(input: &str) -> Vec<Game> {
	input
		.lines()
		.map(game_parser::game)
		.filter_map(Result::ok)
		.collect()
}

#[aoc(day2, part1)]
fn day2_part1(games: &[Game]) -> usize {
	const VALUES: [(Cube, usize); 3] = [(Red, 12), (Green, 13), (Blue, 14)];

	games
		.iter()
		.filter_map(|g| {
			if g.is_possible(&VALUES) {
				Some(g.id)
			} else {
				None
			}
		})
		.sum()
}

#[aoc(day2, part2)]
fn day2_part2(games: &[Game]) -> usize {
	games
		.iter()
		.map(Game::least_possible)
		.map(|s| s.groups.iter().map(|(_, n)| n).product::<usize>())
		.sum::<usize>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_number() {
		let input = "123";

		assert_eq!(game_parser::number(input), Ok(123))
	}

	#[test]
	fn test_parse_identifier() {
		let input = "green";

		assert_eq!(game_parser::identifier(input), Ok("green".to_string()))
	}

	#[test]
	fn test_parse_group() {
		let input = "14 green";

		assert_eq!(game_parser::group(input), Ok((Green, 14)))
	}

	#[test]
	fn test_parse_set() {
		let input = "3 blue, 4 red";

		assert_eq!(
			game_parser::set(input),
			Ok(Set {
				groups: vec![(Blue, 3), (Red, 4)]
			})
		)
	}

	#[test]
	fn test_parse_game() {
		let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

		assert_eq!(
			game_parser::game(input),
			Ok(Game {
				id: 1,
				sets: vec![
					Set {
						groups: vec![(Blue, 3), (Red, 4)]
					},
					Set {
						groups: vec![(Red, 1), (Green, 2), (Blue, 6)]
					},
					Set {
						groups: vec![(Green, 2)]
					},
				],
			})
		)
	}

	#[test]
	fn test_part1() {
		let input = [
			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		]
			.join("\n");
		assert_eq!(day2_part1(&parse_day2(&input)), 8);
	}

	#[test]
	fn test_part2() {
		let input = [
			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		]
			.join("\n");
		assert_eq!(day2_part2(&parse_day2(&input)), 2286);
	}
}
