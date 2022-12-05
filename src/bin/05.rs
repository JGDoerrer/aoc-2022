use std::vec;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut stacks = vec![
        vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
        vec!['T', 'B', 'M', 'Z', 'R'],
        vec!['Z', 'L', 'C', 'H', 'N', 'S'],
        vec!['S', 'C', 'F', 'J'],
        vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
        vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
        vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
        vec!['M', 'Z', 'R'],
        vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
    ];

    loop {
        let line = lines.next().unwrap();

        if line.starts_with('[') {
        } else if line.trim().starts_with('1') {
        } else {
            break;
        }
    }

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();

        let count: u32 = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    dbg!(stacks);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut stacks = vec![
        vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
        vec!['T', 'B', 'M', 'Z', 'R'],
        vec!['Z', 'L', 'C', 'H', 'N', 'S'],
        vec!['S', 'C', 'F', 'J'],
        vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
        vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
        vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
        vec!['M', 'Z', 'R'],
        vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
    ];

    loop {
        let line = lines.next().unwrap();

        if line.starts_with('[') {
        } else if line.trim().starts_with('1') {
        } else {
            break;
        }
    }

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();

        let count: u32 = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        let mut temp = vec![];
        for _ in 0..count {
            temp.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..count {
            stacks[to - 1].push(temp.pop().unwrap());
        }
    }

    dbg!(stacks);
    None
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
