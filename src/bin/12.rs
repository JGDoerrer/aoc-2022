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

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse_map(input);
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    queue.push(Reverse((0, start)));
    costs.insert(start, 0);

    while let Some(Reverse((_, (x, y)))) = queue.pop() {
        if (x, y) == end {
            break;
        }
        if !visited.insert((x, y)) {
            continue;
        }

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
                    let other_cost = costs.get(&(x, y)).unwrap();

                    let new_cost = (*old_cost).min(*other_cost + 1);

                    if new_cost < *old_cost {
                        costs.insert((dx, dy), new_cost);
                    }

                    queue.push(Reverse((new_cost, (dx, dy))));
                }
            }
        }
    }

    costs.get(&end).map(|t| *t)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, _, end) = parse_map(input);

    let mut min_steps = u32::MAX;
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    queue.push(Reverse((0, end)));
    costs.insert(end, 0);

    while let Some(Reverse((_, (x, y)))) = queue.pop() {
        let elevation = map[x][y];
        if elevation == 0 {
            min_steps = *costs.get(&(x, y)).unwrap();
            break;
        }
        if !visited.insert((x, y)) {
            continue;
        }

        let mut dirs = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            dirs.push((x - 1, y));
        }
        if y > 0 {
            dirs.push((x, y - 1));
        }

        for (dx, dy) in dirs {
            if let Some(Some(elev)) = map.get(dx).map(|t| t.get(dy)) {
                if *elev + 1 >= elevation {
                    let old_cost = costs.get(&(dx, dy)).unwrap_or(&u32::MAX);
                    let other_cost = costs.get(&(x, y)).unwrap();

                    let new_cost = (*old_cost).min(*other_cost + 1);

                    if new_cost < *old_cost {
                        costs.insert((dx, dy), new_cost);
                    }

                    queue.push(Reverse((new_cost, (dx, dy))));
                }
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
