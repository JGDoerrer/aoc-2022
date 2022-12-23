use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                '#' => {
                    elves.insert((x as i32, y as i32));
                }
                _ => unreachable!(),
            }
        }
    }

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let mut moves = HashMap::new();

        for (x, y) in &elves {
            if [
                (x - 1, y - 1),
                (x - 1, *y),
                (x - 1, y + 1),
                (*x, y - 1),
                (*x, y + 1),
                (x + 1, y - 1),
                (x + 1, *y),
                (x + 1, y + 1),
            ]
            .iter()
            .all(|m| !elves.contains(m))
            {
                continue;
            }

            for direction in directions.iter() {
                let free = match direction {
                    Direction::North => [(x - 1, y - 1), (x - 1, *y), (x - 1, y + 1)]
                        .iter()
                        .all(|m| !elves.contains(m)),
                    Direction::South => [(x + 1, y - 1), (x + 1, *y), (x + 1, y + 1)]
                        .iter()
                        .all(|m| !elves.contains(m)),
                    Direction::West => [(x + 1, y - 1), (*x, y - 1), (x - 1, y - 1)]
                        .iter()
                        .all(|m| !elves.contains(m)),
                    Direction::East => [(x + 1, y + 1), (*x, y + 1), (x - 1, y + 1)]
                        .iter()
                        .all(|m| !elves.contains(m)),
                };
                if free {
                    moves.insert((*x, *y), direction.clone());
                    break;
                }
            }
        }

        let dir = directions.remove(0);
        directions.push(dir);

        let new_positions: Vec<_> = moves
            .iter()
            .map(|((x, y), d)| match d {
                Direction::North => (*x - 1, *y),
                Direction::South => (*x + 1, *y),
                Direction::West => (*x, *y - 1),
                Direction::East => (*x, *y + 1),
            })
            .collect();

        for ((x, y), d) in moves {
            let new_pos = match d {
                Direction::North => (x - 1, y),
                Direction::South => (x + 1, y),
                Direction::West => (x, y - 1),
                Direction::East => (x, y + 1),
            };

            if new_positions.iter().filter(|pos| **pos == new_pos).count() <= 1 {
                elves.remove(&(x, y));
                elves.insert(new_pos);
            }
        }
    }

    let min_x = elves.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = elves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = elves.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = elves.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let area = ((max_x - min_x + 1) * (max_y - min_y + 1)) as u32;

    Some(area - elves.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                '#' => {
                    elves.insert((x as i32, y as i32));
                }
                _ => unreachable!(),
            }
        }
    }

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for i in 0.. {
        let mut moves = HashMap::new();

        for (x, y) in &elves {
            if [
                (x - 1, y - 1),
                (x - 1, *y),
                (x - 1, y + 1),
                (*x, y - 1),
                (*x, y + 1),
                (x + 1, y - 1),
                (x + 1, *y),
                (x + 1, y + 1),
            ]
            .into_iter()
            .all(|m| !elves.contains(&m))
            {
                continue;
            }

            for direction in directions.iter() {
                let free = match direction {
                    Direction::North => [(x - 1, y - 1), (x - 1, *y), (x - 1, y + 1)]
                        .into_iter()
                        .all(|m| !elves.contains(&m)),
                    Direction::South => [(x + 1, y - 1), (x + 1, *y), (x + 1, y + 1)]
                        .into_iter()
                        .all(|m| !elves.contains(&m)),
                    Direction::West => [(x + 1, y - 1), (*x, y - 1), (x - 1, y - 1)]
                        .into_iter()
                        .all(|m| !elves.contains(&m)),
                    Direction::East => [(x + 1, y + 1), (*x, y + 1), (x - 1, y + 1)]
                        .into_iter()
                        .all(|m| !elves.contains(&m)),
                };
                if free {
                    moves.insert((*x, *y), direction.clone());
                    break;
                }
            }
        }

        let dir = directions.remove(0);
        directions.push(dir);

        let new_positions: HashSet<_> = moves
            .iter()
            .map(|((x, y), d)| match d {
                Direction::North => (*x - 1, *y),
                Direction::South => (*x + 1, *y),
                Direction::West => (*x, *y - 1),
                Direction::East => (*x, *y + 1),
            })
            .counts()
            .into_iter()
            .filter(|a| a.1 > 1)
            .map(|a| a.0)
            .collect();

        let mut moved = false;

        for ((x, y), d) in moves {
            let new_pos = match d {
                Direction::North => (x - 1, y),
                Direction::South => (x + 1, y),
                Direction::West => (x, y - 1),
                Direction::East => (x, y + 1),
            };

            if !new_positions.contains(&new_pos) {
                elves.remove(&(x, y));
                elves.insert(new_pos);
                moved = true;
            }
        }

        if !moved {
            return Some(i + 1);
        }
    }

    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
