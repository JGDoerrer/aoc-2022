use std::{str::Lines, vec};

#[inline]
fn parse_stacks(lines: &mut Lines) -> Vec<Vec<char>> {
    let stack_count = 9;

    let mut stacks: Vec<_> = (0..stack_count).map(|_| vec![]).collect();

    loop {
        let line = lines.next().unwrap();

        if line.starts_with('[') {
            for i in 0..stack_count as usize {
                let index = i * 4 + 1;
                match line.chars().nth(index) {
                    Some(c) => {
                        if !c.is_whitespace() {
                            stacks[i].insert(0, c);
                        }
                    }
                    None => {}
                }
            }
        } else if line.trim().starts_with('1') {
        } else {
            break;
        }
    }
    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();

    let mut stacks = parse_stacks(&mut lines);

    for line in lines {
        let mut parts = line.split(' ');

        let count: u32 = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();

        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    Some(
        stacks
            .into_iter()
            .map(|v| v.last().unwrap().clone())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();

    let mut stacks = parse_stacks(&mut lines);

    for line in lines {
        let mut parts = line.split(' ');

        let count: u32 = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();

        let mut temp = vec![];
        for _ in 0..count {
            temp.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..count {
            stacks[to - 1].push(temp.pop().unwrap());
        }
    }

    Some(
        stacks
            .into_iter()
            .map(|v| v.last().unwrap().clone())
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
