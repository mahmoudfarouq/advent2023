use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Span {
    from: Point,
    to: Point,
}

impl Span {
    fn has_adjacent_point(&self, p: &Point) -> bool {
        let (xf, xt, y) = (
            self.from.x as isize,
            self.to.x as isize,
            self.from.y as isize,
        );

        if !(y - 1..=y + 1).contains(&(p.y as isize)) {
            return false;
        }

        if !(xf - 1..=xt + 1).contains(&(p.x as isize)) {
            return false;
        }

        true
    }
}

struct ParsedInput {
    numbers: Vec<(Span, usize)>,
    marks: Vec<(Point, char)>,
}

#[aoc_generator(day3)]
fn parse_day3(input: &str) -> ParsedInput {
    let board = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut numbers = vec![];
    let mut marks = vec![];

    for y in 0..board.len() {
        let mut x = 0;
        while x < board[0].len() {
            match board[y][x] {
                '0'..='9' => {
                    let (span, number) = eat_number(&board, y, x);

                    x = span.to.x + 1;
                    numbers.push((span, number));
                }
                '.' => {
                    x += 1;
                }
                mark => {
                    marks.push((Point { x, y }, mark));
                    x += 1;
                }
            }
        }
    }

    ParsedInput { numbers, marks }
}

#[aoc(day3, part1)]
fn day3_part1(input: &ParsedInput) -> usize {
    let ParsedInput { numbers, marks } = input;

    let mut hash = HashSet::new();
    for (span, num) in numbers {
        for (point, _) in marks.iter() {
            if span.has_adjacent_point(point) {
                hash.insert((span, num));

                break;
            }
        }
    }

    hash.iter().map(|(_, &n)| n).sum()
}

#[aoc(day3, part2)]
fn day3_part2(input: &ParsedInput) -> usize {
    let ParsedInput { numbers, marks } = input;

    marks
        .iter()
        .filter(|(_, c)| c == &'*')
        .map(|(p, _)| {
            numbers
                .iter()
                .filter(|(s, _)| s.has_adjacent_point(p))
                .map(|(_, n)| n)
                .collect::<Vec<_>>()
        })
        .filter(|l| l.len() == 2)
        .map(|l| l.iter().cloned().product::<usize>())
        .sum()
}

fn eat_number(board: &[Vec<char>], y: usize, x: usize) -> (Span, usize) {
    let mut builder = "".to_string();

    let mut cursor = x;
    while cursor < board[0].len() {
        match board[y][cursor] {
            c @ '0'..='9' => {
                builder.push(c);
                cursor += 1;
            }
            _ => {
                break;
            }
        }
    }

    (
        Span {
            from: Point { x, y },
            to: Point { x: cursor - 1, y },
        },
        builder.parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        assert_eq!(day3_part1(&parse_day3(&input)), 4361);
    }

    #[test]
    fn test_part2() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        assert_eq!(day3_part2(&parse_day3(&input)), 467835);
    }
}
