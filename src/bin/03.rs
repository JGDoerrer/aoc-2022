use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let c = first
            .chars()
            .filter(|&c| second.contains(c))
            .next()
            .unwrap();

        let priority = if c.is_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 27
        };

        sum += priority;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for mut chunk in &input.lines().chunks(3) {
        let first = chunk.next().unwrap();
        let second = chunk.next().unwrap();
        let third = chunk.next().unwrap();

        let c = first
            .chars()
            .filter(|&c| second.contains(c) && third.contains(c))
            .next()
            .unwrap();

        let priority = if c.is_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 27
        };

        sum += priority;
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
