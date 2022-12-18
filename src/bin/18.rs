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
    inner: &mut Vec<(i32, i32, i32)>,
    outer: &mut Vec<(i32, i32, i32)>,
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

    while !queue.is_empty() {
        let (x, y, z) = queue.remove(0);
        if done.len() >= max {
            outer.push(pos);
            return true;
        }

        if outer.contains(&(x, y, z)) {
            return true;
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

    inner.append(&mut done);
    inner.sort();
    inner.dedup();

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

    let mut inner = vec![];
    let mut outer = vec![];

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
