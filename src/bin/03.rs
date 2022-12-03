use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        for c in first.chars() {
            if second.contains(c) {
                let priority = if c.is_lowercase() {
                    c as u32 - 'a' as u32 + 1
                } else {
                    c as u32 - 'A' as u32 + 27
                };

                sum += priority;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let mut lines = vec![];

    for line in input.lines() {
        lines.push(line);

        if lines.len() == 3 {
            let mut chars2 = HashSet::new();
            let mut chars3 = HashSet::new();

            for c in lines[1].chars() {
                chars2.insert(c);
            }
            for c in lines[2].chars() {
                chars3.insert(c);
            }

            for c in lines[0].chars() {
                if chars2.contains(&c) && chars3.contains(&c) {
                    let priority = if c.is_lowercase() {
                        c as u32 - 'a' as u32 + 1
                    } else {
                        c as u32 - 'A' as u32 + 27
                    };

                    sum += priority;
                    break;
                }
            }

            lines.clear();
        }
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
