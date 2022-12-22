use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let map = {
        let mut map = vec![];
        for line in input.lines() {
            if line.is_empty() {
                break;
            }

            let row = line
                .chars()
                .map(|c| match c {
                    ' ' => None,
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => unreachable!(),
                })
                .collect_vec();
            map.push(row);
        }
        map
    };

    let instructions = input.lines().last().unwrap();
    let instructions = instructions
        .split_inclusive(['R', 'L'])
        .map(|ins| {
            if ins.ends_with(['L', 'R']) {
                (
                    ins[0..ins.len() - 1].parse().unwrap(),
                    Some(ins.chars().nth(ins.len() - 1).unwrap() == 'R'),
                )
            } else {
                (ins.parse().unwrap(), None)
            }
        })
        .collect_vec();

    let mut position = (
        0,
        map.first()
            .unwrap()
            .iter()
            .position(|t| match t {
                Some(false) => true,
                _ => false,
            })
            .unwrap(),
    );
    let mut facing = 0;

    for (length, clockwise) in instructions {
        for _ in 0..length {
            match facing {
                0 => {
                    if position.1 + 1 >= map[position.0].len()
                        || map[position.0][position.1 + 1].is_none()
                    {
                        let next_pos = map[position.0].iter().position(|t| t.is_some()).unwrap();
                        let next = map[position.0][next_pos];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 = next_pos,
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0][position.1 + 1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 += 1,
                            _ => unreachable!(),
                        }
                    }
                }
                1 => {
                    if position.0 + 1 >= map.len()
                        || map[position.0 + 1]
                            .get(position.1)
                            .map_or(true, |t| t.is_none())
                    {
                        let next_pos = map
                            .iter()
                            .position(|row| match row.get(position.1) {
                                Some(t) => t.is_some(),
                                None => false,
                            })
                            .unwrap();
                        let next = map[next_pos][position.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 = next_pos,
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0 + 1][position.1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 += 1,
                            _ => unreachable!(),
                        }
                    }
                }
                2 => {
                    if position.1 <= 0 || map[position.0][position.1 - 1].is_none() {
                        let next_pos = map[position.0].len()
                            - 1
                            - map[position.0]
                                .iter()
                                .rev()
                                .position(|t| t.is_some())
                                .unwrap();
                        let next = map[position.0][next_pos];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 = next_pos,
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0][position.1 - 1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 -= 1,
                            _ => unreachable!(),
                        }
                    }
                }
                3 => {
                    if position.0 <= 0 || map[position.0 - 1][position.1].is_none() {
                        let next_pos = map.len()
                            - 1
                            - map
                                .iter()
                                .rev()
                                .position(|row| match row.get(position.1) {
                                    Some(t) => t.is_some(),
                                    None => false,
                                })
                                .unwrap();
                        let next = map[next_pos][position.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 = next_pos,
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0 - 1][position.1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 -= 1,
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        facing = match clockwise {
            Some(true) => match facing {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 0,
                _ => unreachable!(),
            },
            Some(false) => match facing {
                0 => 3,
                1 => 0,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            },
            None => facing,
        };
    }

    Some((1 + position.0 as u32) * 1000 + (1 + position.1 as u32) * 4 + facing)
}

// 0    5  10 15
//      +I-+H-+
//      J  |  G
// 5    +--+F-+
//      K  E
//10 +L-+--+
//   M  |  D
//15 +--+C-+
//   N  B
//20 +A-+
//
// A <-> H, B <-> C,
// D <-> G, E <-> F,
// I <-> N, J <-> M,
// K <-> L
pub fn part_two(input: &str) -> Option<u32> {
    let map = {
        let mut map = vec![];
        for line in input.lines() {
            if line.is_empty() {
                break;
            }

            let row = line
                .chars()
                .map(|c| match c {
                    ' ' => None,
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => unreachable!(),
                })
                .collect_vec();
            map.push(row);
        }
        map
    };

    if map.len() < 100 {
        return None;
    }

    let instructions = input.lines().last().unwrap();
    let instructions = instructions
        .split_inclusive(['R', 'L'])
        .map(|ins| {
            if ins.ends_with(['L', 'R']) {
                (
                    ins[0..ins.len() - 1].parse().unwrap(),
                    Some(ins.chars().nth(ins.len() - 1).unwrap() == 'R'),
                )
            } else {
                (ins.parse().unwrap(), None)
            }
        })
        .collect_vec();

    let mut position = (
        0,
        map.first()
            .unwrap()
            .iter()
            .position(|t| match t {
                Some(false) => true,
                _ => false,
            })
            .unwrap(),
    );
    let mut facing = 0;

    for (length, clockwise) in instructions {
        for _ in 0..length {
            match facing {
                0 => {
                    // right
                    if position.1 == 149 && position.0 <= 49 {
                        // G -> D
                        let next_pos = (149 - position.0, 99);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 2;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 99 && position.0 >= 50 && position.0 <= 99 {
                        // E -> F
                        let next_pos = (49, position.0 - 50 + 100);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 3;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 99 && position.0 >= 100 && position.0 <= 149 {
                        // D -> G
                        let next_pos = (149 - position.0, 149);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 2;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 49 && position.0 >= 150 && position.0 <= 199 {
                        // B -> C
                        let next_pos = (149, position.0 - 150 + 50);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 3;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0][position.1 + 1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 += 1,
                            _ => unreachable!(),
                        }
                    }
                }
                1 => {
                    // down
                    if position.1 <= 49 && position.0 == 199 {
                        // A -> H
                        let next_pos = (0, position.1 + 100);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 1;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 >= 50 && position.1 <= 99 && position.0 == 149 {
                        // C -> B
                        let next_pos = (position.1 - 50 + 150, 49);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 2;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 >= 100 && position.1 <= 149 && position.0 == 49 {
                        // F -> E
                        let next_pos = (position.1 - 50, 99);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 2;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0 + 1][position.1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 += 1,
                            _ => unreachable!(),
                        }
                    }
                }
                2 => {
                    // left
                    if position.1 == 50 && position.0 <= 49 {
                        // J -> M
                        let next_pos = (149 - position.0, 0);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 0;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 50 && position.0 >= 50 && position.0 <= 99 {
                        // K -> L
                        let next_pos = (100, position.0 - 50);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 1;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 0 && position.0 >= 100 && position.0 <= 149 {
                        // M -> J
                        let next_pos = (149 - position.0, 50);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 0;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 == 0 && position.0 >= 150 && position.0 <= 199 {
                        // N -> I
                        let next_pos = (0, position.0 - 100);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 1;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0][position.1 - 1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.1 -= 1,
                            _ => unreachable!(),
                        }
                    }
                }
                3 => {
                    // up
                    if position.1 <= 49 && position.0 == 100 {
                        // L -> K
                        let next_pos = (50 + position.1, 50);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 0;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 >= 50 && position.1 <= 99 && position.0 == 0 {
                        // I -> N
                        let next_pos = (position.1 + 100, 0);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => {
                                position = next_pos;
                                facing = 0;
                            }
                            _ => unreachable!(),
                        }
                    } else if position.1 >= 100 && position.1 <= 149 && position.0 == 0 {
                        // H -> A
                        let next_pos = (199, position.1 - 100);
                        let next = map[next_pos.0][next_pos.1];
                        match next {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position = next_pos,
                            _ => unreachable!(),
                        }
                    } else {
                        match map[position.0 - 1][position.1] {
                            Some(true) => {
                                break;
                            }
                            Some(false) => position.0 -= 1,
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        facing = match clockwise {
            Some(true) => match facing {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 0,
                _ => unreachable!(),
            },
            Some(false) => match facing {
                0 => 3,
                1 => 0,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            },
            None => facing,
        };
    }

    Some((1 + position.0 as u32) * 1000 + (1 + position.1 as u32) * 4 + facing)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
