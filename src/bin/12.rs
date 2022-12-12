use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn parse_map(input: &str) -> (Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let mut map = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (x, line) in input.lines().enumerate() {
        map.push(vec![]);

        for (y, char) in line.chars().enumerate() {
            match char {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => {}
            }
            map.last_mut().unwrap().push(match char {
                'a'..='z' => char as u32 - 'a' as u32,
                'S' => 0,
                'E' => 26,
                _ => unreachable!(),
            });
        }
    }

    (map, start, end)
}

fn get_steps(map: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut prev = HashMap::new();

    queue.push(Reverse((0, start)));
    costs.insert(start, 0);

    while let Some(Reverse((_, (x, y)))) = queue.pop() {
        if (x, y) == end {
            break;
        }
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let elevation = map[x][y];
        let mut dirs = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            dirs.push((x - 1, y));
        }
        if y > 0 {
            dirs.push((x, y - 1));
        }

        for (dx, dy) in dirs {
            if let Some(Some(elev)) = map.get(dx).map(|t| t.get(dy)) {
                if *elev <= elevation + 1 {
                    let old_cost = costs.get(&(dx, dy)).unwrap_or(&u32::MAX);
                    let other_cost = costs.get(&(x, y)).unwrap_or(&u32::MAX);

                    if *other_cost + 1 < *old_cost {
                        costs.insert((dx, dy), *other_cost + 1);
                        prev.insert((dx, dy), (x, y));
                    }

                    queue.push(Reverse((*costs.get(&(dx, dy)).unwrap(), (dx, dy))));
                }
            }
        }
    }

    costs.get(&end).map(|t| *t)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse_map(input);

    get_steps(&map, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, _, end) = parse_map(input);

    let mut min_steps = u32::MAX;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 0 {
                min_steps = min_steps.min(get_steps(&map, (x, y), end).unwrap_or(u32::MAX));
            }
        }
    }

    Some(min_steps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
