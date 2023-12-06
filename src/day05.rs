use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{max, min, Itertools};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Range {
    start: usize,
    length: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ClipResult {
    contained: Vec<Range>,
    un_contained: Vec<Range>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SplitResult {
    contained: Vec<(Range, CompoundRange)>,
    un_contained: Vec<Range>,
}

impl Range {
    fn end(&self) -> usize {
        self.start + self.length
    }

    fn clip(&self, cheese: &Range) -> ClipResult {
        let mut cheese = *cheese;
        let mut result = ClipResult {
            contained: vec![],
            un_contained: vec![],
        };

        // This means our chees will be cut from the left.
        if cheese.start < self.start {
            let end = *min(&[cheese.end(), self.start]).unwrap();

            result.un_contained.push(Range {
                start: cheese.start,
                length: end - cheese.start,
            });

            cheese.length -= end - cheese.start;
            cheese.start = end;
        }

        // This means our chees will be cut from the right.
        if cheese.end() > self.end() {
            let start = *max(&[cheese.start, self.end()]).unwrap();

            result.un_contained.push(Range {
                start,
                length: cheese.end() - start,
            });

            cheese.length -= cheese.end() - start;
        }

        // If contained or on the thing.
        if cheese.length > 0 {
            result.contained.push(cheese)
        }

        result
    }

    fn split_on_ranges(&self, ranges: &[CompoundRange]) -> SplitResult {
        let mut result = SplitResult {
            contained: vec![],
            un_contained: vec![*self],
        };

        for cr in ranges {
            let mut new_un_contained = vec![];

            for uc in result.un_contained {
                let sub = cr.source.clip(&uc);

                for r in sub.contained {
                    result.contained.push((r, *cr))
                }

                new_un_contained.extend(sub.un_contained);
            }

            result.un_contained = new_un_contained;
        }

        result
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CompoundRange {
    source: Range,
    destination: Range,
}

impl CompoundRange {
    fn new(destination: usize, source: usize, length: usize) -> Self {
        Self {
            source: Range {
                start: source,
                length,
            },
            destination: Range {
                start: destination,
                length,
            },
        }
    }

    fn map(&self, n: usize) -> Option<usize> {
        if self.source.start <= n && n < self.source.start + self.source.length {
            Some(self.destination.start + n - self.source.start)
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    from: String,
    to: String,
    ranges: Vec<CompoundRange>,
}

impl Map {
    fn map(&self, n: usize) -> usize {
        self.ranges.iter().find_map(|r| r.map(n)).unwrap_or(n)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedInput {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

peg::parser! {
    grammar parser() for str {
        pub rule sep() -> String = n:$([' ']+) { n.to_string() }

        pub rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

        pub rule seeds() -> Vec<usize>
            = "seeds:" sep() numbers:(number() ** sep()) { numbers }

        pub rule name() -> String = from:$(['a'..='z']+) { from.to_string() }

        pub rule range() -> CompoundRange
            = destination:number() sep() source:number() sep() length:number() {CompoundRange::new(destination, source,length)}

        pub rule ranges() -> Vec<CompoundRange> = ranges:(range() ** "\n")

        pub rule map() -> Map
            = from:name() "-to-" to:name() sep() "map:\n" ranges:ranges() { Map{from, to, ranges} }

        pub rule maps() -> Vec<Map> = maps:(map() ** "\n\n")

        pub rule input() -> ParsedInput
            = seeds:seeds() "\n\n" maps:maps()  { ParsedInput{seeds, maps} }
    }
}

#[aoc_generator(day5)]
fn parse_day5(input: &str) -> ParsedInput {
    parser::input(input).unwrap()
}

#[aoc(day5, part1)]
fn day5_part1(input: &ParsedInput) -> usize {
    input
        .seeds
        .iter()
        .map(|&s| input.maps.iter().fold(s, |acc, m| m.map(acc)))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn day5_part2(input: &ParsedInput) -> usize {
    let mut seeds = input
        .seeds
        .clone()
        .into_iter()
        .tuples::<(usize, usize)>()
        .map(|(x, y)| {
            (
                Range {
                    start: x,
                    length: y,
                },
                0,
            )
        })
        .collect::<Vec<_>>();

    let mut minm = usize::MAX;
    while let Some((range, stage)) = seeds.pop() {
        let Range { start, length } = range;
        if stage >= input.maps.len() {
            if start < minm {
                minm = start;
            }

            continue;
        }

        let SplitResult {
            contained,
            un_contained,
        } = range.split_on_ranges(&input.maps[stage].ranges);

        for (r, cr) in contained {
            seeds.push((
                Range {
                    start: cr.map(r.start).unwrap(),
                    ..r
                },
                stage + 1,
            ));
        }

        for r in un_contained {
            seeds.push((r, stage + 1));
        }
    }

    minm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        assert_eq!(
            parser::input(input.join("\n").as_str()),
            Ok(ParsedInput {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    Map {
                        from: "seed".to_string(),
                        to: "soil".to_string(),
                        ranges: vec![
                            CompoundRange::new(50, 98, 2),
                            CompoundRange::new(52, 50, 48),
                        ],
                    },
                    Map {
                        from: "soil".to_string(),
                        to: "fertilizer".to_string(),
                        ranges: vec![
                            CompoundRange::new(0, 15, 37),
                            CompoundRange::new(37, 52, 2),
                            CompoundRange::new(39, 0, 15),
                        ],
                    },
                    Map {
                        from: "fertilizer".to_string(),
                        to: "water".to_string(),
                        ranges: vec![
                            CompoundRange::new(49, 53, 8),
                            CompoundRange::new(0, 11, 42),
                            CompoundRange::new(42, 0, 7),
                            CompoundRange::new(57, 7, 4),
                        ],
                    },
                    Map {
                        from: "water".to_string(),
                        to: "light".to_string(),
                        ranges: vec![
                            CompoundRange::new(88, 18, 7),
                            CompoundRange::new(18, 25, 70),
                        ],
                    },
                    Map {
                        from: "light".to_string(),
                        to: "temperature".to_string(),
                        ranges: vec![
                            CompoundRange::new(45, 77, 23),
                            CompoundRange::new(81, 45, 19),
                            CompoundRange::new(68, 64, 13),
                        ],
                    },
                    Map {
                        from: "temperature".to_string(),
                        to: "humidity".to_string(),
                        ranges: vec![CompoundRange::new(0, 69, 1), CompoundRange::new(1, 0, 69),],
                    },
                    Map {
                        from: "humidity".to_string(),
                        to: "location".to_string(),
                        ranges: vec![
                            CompoundRange::new(60, 56, 37),
                            CompoundRange::new(56, 93, 4),
                        ],
                    },
                ],
            })
        )
    }

    #[test]
    fn test_part1() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .join("\n");
        assert_eq!(day5_part1(&parse_day5(&input)), 35);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .join("\n");
        assert_eq!(day5_part2(&parse_day5(&input)), 46);
    }
}
