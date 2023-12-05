use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{max, min, Itertools};

#[derive(Debug, Eq, PartialEq)]
pub struct Range {
    destination: usize,
    source: usize,
    length: usize,
}

impl Range {
    fn map(&self, n: usize) -> Option<usize> {
        if self.source <= n && n < self.source + self.length {
            Some(self.destination + n - self.source)
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, n: usize) -> usize {
        self.ranges.iter().find_map(|r| r.map(n)).unwrap_or(n)
        // self.another_map(n, 1).first().unwrap().0
    }

    fn map_range(&self, n: usize, length: usize) -> Vec<(usize, usize)> {
        let mut result =
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

        pub rule range() -> Range
            = destination:number() sep() source:number() sep() length:number() {Range {destination, source,length}}

        pub rule ranges() -> Vec<Range> = ranges:(range() ** "\n")

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

fn help(ranges: &[Range], seed: usize) -> (Option<usize>, usize) {
    for (i, range) in ranges.iter().enumerate() {
        if let Some(new) = range.map(seed) {
            return (Some(i), new);
        }
    }

    (None, seed)
}

#[aoc(day5, part2)]
fn day5_part2(input: &ParsedInput) -> usize {
    let mut seeds = input
        .seeds
        .clone()
        .into_iter()
        .tuples::<(usize, usize)>()
        .map(|(x, y)| (x, y, 0))
        .collect::<Vec<_>>();

    let mut minm = usize::MAX;
    while let Some((seed, length, stage)) = seeds.pop() {
        if stage >= input.maps.len() {
            if seed < minm {
                minm = seed;
            }

            continue;
        }

        match help(&input.maps[stage].ranges, seed) {
            (Some(i), new) => {
                let r = &input.maps[stage].ranges[i];
                let lower = new;
                let upper = *min(&[r.destination + r.length, new + length]).unwrap();

                let left_length = upper - lower;
                seeds.push((new, left_length, stage + 1));

                if length > left_length {
                    let right_length = length - left_length;
                    seeds.push((new + left_length, right_length, stage))
                }
            }
            (None, new) => {
                seeds.push((new, 1, stage + 1));
                if length > 1 {
                    seeds.push((new + 1, length - 1, stage));
                }
            }
        }
    }

    minm

    // let mut min = usize::MAX;
    // for (seed, length) in x {
    //     let mut moving_seed = seed;
    //
    //     while moving_seed < seed + length {
    //         let mut result = moving_seed;
    //
    //         for map in input.maps.iter() {
    //             for range in map.ranges.iter() {
    //                 if let Some(new) = range.map(result) {
    //                     result = new;
    //                     break;
    //
    //                     // let diff = range.destination - new;
    //                     //
    //                     // seed += diff
    //                 }
    //             }
    //         }
    //
    //         if result < min {
    //             min = result
    //         }
    //
    //         moving_seed += 1;
    //     }
    // }
    //
    // min

    // input
    //     .seeds
    //     .clone()
    //     .into_iter()
    //     .tuples::<(usize, usize)>()
    //     .map(|(seed, length)| input.maps.iter().fold(s, |acc, m| m.map(acc)))
    //     .min()
    //     .unwrap()

    // input
    //     .seeds
    //     .clone()
    //     .into_iter()
    //     .tuples::<(usize, usize)>()
    //     // .map(|(x, _)| x)
    //     .flat_map(|(from, length)| from..from + length)
    //     .map(|s| input.maps.iter().fold(s, |acc, m| m.map(acc)))
    //     .min()
    //     .unwrap()
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
                            Range {
                                destination: 50,
                                source: 98,
                                length: 2
                            },
                            Range {
                                destination: 52,
                                source: 50,
                                length: 48
                            }
                        ]
                    },
                    Map {
                        from: "soil".to_string(),
                        to: "fertilizer".to_string(),
                        ranges: vec![
                            Range {
                                destination: 0,
                                source: 15,
                                length: 37
                            },
                            Range {
                                destination: 37,
                                source: 52,
                                length: 2
                            },
                            Range {
                                destination: 39,
                                source: 0,
                                length: 15
                            }
                        ]
                    },
                    Map {
                        from: "fertilizer".to_string(),
                        to: "water".to_string(),
                        ranges: vec![
                            Range {
                                destination: 49,
                                source: 53,
                                length: 8
                            },
                            Range {
                                destination: 0,
                                source: 11,
                                length: 42
                            },
                            Range {
                                destination: 42,
                                source: 0,
                                length: 7
                            },
                            Range {
                                destination: 57,
                                source: 7,
                                length: 4
                            }
                        ]
                    },
                    Map {
                        from: "water".to_string(),
                        to: "light".to_string(),
                        ranges: vec![
                            Range {
                                destination: 88,
                                source: 18,
                                length: 7
                            },
                            Range {
                                destination: 18,
                                source: 25,
                                length: 70
                            }
                        ]
                    },
                    Map {
                        from: "light".to_string(),
                        to: "temperature".to_string(),
                        ranges: vec![
                            Range {
                                destination: 45,
                                source: 77,
                                length: 23
                            },
                            Range {
                                destination: 81,
                                source: 45,
                                length: 19
                            },
                            Range {
                                destination: 68,
                                source: 64,
                                length: 13
                            }
                        ]
                    },
                    Map {
                        from: "temperature".to_string(),
                        to: "humidity".to_string(),
                        ranges: vec![
                            Range {
                                destination: 0,
                                source: 69,
                                length: 1
                            },
                            Range {
                                destination: 1,
                                source: 0,
                                length: 69
                            }
                        ]
                    },
                    Map {
                        from: "humidity".to_string(),
                        to: "location".to_string(),
                        ranges: vec![
                            Range {
                                destination: 60,
                                source: 56,
                                length: 37
                            },
                            Range {
                                destination: 56,
                                source: 93,
                                length: 4
                            }
                        ]
                    }
                ]
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
