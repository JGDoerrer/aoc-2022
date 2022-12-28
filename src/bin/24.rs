#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blizzards = vec![];
    let map = {
        let mut map = vec![];
        for (x, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (y, char) in line.chars().enumerate() {
                row.push(match char {
                    '#' => false,
                    '.' => true,
                    '>' => {
                        blizzards.push((x, y, Direction::East));
                        true
                    }
                    '<' => {
                        blizzards.push((x, y, Direction::West));
                        true
                    }
                    '^' => {
                        blizzards.push((x, y, Direction::North));
                        true
                    }
                    'v' => {
                        blizzards.push((x, y, Direction::South));
                        true
                    }
                    _ => unreachable!(),
                })
            }
            map.push(row);
        }
        map
    };

    let mut positions = vec![(0, 1)];

    for i in 0.. {
        let mut new_positions = vec![];
        for (x, y) in positions {
            new_positions.push((x, y));
            if x > 0 && map[x - 1][y] {
                new_positions.push((x - 1, y));
            }
            if map[x + 1][y] {
                new_positions.push((x + 1, y));
            }
            if map[x][y - 1] {
                new_positions.push((x, y - 1));
            }
            if map[x][y + 1] {
                new_positions.push((x, y + 1));
            }
        }
        positions = new_positions;

        blizzards.iter_mut().for_each(|(x, y, d)| match *d {
            Direction::North => {
                if *x <= 1 {
                    *x = map.len() - 2;
                } else {
                    *x -= 1;
                }
            }
            Direction::South => {
                if *x >= map.len() - 2 {
                    *x = 1;
                } else {
                    *x += 1;
                }
            }
            Direction::West => {
                if *y <= 1 {
                    *y = map[0].len() - 2;
                } else {
                    *y -= 1;
                }
            }
            Direction::East => {
                if *y >= map[0].len() - 2 {
                    *y = 1;
                } else {
                    *y += 1;
                }
            }
        });

        positions.retain(|(x, y)| {
            blizzards
                .iter()
                .map(|(x, y, _)| (x, y))
                .find(|a| *a == (x, y))
                .is_none()
        });

        positions.sort();
        positions.dedup();
        positions.sort_by(|a, b| {
            let ax = map.len() - 1 - a.0;
            let ay = map[0].len() - 2 - a.1;
            let bx = map.len() - 1 - b.0;
            let by = map[0].len() - 2 - b.1;
            let a = ax * ax + ay * ay;
            let b = bx * bx + by * by;
            a.cmp(&b)
        });
        positions.truncate(50);

        if positions.contains(&(map.len() - 1, map[0].len() - 2)) {
            return Some(i + 1);
        }
    }

    unreachable!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut blizzards = vec![];
    let map = {
        let mut map = vec![];
        for (x, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (y, char) in line.chars().enumerate() {
                row.push(match char {
                    '#' => false,
                    '.' => true,
                    '>' => {
                        blizzards.push((x, y, Direction::East));
                        true
                    }
                    '<' => {
                        blizzards.push((x, y, Direction::West));
                        true
                    }
                    '^' => {
                        blizzards.push((x, y, Direction::North));
                        true
                    }
                    'v' => {
                        blizzards.push((x, y, Direction::South));
                        true
                    }
                    _ => unreachable!(),
                })
            }
            map.push(row);
        }
        map
    };

    let mut positions = vec![(0, 1)];

    let mut reached_end = false;
    let mut reached_start = false;

    for i in 0.. {
        let mut new_positions = vec![];
        for (x, y) in positions {
            new_positions.push((x, y));
            if x > 0 && map[x - 1][y] {
                new_positions.push((x - 1, y));
            }
            if x < map.len() - 1 && map[x + 1][y] {
                new_positions.push((x + 1, y));
            }
            if map[x][y - 1] {
                new_positions.push((x, y - 1));
            }
            if map[x][y + 1] {
                new_positions.push((x, y + 1));
            }
        }
        positions = new_positions;

        blizzards.iter_mut().for_each(|(x, y, d)| match *d {
            Direction::North => {
                if *x <= 1 {
                    *x = map.len() - 2;
                } else {
                    *x -= 1;
                }
            }
            Direction::South => {
                if *x >= map.len() - 2 {
                    *x = 1;
                } else {
                    *x += 1;
                }
            }
            Direction::West => {
                if *y <= 1 {
                    *y = map[0].len() - 2;
                } else {
                    *y -= 1;
                }
            }
            Direction::East => {
                if *y >= map[0].len() - 2 {
                    *y = 1;
                } else {
                    *y += 1;
                }
            }
        });

        positions.sort();
        positions.dedup();

        positions.retain(|(x, y)| {
            blizzards
                .iter()
                .map(|(x, y, _)| (x, y))
                .find(|a| *a == (x, y))
                .is_none()
        });

        if !reached_end || reached_start {
            positions.sort_by(|a, b| {
                let ax = map.len() - 1 - a.0;
                let ay = map[0].len() - 2 - a.1;
                let bx = map.len() - 1 - b.0;
                let by = map[0].len() - 2 - b.1;
                let a = ax * ax + ay * ay;
                let b = bx * bx + by * by;
                a.cmp(&b)
            });
        } else if !reached_start {
            positions.sort_by(|a, b| {
                let ax = a.0;
                let ay = a.1 - 1;
                let bx = b.0;
                let by = b.1 - 1;
                let a = ax * ax + ay * ay;
                let b = bx * bx + by * by;
                a.cmp(&b)
            });
        }

        positions.truncate(50);

        if !reached_end && positions.contains(&(map.len() - 1, map[0].len() - 2)) {
            reached_end = true;
            positions = vec![(map.len() - 1, map[0].len() - 2)];
        }
        if reached_end && !reached_start && positions.contains(&(0, 1)) {
            reached_start = true;
            positions = vec![(0, 1)];
        }
        if reached_start && positions.contains(&(map.len() - 1, map[0].len() - 2)) {
            return Some(i + 1);
        }
    }
    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
