pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let opponent = line.chars().next().unwrap();
        let me = line.chars().skip(2).next().unwrap();

        score += match opponent {
            'A' => match me {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => unreachable!(),
            },
            'B' => match me {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => unreachable!(),
            },
            'C' => match me {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let opponent = line.chars().next().unwrap();
        let result = line.chars().skip(2).next().unwrap();

        score += match result {
            'X' => match opponent {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            },
            'Y' => match opponent {
                'A' => 4,
                'B' => 5,
                'C' => 6,
                _ => unreachable!(),
            },
            'Z' => match opponent {
                'A' => 8,
                'B' => 9,
                'C' => 7,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
