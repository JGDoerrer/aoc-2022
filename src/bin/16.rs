use std::collections::HashMap;

use itertools::Itertools;

fn parse_valves(input: &str) -> HashMap<String, (u32, Vec<String>)> {
    let mut valves = HashMap::new();

    for line in input.lines() {
        let valve = line[6..8].to_string();
        let (flow, connections) = line[23..].split_once(';').unwrap();
        let flow = flow.parse().unwrap();
        let connections = connections[23..]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect_vec();

        valves.insert(valve, (flow, connections));
    }

    valves
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = parse_valves(input);

    let mut states = vec![(0, "AA".to_string(), vec![])];

    for _ in 0..30 {
        let mut new_states = vec![];
        for (pressure, current, open) in states {
            let pressure = pressure + open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();

            if !open.contains(&current) {
                let mut new = open.clone();
                new.push(current.clone());
                new_states.push((pressure, current.clone(), new));
            }

            for valve in &valves.get(&current).unwrap().1 {
                new_states.push((pressure, valve.clone(), open.clone()));
            }
        }
        dbg!(new_states.len());
        new_states.sort();
        new_states.dedup();
        states = new_states.into_iter().rev().take(100).collect_vec();
    }

    let max = states
        .into_iter()
        .max_by(|(a, _, _), (b, _, _)| a.cmp(b))
        .unwrap()
        .0;

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = parse_valves(input);

    let zero_valves = valves
        .iter()
        .filter(|(_, (p, _))| *p == 0)
        .map(|(name, _)| name.clone())
        .collect_vec();

    let non_zero_valves: u32 = valves
        .iter()
        .filter(|(_, (p, _))| *p != 0)
        .map(|(_, (p, _))| p)
        .sum();
    let mut states = vec![(0, "AA".to_string(), "AA".to_string(), zero_valves.clone())];

    for i in (0..26).rev() {
        let mut new_states = vec![];
        for (pressure, current, elephant, open) in states {
            let pressure = pressure + open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();

            if valves.iter().all(|(valve, _)| open.contains(valve)) {
                new_states.push((pressure, current, elephant, open));
                continue;
            }

            if !open.contains(&current) {
                let mut new = open.clone();
                new.push(current.clone());

                for valve2 in &valves.get(&elephant).unwrap().1 {
                    new_states.push((pressure, current.clone(), valve2.clone(), new.clone()));
                }
            }
            if !open.contains(&elephant) {
                let mut new = open.clone();
                new.push(elephant.clone());

                for valve1 in &valves.get(&current).unwrap().1 {
                    new_states.push((pressure, valve1.clone(), elephant.clone(), new.clone()));
                }
            }
            if !open.contains(&current) && !open.contains(&elephant) && current != elephant {
                let mut new = open.clone();
                new.push(current.clone());
                new.push(elephant.clone());

                new_states.push((pressure, current.clone(), elephant.clone(), new.clone()));
            }

            for valve1 in &valves.get(&current).unwrap().1 {
                for valve2 in &valves.get(&elephant).unwrap().1 {
                    new_states.push((pressure, valve1.clone(), valve2.clone(), open.clone()));
                }
            }
        }

        dbg!(new_states.len());
        new_states.retain(|candidate| {
            let potential = candidate.0 + non_zero_valves * i;
            potential > 1000
        });
        new_states.sort_by(|a, b| {
            (a.0 + i * a.3.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>())
                .cmp(&(b.0 + i * b.3.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>()))
        });
        new_states.dedup();
        states = new_states.into_iter().rev().take(100000).collect_vec();
    }

    let max = states
        .into_iter()
        .max_by(|(a, _, _, _), (b, _, _, _)| a.cmp(b))
        .unwrap()
        .0;
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
