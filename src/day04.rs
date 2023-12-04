use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{min, Itertools};

#[derive(Debug, Eq, PartialEq)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}

impl Card {
    /// count returns the count of numbers we have that are winning numbers.
    fn count(&self) -> usize {
        *self
            .have
            .iter()
            .counts_by(|n| self.winning.contains(n))
            .get(&true)
            .unwrap_or(&0)
    }
}

peg::parser! {
  grammar card_parser() for str {
    pub rule number() -> usize
      = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

    pub rule sep() -> String = n:$([' ']+) { n.to_string() }

    pub rule set() -> Vec<usize> = numbers:(number() ** sep())

    pub rule card() -> Card
      = "Card" " "* id:(number()) ":" " "*  winning:(set()) " "* "|" " "*  have:(set()) { Card{id, winning, have} }

    pub rule cards() -> Vec<Card> = cards:(card() ** "\n")
  }
}

#[aoc_generator(day4)]
fn parse_day4(input: &str) -> Vec<Card> {
    card_parser::cards(input).unwrap()
}

#[aoc(day4, part1)]
fn day4_part1(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(Card::count)
        .map(|s| if s > 0 { 2_usize.pow((s - 1) as _) } else { 0 })
        .sum()
}

#[aoc(day4, part2)]
fn day4_part2(cards: &[Card]) -> usize {
    struct G {
        count: usize,
        score: usize,
    }

    let mut scores: Vec<G> = cards
        .iter()
        .map(|c| G {
            count: 1,
            score: c.count(),
        })
        .collect();

    for i in 0..scores.len() {
        let upper_bound = *min(&[i + scores[i].score, scores.len()]).unwrap();
        for j in i + 1..=upper_bound {
            scores[j].count += scores[i].count;
        }
    }

    scores.iter().map(|r| r.count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let input = "123";

        assert_eq!(card_parser::number(input), Ok(123))
    }

    #[test]
    fn test_parse_set() {
        let input = "83 86  6 31 17  9 48 53";

        assert_eq!(
            card_parser::set(input),
            Ok(vec![83, 86, 6, 31, 17, 9, 48, 53])
        )
    }

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        assert_eq!(
            card_parser::card(input),
            Ok(Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                have: vec![83, 86, 6, 31, 17, 9, 48, 53],
            })
        )
    }

    #[test]
    fn test_part1() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .join("\n");
        assert_eq!(day4_part1(&parse_day4(&input)), 13);
    }

    #[test]
    fn test_part2() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .join("\n");
        assert_eq!(day4_part2(&parse_day4(&input)), 30);
    }
}
