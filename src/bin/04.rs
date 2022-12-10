pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let (l_from, l_to) = left.split_once('-').unwrap();
        let (r_from, r_to) = right.split_once('-').unwrap();

        let (l_from, l_to): (u32, u32) = (l_from.parse().unwrap(), l_to.parse().unwrap());
        let (r_from, r_to): (u32, u32) = (r_from.parse().unwrap(), r_to.parse().unwrap());

        if (l_from >= r_from && l_to <= r_to) || (l_from <= r_from && l_to >= r_to) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let (l_from, l_to) = left.split_once('-').unwrap();
        let (r_from, r_to) = right.split_once('-').unwrap();

        let (l_from, l_to): (u32, u32) = (l_from.parse().unwrap(), l_to.parse().unwrap());
        let (r_from, r_to): (u32, u32) = (r_from.parse().unwrap(), r_to.parse().unwrap());

        if (l_from <= r_from && l_to >= r_from) || (r_from <= l_from && r_to >= l_from) {
            count += 1;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
