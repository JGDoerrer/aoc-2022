use std::cmp::Ordering;

use itertools::Itertools;

fn parse_blueprints(input: &str) -> Vec<(u32, u32, (u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|line| {
            let a = line.split_once(':').unwrap().1;
            let ore = a[22..23].parse().unwrap();
            let clay = a[51..52].parse().unwrap();
            let o_ore = a[84..85].parse().unwrap();
            let o_clay = a[94..96].trim().parse().unwrap();

            let a = a[96..].split_once('.').unwrap().1;
            let g_ore = a[24..25].trim().parse().unwrap();
            let g_clay = a[34..36].trim().parse().unwrap();

            (ore, clay, (o_ore, o_clay), (g_ore, g_clay))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_blueprints(input);
    let mut quality = 0;

    for (i, (o_ore, c_ore, (ob_ore, ob_clay), (g_ore, g_ob))) in blueprints.into_iter().enumerate()
    {
        let start = (0, 0, 0, 0, 1, 0, 0, 0);

        let mut states = vec![start];
        let mut new_states;

        for _ in 0..24 {
            new_states = vec![];

            for (ore, clay, ob, geo, r_ore, r_clay, r_ob, r_geo) in states {
                let new_ore = ore + r_ore;
                let new_clay = clay + r_clay;
                let new_ob = ob + r_ob;
                let new_geo = geo + r_geo;

                new_states.push((
                    new_ore, new_clay, new_ob, new_geo, r_ore, r_clay, r_ob, r_geo,
                ));

                if ore >= o_ore {
                    new_states.push((
                        new_ore - o_ore,
                        new_clay,
                        new_ob,
                        new_geo,
                        r_ore + 1,
                        r_clay,
                        r_ob,
                        r_geo,
                    ));
                }
                if ore >= c_ore {
                    new_states.push((
                        new_ore - c_ore,
                        new_clay,
                        new_ob,
                        new_geo,
                        r_ore,
                        r_clay + 1,
                        r_ob,
                        r_geo,
                    ));
                }
                if ore >= ob_ore && clay >= ob_clay {
                    new_states.push((
                        new_ore - ob_ore,
                        new_clay - ob_clay,
                        new_ob,
                        new_geo,
                        r_ore,
                        r_clay,
                        r_ob + 1,
                        r_geo,
                    ));
                }
                if ore >= g_ore && ob >= g_ob {
                    new_states.push((
                        new_ore - g_ore,
                        new_clay,
                        new_ob - g_ob,
                        new_geo,
                        r_ore,
                        r_clay,
                        r_ob,
                        r_geo + 1,
                    ));
                }
            }

            new_states.sort_by(|a, b| match a.7.cmp(&b.7) {
                Ordering::Equal => match a.3.cmp(&b.3) {
                    Ordering::Equal => match a.2.cmp(&b.2) {
                        o => o,
                    },
                    o => o,
                },
                o => o,
            });
            states = new_states.into_iter().rev().take(10000).collect_vec();
        }

        let best = states.into_iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap();
        let max_geodes = best.3;

        quality += (i + 1) as u32 * max_geodes;
    }

    Some(quality)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_blueprints(input);
    let mut quality = 1;

    for (o_ore, c_ore, (ob_ore, ob_clay), (g_ore, g_ob)) in blueprints.into_iter().take(3) {
        let start = (0, 0, 0, 0, 1, 0, 0, 0);

        let mut states = vec![start];
        let mut new_states;

        for i in (0..32).rev() {
            new_states = vec![];

            for (ore, clay, ob, geo, r_ore, r_clay, r_ob, r_geo) in states {
                let new_ore = ore + r_ore;
                let new_clay = clay + r_clay;
                let new_ob = ob + r_ob;
                let new_geo = geo + r_geo;

                new_states.push((
                    new_ore, new_clay, new_ob, new_geo, r_ore, r_clay, r_ob, r_geo,
                ));

                if ore >= o_ore {
                    new_states.push((
                        new_ore - o_ore,
                        new_clay,
                        new_ob,
                        new_geo,
                        r_ore + 1,
                        r_clay,
                        r_ob,
                        r_geo,
                    ));
                }
                if ore >= c_ore {
                    new_states.push((
                        new_ore - c_ore,
                        new_clay,
                        new_ob,
                        new_geo,
                        r_ore,
                        r_clay + 1,
                        r_ob,
                        r_geo,
                    ));
                }
                if ore >= ob_ore && clay >= ob_clay {
                    new_states.push((
                        new_ore - ob_ore,
                        new_clay - ob_clay,
                        new_ob,
                        new_geo,
                        r_ore,
                        r_clay,
                        r_ob + 1,
                        r_geo,
                    ));
                }
                if ore >= g_ore && ob >= g_ob {
                    new_states.push((
                        new_ore - g_ore,
                        new_clay,
                        new_ob - g_ob,
                        new_geo,
                        r_ore,
                        r_clay,
                        r_ob,
                        r_geo + 1,
                    ));
                }
            }

            new_states.sort_by(|a, b| match (a.7 * i + a.3).cmp(&(b.7 * i + b.3)) {
                Ordering::Equal => match (a.6 * i + a.2).cmp(&(b.6 * i + b.2)) {
                    o => o,
                },
                o => o,
            });
            states = new_states.into_iter().rev().take(100000).collect_vec();
        }

        let best = states.into_iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap();
        let max_geodes = best.3;

        quality *= max_geodes;
    }

    Some(quality)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(62 * 56));
    }
}
