use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

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

fn costs(valves: &HashMap<String, (u32, Vec<String>)>, current: &String) -> HashMap<String, u32> {
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
            let other_cost = costs.get(&valve).unwrap();

            let new_cost = (*old_cost).min(*other_cost + 1);

            if new_cost < *old_cost {
                costs.insert(next.to_string(), new_cost);
            }

            queue.push(Reverse((new_cost, next.to_string())));
        }
    }

    costs
}

fn part_two_(
    valves: &HashMap<String, (u32, Vec<String>)>,
    time_left: u32,
    pressure: u32,
    current: String,
    elephant: String,
    open: Vec<String>,
    mut best: u32,
) -> u32 {
    if time_left == 0 {
        return best.max(pressure);
    }

    if valves.iter().all(|(v, _)| open.contains(v)) {
        return best.max(
            pressure + time_left * open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>(),
        );
    }

    let non_zero_valves: u32 = valves
        .iter()
        .filter(|(_, (p, _))| *p != 0)
        .map(|(_, (p, _))| p)
        .sum();
    let potential = pressure + time_left * non_zero_valves;

    if potential < best {
        return best;
    }

    let pressure = pressure + open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();

    let _my_costs = costs(valves, &current);
    let _el_costs = costs(valves, &elephant);

    if !open.contains(&current) {
        let mut new = open.clone();
        new.push(current.clone());
        for valve2 in &valves.get(&elephant).unwrap().1 {
            best = part_two_(
                valves,
                time_left - 1,
                pressure,
                current.clone(),
                valve2.clone(),
                new.clone(),
                best,
            );
        }
    }

    if !open.contains(&elephant) {
        let mut new = open.clone();
        new.push(elephant.clone());
        for valve2 in &valves.get(&current).unwrap().1 {
            best = part_two_(
                valves,
                time_left - 1,
                pressure,
                valve2.clone(),
                elephant.clone(),
                new.clone(),
                best,
            );
        }
    }

    if !open.contains(&current) && !open.contains(&elephant) && current != elephant {
        let mut new = open.clone();
        new.push(current.clone());
        new.push(elephant.clone());
        best = part_two_(
            valves,
            time_left - 1,
            pressure,
            current.clone(),
            elephant.clone(),
            new.clone(),
            best,
        );
    }

    let count = valves
        .get(&current)
        .unwrap()
        .1
        .iter()
        .cartesian_product(valves.get(&elephant).unwrap().1.iter())
        .count();

    for (i, (valve1, valve2)) in valves
        .get(&current)
        .unwrap()
        .1
        .iter()
        .cartesian_product(valves.get(&elephant).unwrap().1.iter())
        .enumerate()
    {
        if time_left >= 24 {
            println!("{best}, {i}/{count}");
        }
        best = part_two_(
            valves,
            time_left - 1,
            pressure,
            valve1.clone(),
            valve2.clone(),
            open.clone(),
            best,
        );
    }

    best
}

