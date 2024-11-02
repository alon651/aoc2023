use std::{cmp, mem::swap};

const INPUT: &str = include_str!("../inputs/day5.txt");

pub fn part1() -> i64 {
    part1_logic(INPUT)
}

fn part1_logic(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| {
            let mut current = *seed;
            maps.iter().for_each(|layer| {
                if let Some(map) = layer
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

pub fn part2() -> i64 {
    part2_logic(INPUT)
}

fn part2_logic(input: &str) -> i64 {
    let (seeds_input, maps) = parse_input(input);

    let mut seed_ranges: Vec<SeedsRange> = seeds_input
        .chunks(2)
        .map(|pair| SeedsRange {
            start: pair[0],
            end: pair[0] + pair[1] - 1,
        })
        .collect();

    seed_ranges.iter().for_each(|r|println!("{r:?}"));
    
    let mut next_layer_ranges: Vec<SeedsRange> = vec![];
    let mut next_ranges: Vec<SeedsRange> = vec![];
    for mapping in maps {
        for seedmap in &mapping {
            for range in seed_ranges.drain(..) {
                if is_overlapping(&range, seedmap) {
                    next_layer_ranges.push(overlap(&range, seedmap));
                    non_overlapping_parts(&range, seedmap).drain(..).for_each(|part| if let Some(part) = part{
                        next_ranges.push(part);
                    });
                }else {
                    next_ranges.push(range);
                }
            }
            swap(&mut seed_ranges, &mut next_ranges); 
        }
        seed_ranges.extend(next_layer_ranges);
        next_layer_ranges = vec![];
    }
    seed_ranges
        .iter()
        .min_by_key(|range| range.start)
        .unwrap()
        .start
}

fn is_overlapping(range: &SeedsRange, map: &SeedsMap) -> bool {
    (range.start >= map.start_index && range.start <= map.end_index)
        || (range.end >= map.start_index && range.end <= map.end_index)
}

fn overlap(range: &SeedsRange, map: &SeedsMap) -> SeedsRange {
    let res = SeedsRange {
        start: cmp::max(range.start, map.start_index) + map.offset,
        end: cmp::min(range.end, map.end_index) + map.offset,
    };
    if res.end ==0 || res.start==0{
        println!("{range:?}{map:?}");
    }
    res
}

fn non_overlapping_parts(
    range: &SeedsRange,
    map: &SeedsMap,
) -> Vec<Option<SeedsRange>> {
    let overlap_start = cmp::max(range.start, map.start_index);
    let overlap_end = cmp::min(range.end, map.end_index);

    if overlap_start == 0{
        println!("{range:?}")
    }

    let left_part = if range.start < overlap_start {
        Some(SeedsRange {
            start: range.start,
            end: overlap_start - 1,
        })
    } else {
        None
    };

    let right_part = if range.end > overlap_end {
        Some(SeedsRange {
            start: overlap_end + 1,
            end: range.end,
        })
    } else {
        None
    };

    vec![left_part, right_part]
}

fn _slow(input: &str) -> i64 {
    let (seeds_input, maps) = parse_input(input);

    let seeds: Vec<i64> = seeds_input
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

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<SeedsMap>>) {
    let mut lines = input.split("\r\n\r\n");

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    (
        seeds,
        lines
            .map(|translation| {
                translation
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let seed_map = line
                            .split_whitespace()
                            .map(|i| i.parse().unwrap())
                            .collect::<Vec<i64>>();
                        SeedsMap {
                            start_index: seed_map[1],
                            end_index: (seed_map[1] + seed_map[2] - 1),
                            offset: (seed_map[0] - seed_map[1]),
                        }
                    })
                    .collect::<Vec<SeedsMap>>()
            })
            .collect(),
    )
}

#[derive(Debug)]
struct SeedsMap {
    start_index: i64,
    end_index: i64,
    offset: i64,
}

#[derive(Debug)]
struct SeedsRange {
    start: i64,
    end: i64,
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
