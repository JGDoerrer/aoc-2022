use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let (dir, amount) = line.split_at(1);

        let amount = amount.trim().parse().unwrap();

        match dir.chars().next().unwrap() {
            'U' => {
                for _ in 0..amount {
                    head.0 += 1;
                    if (tail.0 - head.0).abs() > 1 {
                        if tail.1 == head.1 {
                            tail.0 += 1;
                        } else {
                            tail.0 += 1;
                            tail.1 = head.1;
                        }
                    }
                    visited.insert(tail);
                }
            }
            'D' => {
                for _ in 0..amount {
                    head.0 -= 1;
                    if (tail.0 - head.0).abs() > 1 {
                        if tail.1 == head.1 {
                            tail.0 -= 1;
                        } else {
                            tail.0 -= 1;
                            tail.1 = head.1;
                        }
                    }
                    visited.insert(tail);
                }
            }
            'L' => {
                for _ in 0..amount {
                    head.1 -= 1;
                    if (tail.1 - head.1).abs() > 1 {
                        if tail.0 == head.0 {
                            tail.1 -= 1;
                        } else {
                            tail.1 -= 1;
                            tail.0 = head.0;
                        }
                    }
                    visited.insert(tail);
                }
            }
            'R' => {
                for _ in 0..amount {
                    head.1 += 1;
                    if (tail.1 - head.1).abs() > 1 {
                        if tail.0 == head.0 {
                            tail.1 += 1;
                        } else {
                            tail.1 += 1;
                            tail.0 = head.0;
                        }
                    }

                    visited.insert(tail);
                }
            }
            _ => unreachable!(),
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let (dir, amount) = line.split_at(1);

        let amount = amount.trim().parse().unwrap();

        for _ in 0..amount {
            match dir.chars().next().unwrap() {
                'U' => rope.get_mut(0).unwrap().0 += 1,
                'D' => rope.get_mut(0).unwrap().0 -= 1,
                'L' => rope.get_mut(0).unwrap().1 -= 1,
                'R' => rope.get_mut(0).unwrap().1 += 1,
                _ => unreachable!(),
            }

            for i in 1..10 {
                let prev = *rope.get(i - 1).unwrap();
                let current = rope.get_mut(i).unwrap();

                if (current.0 - prev.0).abs() > 1 || (current.1 - prev.1).abs() > 1 {
                    if current.0 < prev.0 {
                        current.0 += 1;
                    }
                    if current.0 > prev.0 {
                        current.0 -= 1;
                    }
                    if current.1 < prev.1 {
                        current.1 += 1;
                    }
                    if current.1 > prev.1 {
                        current.1 -= 1;
                    }
                }
            }

            visited.insert(rope[9]);
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
