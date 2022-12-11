use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];

    for lines in &input.lines().chunks(7) {
        let lines: Vec<_> = lines.collect();

        let items: Vec<usize> = lines[1][18..]
            .split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();

        let operation = match &lines[2][19..] {
            "old * old" => Operation::Square,
            str => match str.chars().nth(4).unwrap() {
                '+' => Operation::Add(str[6..].parse().unwrap()),
                '*' => Operation::Mult(str[6..].parse().unwrap()),
                _ => unreachable!(),
            },
        };

        let test: usize = lines[3][21..].parse().unwrap();

        let test_true: usize = lines[4][29..].parse().unwrap();
        let test_false: usize = lines[5][30..].parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        })
    }

    monkeys
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.remove(0);
                monkeys[i].inspections += 1;

                match monkeys[i].operation {
                    Operation::Add(a) => item += a,
                    Operation::Mult(a) => item *= a,
                    Operation::Square => item *= item,
                }

                item = item / 3;

                let index = match item % monkeys[i].test {
                    0 => monkeys[i].test_true,
                    _ => monkeys[i].test_false,
                };

                monkeys[index].items.push(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();

    Some(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    let tests: usize = monkeys.iter().map(|m| m.test).product();
    dbg!(tests);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.remove(0);
                monkeys[i].inspections += 1;

                match monkeys[i].operation {
                    Operation::Add(a) => item += a,
                    Operation::Mult(a) => item *= a,
                    Operation::Square => item *= item,
                }

                item %= tests;

                let index = match item % monkeys[i].test {
                    0 => monkeys[i].test_true,
                    _ => monkeys[i].test_false,
                };

                monkeys[index].items.push(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();

    Some(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
