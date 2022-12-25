use itertools::{EitherOrBoth, Itertools};

fn add(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut carry = 0;

    for digit in a.into_iter().rev().zip_longest(b.into_iter().rev()) {
        match digit {
            EitherOrBoth::Both(a, b) => {
                let mut sum = a + b + carry;
                if sum == 3 {
                    carry = 1;
                    sum = -2;
                } else if sum == 4 {
                    carry = 1;
                    sum = -1;
                } else if sum == 5 {
                    carry = 1;
                    sum = 0;
                } else if sum == -3 {
                    carry = -1;
                    sum = 2;
                } else if sum == -4 {
                    carry = -1;
                    sum = 1;
                } else if sum == -5 {
                    carry = -1;
                    sum = 0;
                } else {
                    carry = 0;
                }
                result.insert(0, sum);
            }
            EitherOrBoth::Left(a) | EitherOrBoth::Right(a) => {
                let mut sum = a + carry;
                if sum == 3 {
                    carry = 1;
                    sum = -2;
                } else if sum == -3 {
                    carry = -1;
                    sum = 2;
                } else {
                    carry = 0;
                }
                result.insert(0, sum);
            }
        }
    }

    if carry != 0 {
        result.insert(0, carry);
    }

    result
}

pub fn part_one(input: &str) -> Option<String> {
    let mut numbers = vec![];
    for line in input.lines() {
        let mut number = vec![];

        for char in line.chars().rev() {
            let digit = match char {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };

            number.insert(0, digit);
        }

        numbers.push(number);
    }

    let mut _numbers = vec![
        vec![0],
        vec![1],
        vec![2],
        vec![1, -2],
        vec![1, -1],
        vec![1, 0],
        vec![1, 1],
        vec![1, 2],
        vec![2, -2],
        vec![2, -1],
        vec![2, 0],
    ];

    let mut sum = vec![0];

    for number in numbers {
        sum = add(sum, number);
    }

    let output = sum
        .into_iter()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .collect();

    Some(output)
}

pub fn part_two(_: &str) -> Option<bool> {
    Some(true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), Some(true));
    }
}