struct State<'a> {
    valves: &'a HashMap<String, (u32, Vec<String>)>,
    time_left: u32,
    pressure: u32,
    node_a: String,
    node_b: String,
    open: Vec<String>,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let open_a = self
            .open
            .iter()
            .map(|v| self.valves.get(v).unwrap().0)
            .sum::<u32>();
        let open_b = other
            .open
            .iter()
            .map(|v| other.valves.get(v).unwrap().0)
            .sum::<u32>();

        let result_a = self.pressure + self.time_left * open_a;
        let result_b = other.pressure + other.time_left * open_b;

        result_a.cmp(&result_b)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Eq for State<'a> {}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = parse_valves(input);

    let non_zero_valves = valves
        .iter()
        .filter(|(_, (p, _))| *p != 0)
        .map(|(name, _)| name.clone())
        .collect_vec();
    let zero_valves = valves
        .iter()
        .filter(|(_, (p, _))| *p == 0)
        .map(|(name, _)| name.clone())
        .collect_vec();

    // Some(part_two_(
    //     &valves,
    //     26,
    //     0,
    //     "AA".to_string(),
    //     "AA".to_string(),
    //     zero_valves,
    //     0,
    // ))

    // let non_zero_valves: u32 = valves
    //     .iter()
    //     .filter(|(_, (p, _))| *p != 0)
    //     .map(|(_, (p, _))| p)
    //     .sum();

    // let mut candidates = BinaryHeap::new();
    // candidates.push(State {
    //     valves: &valves,
    //     time_left: 26,
    //     pressure: 0,
    //     node_a: "AA".to_string(),
    //     node_b: "AA".to_string(),
    //     open: zero_valves,
    // });

    // let mut best = 2200;

    // while let Some(candidate) = candidates.pop() {
    //     if valves.iter().all(|(v, _)| candidate.open.contains(v)) {
    //         let pressure = candidate.pressure
    //             + candidate.time_left
    //                 * candidate
    //                     .open
    //                     .iter()
    //                     .map(|v| valves.get(v).unwrap().0)
    //                     .sum::<u32>();
    //         if best < pressure {
    //             best = pressure;

    //             let mut vec = candidates.into_vec();
    //             vec.retain(|candidate| {
    //                 let potential = candidate.pressure + non_zero_valves * candidate.time_left;
    //                 potential > best
    //             });
    //             candidates = BinaryHeap::from(vec);

    //             dbg!(best);
    //             dbg!(candidates.len());
    //         }

    //         continue;
    //     }

    //     if candidate.time_left == 0 {
    //         if best < candidate.pressure {
    //             best = candidate.pressure;

    //             let mut vec = candidates.into_vec();
    //             vec.retain(|candidate| {
    //                 let potential = candidate.pressure + non_zero_valves * candidate.time_left;
    //                 potential > best
    //             });
    //             candidates = BinaryHeap::from(vec);

    //             dbg!(best);
    //             dbg!(candidates.len());
    //         }

    //         continue;
    //     }

    //     let total = candidate.pressure
    //         + candidate.time_left
    //             * candidate
    //                 .open
    //                 .iter()
    //                 .map(|v| valves.get(v).unwrap().0)
    //                 .sum::<u32>();

    //     if best < total {
    //         best = total;

    //         let mut vec = candidates.into_vec();
    //         vec.retain(|candidate| {
    //             let potential = candidate.pressure + non_zero_valves * candidate.time_left;
    //             potential > best
    //         });
    //         candidates = BinaryHeap::from(vec);

    //         dbg!(best);
    //         dbg!(candidates.len());
    //     }

    //     let next_pressure = candidate.pressure
    //         + candidate
    //             .open
    //             .iter()
    //             .map(|v| valves.get(v).unwrap().0)
    //             .sum::<u32>();

    //     let open = &candidate.open;
    //     let time_left = candidate.time_left - 1;
    //     let a = &candidate.node_a;
    //     let b = &candidate.node_b;

    //     let mut neighbours = vec![];

    //     if !open.contains(&a) {
    //         let mut new = open.clone();
    //         new.push(a.clone());

    //         for b in &valves.get(b).unwrap().1 {
    //             neighbours.push(State {
    //                 valves: &valves,
    //                 time_left,
    //                 pressure: next_pressure,
    //                 node_a: a.clone(),
    //                 node_b: b.clone(),
    //                 open: new.clone(),
    //             });
    //         }
    //     }

    //     if !open.contains(&b) {
    //         let mut new = open.clone();
    //         new.push(b.clone());

    //         for a in &valves.get(a).unwrap().1 {
    //             neighbours.push(State {
    //                 valves: &valves,
    //                 time_left,
    //                 pressure: next_pressure,
    //                 node_a: a.clone(),
    //                 node_b: b.clone(),
    //                 open: new.clone(),
    //             });
    //         }
    //     }
    //     if !open.contains(&a) && !open.contains(&b) && a != b {
    //         let mut new = open.clone();
    //         new.push(a.clone());
    //         new.push(b.clone());

    //         neighbours.push(State {
    //             valves: &valves,
    //             time_left,
    //             pressure: next_pressure,
    //             node_a: a.clone(),
    //             node_b: b.clone(),
    //             open: new,
    //         });
    //     }

    //     for a in &valves.get(&candidate.node_a).unwrap().1 {
    //         for b in &valves.get(&candidate.node_b).unwrap().1 {
    //             neighbours.push(State {
    //                 valves: &valves,
    //                 time_left,
    //                 pressure: next_pressure,
    //                 node_a: a.clone(),
    //                 node_b: b.clone(),
    //                 open: open.clone(),
    //             });
    //         }
    //     }

    //     neighbours.retain(|candidate| {
    //         let potential = candidate.pressure + non_zero_valves * candidate.time_left;
    //         potential > best
    //     });

    //     for n in neighbours {
    //         candidates.push(n);
    //     }

    //     // candidates.sort_by(|a, b| {
    //     //     let result_a = a.pressure
    //     //         + a.time_left * a.open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();
    //     //     let result_b = b.pressure
    //     //         + b.time_left * b.open.iter().map(|v| valves.get(v).unwrap().0).sum::<u32>();

    //     //     result_a.cmp(&result_b)
    //     // });
    // }

    // Some(best)

    // dbg!(non_zero_valves);

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
