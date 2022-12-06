use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .char_indices()
            .tuple_windows()
            .filter(|((_, a), (_, b), (_, c), (_, d))| {
                a != b && a != c && a != d && b != c && b != d && c != d
            })
            .next()
            .unwrap()
            .3
             .0 as u32
            + 1,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    let mut index = 14;

    for window in chars.windows(14) {
        if window
            .iter()
            .filter(|c| window.iter().filter(|a| a == c).count() > 1)
            .next()
            == None
        {
            break;
        }
        index += 1;
    }

    Some(index)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
