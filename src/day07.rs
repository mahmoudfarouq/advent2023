use crate::day07::CardType::{
    FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair,
};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Card {
    Ace,
    King,
    Queen,
    Joker,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    SneakyJoker,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Joker,
            "T" => Self::T,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum CardType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn cards_type(&self) -> CardType {
        let mut counts = self.cards.iter().counts();

        let sneaky_jokers = counts.remove(&Card::SneakyJoker).unwrap_or(0);

        let mut counts = counts.values().sorted().cloned().rev().collect::<Vec<_>>();

        if sneaky_jokers == 5 {
            counts.push(sneaky_jokers)
        } else {
            counts[0] += sneaky_jokers;
        }

        match counts.len() {
            1 => FiveOfKind,
            a if a >= 5 => HighCard,
            4 => OnePair,
            2 => match counts[0] {
                4 => FourOfKind,
                3 => FullHouse,
                _ => unreachable!(),
            },
            3 => match counts[0] {
                3 => ThreeOfKind,
                2 => TwoPair,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

peg::parser! {
  grammar input_parser() for str {
    pub rule number() -> usize
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule sep() -> String = n:$([' ']+) { n.to_string() }

    pub rule card() -> Card = c:$(['2'..='9'|'A'|'K'|'Q'|'J'|'T']) { Card::from(c) }

    pub rule hand() -> Hand = cards:(card()*) sep() bid:(number()) { Hand{cards, bid} }

    pub rule input() -> Vec<Hand> = hands:(hand() ** "\n")
  }
}

#[aoc_generator(day7)]
fn parse_day7(input: &str) -> Vec<Hand> {
    input_parser::input(input).unwrap()
}

#[aoc(day7, part1)]
fn day7_part1(hands: &[Hand]) -> usize {
    hands
        .iter()
        .cloned()
        .sorted_by(|h1, h2| match h1.cards_type().cmp(&h2.cards_type()) {
            Ordering::Equal => h1.cards.cmp(&h2.cards),
            whatever => whatever,
        })
        .rev()
        .zip(1..)
        .fold(0, |acc, (h, r)| acc + r * h.bid)
}

#[aoc(day7, part2)]
fn day7_part2(hands: &[Hand]) -> usize {
    let mut hands = hands.to_vec();
    for hand in hands.iter_mut() {
        for i in hand.cards.iter_mut() {
            if *i == Card::Joker {
                *i = Card::SneakyJoker
            }
        }
    }

    day7_part1(&hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .join("\n");
        assert_eq!(day7_part1(&parse_day7(&input)), 6440);
    }

    #[test]
    fn test_part2() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .join("\n");
        assert_eq!(day7_part2(&parse_day7(&input)), 5905);
    }
}
