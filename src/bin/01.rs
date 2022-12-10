pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut current = 0;

    for line in input.lines() {
        match line.trim().parse::<u32>() {
            Ok(v) => current += v,
            _ => {
                max = max.max(current);
                current = 0;
            }
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = vec![0];

    for line in input.lines() {
        match line.trim().parse::<i32>() {
            Ok(v) => *elves.last_mut().unwrap() += v,
            _ => elves.push(0),
        }
    }

    elves.sort();
    Some(elves.iter().rev().take(3).sum::<i32>() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
