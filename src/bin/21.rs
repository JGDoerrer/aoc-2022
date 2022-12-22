use std::collections::HashMap;

#[derive(Debug)]
enum Math {
    Number(i64),
    Human,
    Eq(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn calc(monkeys: &HashMap<&str, Math>, results: &mut HashMap<String, i64>, name: &str) -> i64 {
    match results.get(name) {
        Some(res) => *res,
        None => {
            let result = match monkeys.get(name).unwrap() {
                Math::Number(num) => *num,
                Math::Add(a, b) => calc(monkeys, results, a) + calc(monkeys, results, b),
                Math::Sub(a, b) => calc(monkeys, results, a) - calc(monkeys, results, b),
                Math::Mul(a, b) => calc(monkeys, results, a) * calc(monkeys, results, b),
                Math::Div(a, b) => calc(monkeys, results, a) / calc(monkeys, results, b),
                _ => unreachable!(),
            };
            results.insert(name.to_string(), result);
            result
        }
    }
}

fn calc2(monkeys: &HashMap<&str, Math>, results: &mut HashMap<String, i64>, name: &str) -> i64 {
    match results.get(name) {
        Some(res) => *res,
        None => {
            let result = match monkeys.get(name).unwrap() {
                Math::Number(num) => *num,
                Math::Add(a, b) => calc(monkeys, results, a) + calc(monkeys, results, b),
                Math::Sub(a, b) => calc(monkeys, results, a) - calc(monkeys, results, b),
                Math::Mul(a, b) => calc(monkeys, results, a) * calc(monkeys, results, b),
                Math::Div(a, b) => calc(monkeys, results, a) / calc(monkeys, results, b),
                Math::Eq(a, b) => {
                    if contains_human(monkeys, a) {
                        let other = calc(monkeys, results, b);
                        calc3(monkeys, results, a, other)
                    } else {
                        let other = calc(monkeys, results, a);
                        calc3(monkeys, results, b, other)
                    }
                }
                _ => unreachable!(),
            };
            results.insert(name.to_string(), result);
            result
        }
    }
}

fn calc3(
    monkeys: &HashMap<&str, Math>,
    results: &mut HashMap<String, i64>,
    name: &str,
    needed: i64,
) -> i64 {
    match results.get(name) {
        Some(res) => *res,
        None => match monkeys.get(name).unwrap() {
            Math::Number(num) => *num,
            Math::Add(a, b) => {
                if contains_human(monkeys, a) {
                    let other = calc(monkeys, results, b);
                    calc3(monkeys, results, a, needed - other)
                } else {
                    let other = calc(monkeys, results, a);
                    calc3(monkeys, results, b, needed - other)
                }
            }
            Math::Sub(a, b) => {
                if contains_human(monkeys, a) {
                    let other = calc(monkeys, results, b);
                    calc3(monkeys, results, a, needed + other)
                } else {
                    let other = calc(monkeys, results, a);
                    calc3(monkeys, results, b, other - needed)
                }
            }
            Math::Mul(a, b) => {
                if contains_human(monkeys, a) {
                    let other = calc(monkeys, results, b);
                    calc3(monkeys, results, a, needed / other)
                } else {
                    let other = calc(monkeys, results, a);
                    calc3(monkeys, results, b, needed / other)
                }
            }
            Math::Div(a, b) => {
                if contains_human(monkeys, a) {
                    let other = calc(monkeys, results, b);
                    calc3(monkeys, results, a, needed * other)
                } else {
                    let other = calc(monkeys, results, a);
                    calc3(monkeys, results, b, needed * other)
                }
            }
            Math::Human => needed,
            _ => unreachable!(),
        },
    }
}

fn contains_human(monkeys: &HashMap<&str, Math>, name: &str) -> bool {
    if name == "humn" {
        true
    } else {
        match monkeys.get(name).unwrap() {
            Math::Add(a, b)
            | Math::Sub(a, b)
            | Math::Mul(a, b)
            | Math::Div(a, b)
            | Math::Eq(a, b) => contains_human(monkeys, a) || contains_human(monkeys, b),
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, math) = line.split_once(':').unwrap();
        let math = math.trim();

        let math = match math.parse() {
            Ok(num) => Math::Number(num),
            Err(_) => match math.chars().nth(5).unwrap() {
                '+' => Math::Add(math[0..4].to_string(), math[7..11].to_string()),
                '-' => Math::Sub(math[0..4].to_string(), math[7..11].to_string()),
                '*' => Math::Mul(math[0..4].to_string(), math[7..11].to_string()),
                '/' => Math::Div(math[0..4].to_string(), math[7..11].to_string()),
                _ => unreachable!(),
            },
        };

        monkeys.insert(name, math);
    }

    let mut results = HashMap::new();

    let result = calc2(&monkeys, &mut results, "root");

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, math) = line.split_once(':').unwrap();
        let math = math.trim();

        let math = match name {
            "humn" => Math::Human,
            "root" => Math::Eq(math[0..4].to_string(), math[7..11].to_string()),
            _ => match math.parse() {
                Ok(num) => Math::Number(num),
                Err(_) => match math.chars().nth(5).unwrap() {
                    '+' => Math::Add(math[0..4].to_string(), math[7..11].to_string()),
                    '-' => Math::Sub(math[0..4].to_string(), math[7..11].to_string()),
                    '*' => Math::Mul(math[0..4].to_string(), math[7..11].to_string()),
                    '/' => Math::Div(math[0..4].to_string(), math[7..11].to_string()),
                    _ => unreachable!(),
                },
            },
        };

        monkeys.insert(name, math);
    }

    let mut results = HashMap::new();

    let result = calc2(&monkeys, &mut results, "root");
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
