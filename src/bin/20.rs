use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut numbers: Vec<i32> = vec![];

    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }

    let mut numbers = numbers.into_iter().enumerate().collect_vec();

    for (i, number) in numbers.clone() {
        let position = numbers.iter().position(|(j, _)| *j == i).unwrap();
        let new_position =
            ((position as i32 + number + numbers.len() as i32 - 1 + numbers.len() as i32 - 1)
                % (numbers.len() as i32 - 1)) as usize;

        if position < new_position {
            numbers.remove(position);
            numbers.insert(new_position, (i, number));
        } else {
            numbers.remove(position);
            numbers.insert(new_position, (i, number));
        }
    }

    let zero = numbers.iter().position(|(_, n)| *n == 0).unwrap();
    let mut sum = 0;

    for delta in [1000, 2000, 3000] {
        let index = (zero + delta) % (numbers.len());
        sum += numbers[index].1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let key = 811589153;

    let mut numbers: Vec<i64> = vec![];

    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }

    let mut numbers = numbers
        .into_iter()
        .map(|n| n * key)
        .enumerate()
        .collect_vec();

    let len = numbers.len() as i64 - 1;
    for _ in 0..10 {
        for i in 0..numbers.len() {
            let position = numbers.iter().position(|(j, _)| *j == i).unwrap();
            let number = numbers[position].1;
            let new_position = (((position as i64 + number + len) % len + len) % len) as usize;

            if position < new_position {
                numbers.remove(position);
                numbers.insert(new_position, (i, number));
            } else {
                numbers.remove(position);
                numbers.insert(new_position, (i, number));
            }
        }
    }

    let zero = numbers.iter().position(|(_, n)| *n == 0).unwrap();
    let mut sum = 0;

    for delta in [1000, 2000, 3000] {
        let index = (zero + delta) % (numbers.len());
        sum += numbers[index].1;
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
