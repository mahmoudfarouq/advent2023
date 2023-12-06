use aoc_runner_derive::{aoc, aoc_generator};
use std::time;
use std::time::Duration;

type Distance = usize;
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Race {
    time: time::Duration,
    distance: Distance,
}

impl From<(usize, usize)> for Race {
    fn from((t, d): (usize, usize)) -> Self {
        return Race {
            time: Duration::from_millis(t as u64),
            distance: d,
        };
    }
}

peg::parser! {
  grammar game_parser() for str {
    pub rule number() -> usize
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        pub rule sep() -> String = n:$([' ']+) { n.to_string() }

    pub rule game() -> Vec<Race>
      = "Time:" sep() times:(number() ** sep()) "\n" "Distance:" sep() distances:(number() ** sep())  {
            times.into_iter().zip(distances).map(Race::from).collect()
        }
  }
}

#[aoc_generator(day6)]
fn parse_day6(input: &str) -> Vec<Race> {
    game_parser::game(input).unwrap()
}

#[aoc(day6, part1)]
fn day6_part1(games: &[Race]) -> usize {
    games
        .iter()
        .map(|g| {
            (0..g.time.as_millis())
                .map(|dt| (dt * (g.time.as_millis() - dt) > g.distance as u128) as usize)
                .sum::<usize>()
        })
        .product()
}

#[aoc(day6, part2)]
fn day6_part2(_: &[Race]) -> usize {
    day6_part1(&[Race {
        time: Duration::from_millis(50748685),
        distance: 242101716911252,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ["Time:      7  15   30", "Distance:  9  40  200"].join("\n");
        assert_eq!(day6_part1(&parse_day6(&input)), 288);

        let input = ["Time:      71530", "Distance:  940200"].join("\n");
        assert_eq!(day6_part1(&parse_day6(&input)), 71503);
    }
}
