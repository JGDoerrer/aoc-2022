use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
enum List {
    List(Vec<List>),
    Value(u32),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            List::List(list_a) => match other {
                List::List(list_b) => {
                    match list_a
                        .iter()
                        .zip(list_b)
                        .map(|(a, b)| a.cmp(b))
                        .filter(|o| *o != Ordering::Equal)
                        .next()
                    {
                        Some(o) => o,
                        None => list_a.len().cmp(&list_b.len()),
                    }
                }
                List::Value(value_b) => self.cmp(&&List::List(vec![List::Value(*value_b)])),
            },
            List::Value(value_a) => match other {
                List::List(_) => List::List(vec![List::Value(*value_a)]).cmp(other),
                List::Value(value_b) => value_a.cmp(value_b),
            },
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for List {}

fn parse_list(line: &str) -> List {
    if line.starts_with('[') {
        let mut list = vec![];
        let mut level = 0;
        let mut value = String::new();

        for char in line.chars() {
            match char {
                '[' => {
                    if level != 0 {
                        value.push(char);
                    }
                    level += 1;
                }
                ']' => {
                    if level == 1 {
                        if !value.is_empty() {
                            let item = parse_list(value.as_str());
                            list.push(item);
                        }
                        value.clear();
                    } else {
                        value.push(char);
                    }
                    level -= 1;
                }
                ',' => {
                    if level == 1 {
                        if !value.is_empty() {
                            let item = parse_list(value.as_str());
                            list.push(item);
                        }
                        value.clear();
                    } else {
                        value.push(char)
                    }
                }
                _ => value.push(char),
            }
        }

        List::List(list)
    } else {
        List::Value(line.parse().unwrap())
    }
}

fn parse_lists(input: &str) -> Vec<List> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_lists(input)
            .iter()
            .tuples()
            .enumerate()
            .filter(|(_, (a, b))| a < b)
            .map(|(i, _)| i + 1)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lists = parse_lists(input);
    lists.push(parse_list("[[2]]"));
    lists.push(parse_list("[[6]]"));

    lists.sort();

    let index_1 = lists
        .iter()
        .position(|list| list == &parse_list("[[2]]"))
        .unwrap()
        + 1;

    let index_2 = lists
        .iter()
        .position(|list| list == &parse_list("[[6]]"))
        .unwrap()
        + 1;

    Some(index_1 as u32 * index_2 as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
