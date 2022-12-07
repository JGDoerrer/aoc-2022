use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .bytes()
            .tuple_windows()
            .position(|(a, b, c, d)| a != b && a != c && a != d && b != c && b != d && c != d)
            .unwrap() as u32
            + 4,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();

    Some(
        chars
            .windows(14)
            .position(|window| {
                window
                    .iter()
                    .filter(|c| window.iter().filter(|a| a == c).count() > 1)
                    .next()
                    == None
            })
            .unwrap() as u32
            + 14,
    )
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
