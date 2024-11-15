use std::{cmp, mem::swap};

const INPUT: &str = include_str!("../inputs/day5.txt");
type Number = i64;

pub fn part1() -> Number {
    part1_logic(INPUT)
}

fn part1_logic(input: &str) -> Number {
    let Guide {
        seeds,
        map_layers: maps,
    } = parse_input(input);

    seeds
        .iter()
        .map(|&seed| process_seed(seed, &maps))
        .min()
        .unwrap()
}

pub fn part2() -> Number {
    part2_logic(INPUT)
}

fn part2_logic(input: &str) -> Number {
    let Guide {
        seeds: seeds_input,
        map_layers: maps,
    } = parse_input(input);

    let seed_ranges: Vec<SeedsRange> = seeds_input
        .chunks(2)
        .map(|pair| SeedsRange::from_seed_pair(pair[0], pair[1]))
        .collect();

    process_ranges(seed_ranges, &maps)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn _slow(input: &str) -> Number {
    let Guide {
        seeds: seeds_input,
        map_layers: maps,
    } = parse_input(input);

    let seeds: Vec<Number> = seeds_input
        .chunks(2)
        .flat_map(|pair| match pair {
            [start, count] => (*start..start + count).collect::<Vec<_>>(),
            _ => vec![],
        })
        .collect();

    seeds
        .iter()
        .map(|seed| {
            let mut current = *seed;
            maps.iter().for_each(|layer| {
                if let Some(map) = layer
                    .maps
                    .iter()
                    .find(|map| map.start_index <= current && map.end_index >= current)
                {
                    current += map.offset;
                }
            });
            current
        })
        .min()
        .unwrap()
}

pub fn slow() -> Number {
    _slow(INPUT)
}
fn parse_input(input: &str) -> Guide {
    let mut lines = input.split("\r\n\r\n");

    let seeds: Vec<Number> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<Number>().unwrap())
        .collect();

    let map_layers = lines
        .map(|translation| MapLayer {
            maps: translation
                .lines()
                .skip(1)
                .map(|line| {
                    let seed_map = line
                        .split_whitespace()
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<Number>>();
                    SeedsMap {
                        start_index: seed_map[1],
                        end_index: (seed_map[1] + seed_map[2] - 1),
                        offset: (seed_map[0] - seed_map[1]),
                    }
                })
                .collect::<Vec<SeedsMap>>(),
        })
        .collect();

    Guide { seeds, map_layers }
}

#[derive(Debug)]
struct SeedsMap {
    start_index: Number,
    end_index: Number,
    offset: Number,
}

impl SeedsMap {
    pub fn contains(&self, val: Number) -> bool {
        self.start_index <= val && self.end_index >= val
    }
}

#[derive(Debug)]
struct SeedsRange {
    start: Number,
    end: Number,
}

impl SeedsRange {
    pub fn is_overlapping(&self, map: &SeedsMap) -> bool {
        !(self.end < map.start_index || self.start > map.end_index)
    }

    pub fn compute_overlap(&self, map: &SeedsMap) -> SeedsRange {
        SeedsRange {
            start: cmp::max(self.start, map.start_index) + map.offset,
            end: cmp::min(self.end, map.end_index) + map.offset,
        }
    }

    pub fn compute_non_overlapping(&self, map: &SeedsMap) -> Vec<Option<SeedsRange>> {
        let overlap_start = cmp::max(self.start, map.start_index);
        let overlap_end = cmp::min(self.end, map.end_index);

        vec![
            (self.start < overlap_start).then(|| SeedsRange {
                start: self.start,
                end: overlap_start - 1,
            }),
            (self.end > overlap_end).then(|| SeedsRange {
                start: overlap_end + 1,
                end: self.end,
            }),
        ]
    }

    pub fn from_seed_pair(start: Number, length: Number) -> Self {
        Self {
            start,
            end: start + length - 1,
        }
    }
}

struct MapLayer {
    maps: Vec<SeedsMap>,
}

impl MapLayer {
    pub fn apply(&self, val: Number) -> Number {
        self.maps
            .iter()
            .find(|map| map.contains(val))
            .map_or(val, |map| val + map.offset)
    }

    pub fn apply_range(&self, ranges: Vec<SeedsRange>) -> Vec<SeedsRange> {
        let mut result = Vec::new();
        let mut unconverted = ranges;
        let mut temp_ranges = Vec::new();

        for range_map in &self.maps {
            for range in unconverted.drain(..) {
                if range.is_overlapping(range_map) {
                    result.push(range.compute_overlap(&range_map));
                    range
                        .compute_non_overlapping(&range_map)
                        .into_iter()
                        .flatten()
                        .for_each(|r| temp_ranges.push(r));
                } else {
                    temp_ranges.push(range);
                }
            }
            swap(&mut unconverted, &mut temp_ranges);
        }

        result.extend(unconverted);
        result
    }
}

struct Guide {
    seeds: Vec<Number>,
    map_layers: Vec<MapLayer>,
}

fn process_seed(mut value: Number, maps: &[MapLayer]) -> Number {
    for map in maps {
        value = map.apply(value);
    }
    value
}

fn process_ranges(initial_ranges: Vec<SeedsRange>, maps: &[MapLayer]) -> Vec<SeedsRange> {
    maps.iter()
        .fold(initial_ranges, |ranges, map| map.apply_range(ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../inputs/day5_example.txt");

    #[test]
    fn part1() {
        assert_eq!(part1_logic(EXAMPLE), 35);
    }

    #[test]
    fn part2() {
        assert_eq!(part2_logic(EXAMPLE), 46)
    }
}
