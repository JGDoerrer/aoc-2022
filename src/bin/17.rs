use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let rocks: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 1), (1, 0)],
    ];

    let mut input = input.trim().chars().cycle();

    let mut tower = vec![[true; 7]];

    let mut current_rock = rocks[0].clone();
    let mut next_rock = 1;
    let mut rock_count = 0;

    let floor = tower
        .iter()
        .enumerate()
        .rev()
        .find(|(i, row)| row.iter().any(|t| *t))
        .unwrap()
        .0;

    dbg!(floor);

    current_rock
        .iter_mut()
        .for_each(|(x, y)| (*x, *y) = (*x + 2, *y + floor + 4));

    while rock_count < 2022 {
        match input.next() {
            Some('<') => {
                let collision = current_rock.iter().any(|(x, y)| {
                    if *x == 0 {
                        true
                    } else {
                        tower.get(*y).unwrap_or(&[false; 7])[*x - 1]
                    }
                });

                if !collision {
                    current_rock
                        .iter_mut()
                        .for_each(|(x, y)| (*x, *y) = (*x - 1, *y));
                }
            }
            Some('>') => {
                let collision = current_rock.iter().any(|(x, y)| {
                    if *x == 6 {
                        true
                    } else {
                        tower.get(*y).unwrap_or(&[false; 7])[*x + 1]
                    }
                });

                if !collision {
                    current_rock
                        .iter_mut()
                        .for_each(|(x, y)| (*x, *y) = (*x + 1, *y));
                }
            }
            Some(_) => unreachable!(),
            None => {
                unreachable!()
            }
        };

        let collision = current_rock
            .iter()
            .any(|(x, y)| tower.get(y - 1).unwrap_or(&[false; 7])[*x]);

        if collision {
            current_rock
                .iter()
                .for_each(|(x, y)| match tower.get_mut(*y) {
                    Some(row) => {
                        assert!(!row[*x]);
                        row[*x] = true;
                    }
                    None => {
                        while tower.len() <= *y {
                            tower.push([false; 7]);
                        }
                        let row = tower.get_mut(*y).unwrap();
                        assert!(!row[*x]);
                        row[*x] = true;
                    }
                });

            // for y in (0..tower.len()).rev() {
            //     for x in 0..7 {
            //         if tower[y][x] {
            //             print!("#");
            //         } else {
            //             print!(" ");
            //         }
            //     }
            //     println!();
            // }

            current_rock = rocks[next_rock].clone();

            let floor = tower
                .iter()
                .enumerate()
                .rev()
                .find(|(i, row)| row.iter().any(|t| *t))
                .unwrap()
                .0;

            current_rock
                .iter_mut()
                .for_each(|(x, y)| (*x, *y) = (*x + 2, *y + floor + 4));

            next_rock = (next_rock + 1) % rocks.len();
            rock_count += 1;
        } else {
            current_rock
                .iter_mut()
                .for_each(|(x, y)| (*x, *y) = (*x, *y - 1));
        }

        // for y in (0..(tower.len() + 4)).rev() {
        //     for x in 0..7 {
        //         if current_rock.contains(&(x, y)) {
        //             print!("@");
        //         } else if tower.get(y).unwrap_or(&[false; 7])[x] {
        //             print!("#");
        //         } else {
        //             print!(" ");
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }

    let floor = tower
        .iter()
        .enumerate()
        .rev()
        .find(|(i, row)| row.iter().any(|t| *t))
        .unwrap()
        .0;

    Some(floor as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rocks: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 1), (1, 0)],
    ];

    let mut input = input.trim().chars().enumerate().cycle();

    let mut tower = vec![[true; 7]];

    let mut current_rock = rocks[0].clone();
    let mut shift = (0, 0);
    let mut next_rock = 1;
    let mut rock_count = 0;

    let mut top = tower
        .iter()
        .enumerate()
        .rev()
        .find(|(i, row)| row.iter().any(|t| *t))
        .unwrap()
        .0;

    dbg!(top);

    current_rock
        .iter_mut()
        .for_each(|(x, y)| (*x, *y) = (*x + 2, *y + top + 4));
    shift = (2, top + 4);

    let mut cycle_length = 0;
    let mut cycle_height = 0;

    let mut states = vec![];

    loop {
        let (i, input) = input.next().unwrap();
        match input {
            '<' => {
                let collision = current_rock.iter().any(|(x, y)| {
                    if *x == 0 {
                        true
                    } else {
                        tower.get(*y).unwrap_or(&[false; 7])[*x - 1]
                    }
                });

                if !collision {
                    current_rock
                        .iter_mut()
                        .for_each(|(x, y)| (*x, *y) = (*x - 1, *y));
                    shift.0 -= 1;
                }
            }
            '>' => {
                let collision = current_rock.iter().any(|(x, y)| {
                    if *x == 6 {
                        true
                    } else {
                        tower.get(*y).unwrap_or(&[false; 7])[*x + 1]
                    }
                });

                if !collision {
                    current_rock
                        .iter_mut()
                        .for_each(|(x, y)| (*x, *y) = (*x + 1, *y));
                    shift.0 += 1;
                }
            }
            _ => unreachable!(),
        };

        let collision = current_rock
            .iter()
            .any(|(x, y)| tower.get(y - 1).unwrap_or(&[false; 7])[*x]);

        if collision {
            current_rock
                .iter()
                .for_each(|(x, y)| match tower.get_mut(*y) {
                    Some(row) => {
                        assert!(!row[*x]);
                        row[*x] = true;
                    }
                    None => {
                        while tower.len() <= *y {
                            tower.push([false; 7]);
                        }
                        let row = tower.get_mut(*y).unwrap();
                        assert!(!row[*x]);
                        row[*x] = true;
                    }
                });
            current_rock = rocks[next_rock].clone();

            top = tower
                .iter()
                .enumerate()
                .rev()
                .find(|(i, row)| row.iter().any(|t| *t))
                .unwrap()
                .0;

            current_rock
                .iter_mut()
                .for_each(|(x, y)| (*x, *y) = (*x + 2, *y + top + 4));
            shift = (2, top + 4);

            next_rock = (next_rock + 1) % rocks.len();
            rock_count += 1;

            let cycle = states
                .iter()
                .filter(|(_, _, next, j)| (*next, *j) == (next_rock, i))
                .collect_vec();

            if cycle.len() > 2 {
                let a = cycle[1];
                let b = cycle[2];

                let start_count = a.0;
                let start_height = a.1;

                let count_diff = b.0 - a.0;
                let height_diff = b.1 - a.1;

                let count = 1000000000000usize;

                let cycles = (count - start_count) / count_diff;
                let missing = (count - start_count) % count_diff;

                dbg!(missing);

                let missing = states
                    .iter()
                    .find(|(count, height, next, j)| *count == start_count + missing)
                    .unwrap();

                dbg!(a, b, missing);
                let missing = missing.1 - a.1;

                dbg!(missing);

                let rocks = start_height + missing + cycles * height_diff;

                return Some(rocks);
            }

            states.push((rock_count, top, next_rock, i));
        } else {
            current_rock
                .iter_mut()
                .for_each(|(x, y)| (*x, *y) = (*x, *y - 1));
            shift.1 -= 1;
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
