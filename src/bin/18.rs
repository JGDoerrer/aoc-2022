use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let cubes: Vec<(i32, i32, i32)> = {
        let mut cubes = vec![];
        for line in input.lines() {
            let (x, y, z) = line
                .split(',')
                .map(|c| c.parse().unwrap())
                .tuples()
                .next()
                .unwrap();
            cubes.push((x, y, z));
        }
        cubes
    };

    let mut total = 0;

    for (x, y, z) in &cubes {
        let mut surface = 6;

        if cubes.contains(&(*x - 1, *y, *z)) {
            surface -= 1;
        }
        if cubes.contains(&(*x + 1, *y, *z)) {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y - 1, *z)) {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y + 1, *z)) {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y, *z - 1)) {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y, *z + 1)) {
            surface -= 1;
        }

        total += surface;
    }

    Some(total)
}

fn is_outside(
    cubes: &Vec<(i32, i32, i32)>,
    inner: &mut HashSet<(i32, i32, i32)>,
    outer: &mut HashSet<(i32, i32, i32)>,
    pos: (i32, i32, i32),
) -> bool {
    if inner.contains(&pos) {
        return false;
    }
    if outer.contains(&pos) {
        return true;
    }

    let max = 5000;

    let mut done = vec![];
    let mut queue = vec![pos];

    let min_x = cubes.iter().map(|pos| pos.0).min().unwrap();
    let min_y = cubes.iter().map(|pos| pos.1).min().unwrap();
    let min_z = cubes.iter().map(|pos| pos.2).min().unwrap();
    let max_x = cubes.iter().map(|pos| pos.0).max().unwrap();
    let max_y = cubes.iter().map(|pos| pos.1).max().unwrap();
    let max_z = cubes.iter().map(|pos| pos.2).max().unwrap();

    while !queue.is_empty() {
        let (x, y, z) = queue.remove(0);
        if done.len() >= max
            || outer.contains(&(x, y, z))
            || x < min_x
            || y < min_y
            || z < min_z
            || x > max_x
            || y > max_y
            || z > max_z
        {
            for pos in done {
                outer.insert(pos);
            }
            for pos in queue {
                outer.insert(pos);
            }
            return true;
        }

        if inner.contains(&(x, y, z)) {
            for pos in done {
                inner.insert(pos);
            }
            for pos in queue {
                inner.insert(pos);
            }
            return false;
        }

        if !cubes.contains(&(x - 1, y, z))
            && !queue.contains(&(x - 1, y, z))
            && !done.contains(&(x - 1, y, z))
        {
            queue.push((x - 1, y, z));
        }
        if !cubes.contains(&(x + 1, y, z))
            && !queue.contains(&(x + 1, y, z))
            && !done.contains(&(x + 1, y, z))
        {
            queue.push((x + 1, y, z));
        }
        if !cubes.contains(&(x, y - 1, z))
            && !queue.contains(&(x, y - 1, z))
            && !done.contains(&(x, y - 1, z))
        {
            queue.push((x, y - 1, z));
        }
        if !cubes.contains(&(x, y + 1, z))
            && !queue.contains(&(x, y + 1, z))
            && !done.contains(&(x, y + 1, z))
        {
            queue.push((x, y + 1, z));
        }
        if !cubes.contains(&(x, y, z - 1))
            && !queue.contains(&(x, y, z - 1))
            && !done.contains(&(x, y, z - 1))
        {
            queue.push((x, y, z - 1));
        }
        if !cubes.contains(&(x, y, z + 1))
            && !queue.contains(&(x, y, z + 1))
            && !done.contains(&(x, y, z + 1))
        {
            queue.push((x, y, z + 1));
        }

        done.push((x, y, z));
    }

    for pos in done {
        inner.insert(pos);
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let cubes: Vec<(i32, i32, i32)> = {
        let mut cubes = vec![];
        for line in input.lines() {
            let (x, y, z) = line
                .split(',')
                .map(|c| c.parse().unwrap())
                .tuples()
                .next()
                .unwrap();
            cubes.push((x, y, z));
        }
        cubes
    };

    let mut total = 0;

    let mut inner = HashSet::new();
    let mut outer = HashSet::new();

    for (x, y, z) in &cubes {
        let mut surface = 6;

        if cubes.contains(&(*x - 1, *y, *z))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x - 1, *y, *z))
        {
            surface -= 1;
        }
        if cubes.contains(&(*x + 1, *y, *z))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x + 1, *y, *z))
        {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y - 1, *z))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x, *y - 1, *z))
        {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y + 1, *z))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x, *y + 1, *z))
        {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y, *z - 1))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x, *y, *z - 1))
        {
            surface -= 1;
        }
        if cubes.contains(&(*x, *y, *z + 1))
            || !is_outside(&cubes, &mut inner, &mut outer, (*x, *y, *z + 1))
        {
            surface -= 1;
        }

        total += surface;
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
