use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

fn parse_valves(input: &str) -> HashMap<&str, (u32, Vec<&str>)> {
    let mut valves = HashMap::new();

    for line in input.lines() {
        let valve = &line[6..8];
        let (flow, connections) = line[23..].split_once(';').unwrap();
        let flow = flow.parse().unwrap();
        let connections = connections[23..].split(',').map(|s| s.trim()).collect_vec();

        valves.insert(valve, (flow, connections));
    }

    valves
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = parse_valves(input);

    let mut states = vec![(0, "AA", vec![])];

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

fn costs<'a>(
    valves: &'a HashMap<&'a str, (u32, Vec<&'a str>)>,
    current: &'a str,
) -> HashMap<&'a str, u32> {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    queue.push(Reverse((0, current.clone())));
    costs.insert(current.clone(), 0);
    while let Some(Reverse((_, valve))) = queue.pop() {
        if !visited.insert(valve.to_string()) {
            continue;
        }
        for next in &valves.get(&valve).unwrap().1 {
            let old_cost = costs.get(next).unwrap_or(&u32::MAX);
            let current_cost = costs.get(&valve).unwrap();
            let new_cost = (*old_cost).min(*current_cost + 1);
            if new_cost < *old_cost {
                costs.insert(next, new_cost);
            }
            queue.push(Reverse((new_cost, next)));
        }
    }
    costs
}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = parse_valves(input);

    let max_flow: u32 = valves
        .iter()
        .filter(|(_, (p, _))| *p != 0)
        .map(|(_, (p, _))| p)
        .sum();

    let non_zero_valves = valves
        .iter()
        .filter(|(_, (p, _))| *p != 0)
        .map(|(name, _)| name.clone())
        .collect_vec();

    let valves: HashMap<_, _> = non_zero_valves
        .iter()
        .chain([&"AA"])
        .map(|valve| {
            let other_valves = non_zero_valves.iter().filter(|a| *a != valve).collect_vec();
            let flow = valves.get(valve).unwrap().0;
            let costs = costs(&valves, valve);

            (
                valve,
                (
                    flow,
                    other_valves
                        .into_iter()
                        .map(|v| (v, *costs.get(v).unwrap()))
                        .collect_vec(),
                ),
            )
        })
        .collect();

    let mut states = vec![(0, vec![], "AA", 0, "AA", 0)];

    for i in (0..26).rev() {
        let mut new_states = vec![];

        for (pressure, open, current, c_time, elephant, e_time) in states {
            let pressure = pressure + open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();
            let c_time = if c_time > 0 { c_time - 1 } else { 0 };
            let e_time = if e_time > 0 { e_time - 1 } else { 0 };

            let unopened_valves = non_zero_valves
                .iter()
                .filter(|v| !open.contains(*v))
                .collect_vec();

            if unopened_valves.len() == 0 {
                new_states.push((pressure, open, current, c_time, elephant, e_time));
                continue;
            }

            if c_time == 0 && e_time == 0 {
                if !open.contains(&current) && !open.contains(&elephant) && current != elephant {
                    let mut new = open.clone();
                    new.push(current);
                    new.push(elephant);

                    new_states.push((pressure, new.clone(), current, c_time, elephant, e_time));
                }

                for (valve1, time1) in valves
                    .get(&current)
                    .unwrap()
                    .1
                    .iter()
                    .filter(|(v, _)| unopened_valves.contains(&v))
                {
                    for (valve2, time2) in valves
                        .get(&elephant)
                        .unwrap()
                        .1
                        .iter()
                        .filter(|(v, _)| unopened_valves.contains(&v))
                    {
                        new_states.push((pressure, open.clone(), valve1, *time1, valve2, *time2));
                    }
                }
            }
            if c_time == 0 {
                if !open.contains(&current) {
                    let mut new = open.clone();
                    new.push(current);

                    new_states.push((pressure, new.clone(), current, c_time, elephant, e_time));
                } else {
                    for (valve1, time1) in valves
                        .get(&current)
                        .unwrap()
                        .1
                        .iter()
                        .filter(|(v, _)| unopened_valves.contains(&v))
                    {
                        new_states.push((pressure, open.clone(), valve1, *time1, elephant, e_time));
                    }
                }
            } else if e_time == 0 {
                if !open.contains(&elephant) {
                    let mut new = open.clone();
                    new.push(elephant);

                    new_states.push((pressure, new.clone(), current, c_time, elephant, e_time));
                } else {
                    for (valve2, time2) in valves
                        .get(&elephant)
                        .unwrap()
                        .1
                        .iter()
                        .filter(|(v, _)| unopened_valves.contains(&v))
                    {
                        new_states.push((pressure, open.clone(), current, c_time, valve2, *time2));
                    }
                }
            } else {
                new_states.push((pressure, open, current, c_time, elephant, e_time));
            }
        }

        let best = new_states
            .iter()
            .max_by(|a, b| {
                (a.0 + i * a.1.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>())
                    .cmp(&(b.0 + i * b.1.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>()))
            })
            .unwrap();

        let best = best.0 + i * best.1.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();

        new_states.retain(|candidate| {
            let potential = candidate.0 + max_flow * i;
            potential >= best
        });
        new_states.sort();
        new_states.dedup();
        new_states.sort_by(|a, b| {
            (a.0 + i * a.1.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>())
                .cmp(&(b.0 + i * b.1.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>()))
        });
        dbg!(new_states.len());
        states = new_states.into_iter().rev().take(100000).collect_vec();
    }

    let max = states
        .into_iter()
        .max_by(|(a, _, _, _, _, _), (b, _, _, _, _, _)| a.cmp(b))
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
