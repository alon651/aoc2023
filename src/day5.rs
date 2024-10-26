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
    return 1;
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
        .map(|n| {
            n.parse::<i64>().unwrap()
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../inputs/day5_example.txt");

    #[test]
    fn part1() {
        assert_eq!(part1_logic(EXAMPLE), 35);
    }
}
