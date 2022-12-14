use std::collections::HashSet;

use itertools::Itertools;

pub fn parse_map(input: &str) -> HashSet<(u32, u32)> {
    let mut map = HashSet::new();

    for line in input.lines() {
        let parts = line.split(" -> ");

        for (last, next) in parts.tuple_windows() {
            let last = last.split_once(',').unwrap();
            let last: (u32, u32) = (last.0.parse().unwrap(), last.1.parse().unwrap());

            let next = next.split_once(',').unwrap();
            let next = (next.0.parse().unwrap(), next.1.parse().unwrap());

            if next.0 == last.0 {
                for y in (last.1.min(next.1))..=(last.1.max(next.1)) {
                    map.insert((last.0, y));
                }
            } else {
                for x in (last.0.min(next.0))..=(last.0.max(next.0)) {
                    map.insert((x, last.1));
                }
            }
        }
    }

    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input);

    let lowest = *map.iter().map(|(_, y)| y).max().unwrap();

    let mut count = 0;

    loop {
        let mut sand = (500, 0);

        while sand.1 < lowest {
            let down = (sand.0, sand.1 + 1);
            let down_left = (sand.0 - 1, sand.1 + 1);
            let down_right = (sand.0 + 1, sand.1 + 1);

            if !map.contains(&down) {
                sand = down;
            } else if !map.contains(&down_left) {
                sand = down_left;
            } else if !map.contains(&down_right) {
                sand = down_right;
            } else {
                break;
            }
        }

        if sand.1 < lowest {
            map.insert(sand);
            count += 1;
        } else {
            break;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_map(input);

    let lowest = *map.iter().map(|(_, y)| y).max().unwrap();

    let floor = lowest + 2;

    let mut count = 0;

    loop {
        let mut sand = (500, 0);

        while sand.1 < floor - 1 {
            let down = (sand.0, sand.1 + 1);
            let down_left = (sand.0 - 1, sand.1 + 1);
            let down_right = (sand.0 + 1, sand.1 + 1);

            if !map.contains(&down) {
                sand = down;
            } else if !map.contains(&down_left) {
                sand = down_left;
            } else if !map.contains(&down_right) {
                sand = down_right;
            } else {
                break;
            }
        }

        map.insert(sand);
        count += 1;

        if sand == (500, 0) {
            break;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
