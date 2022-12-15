use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sensors = vec![];

    for line in input.lines() {
        let (sensor, beacon) = line.split_once(':').unwrap();
        let sensor = &sensor[12..];
        let (x, y) = sensor.split_once(',').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y[3..].parse().unwrap();

        let (bx, by) = beacon.split_once(',').unwrap();
        let bx: i32 = bx[24..].parse().unwrap();
        let by: i32 = by[3..].parse().unwrap();

        sensors.push(((x, y), (bx, by)));
    }

    let distances = sensors
        .iter()
        .map(|((x, y), (bx, by))| (bx - x).abs() + (by - y).abs())
        .collect_vec();

    let target_y = if sensors.len() < 15 { 10 } else { 2000000 };

    let sensors_a = sensors
        .iter()
        .zip(distances.iter())
        .filter(|(((_, y), _), dist)| (target_y - y).abs() < **dist)
        .collect_vec();

    let ranges = sensors_a
        .iter()
        .map(|(((x, y), _), dist)| {
            let range = **dist - (target_y - y).abs();
            (x - range, x + range)
        })
        .collect_vec();

    let mut new_ranges = vec![];

    for (mut a, mut b) in ranges {
        loop {
            let mut overlap = false;
            let mut index = 0;

            for (i, (c, d)) in new_ranges.iter().enumerate() {
                if (a <= *c && b >= *c) || (*c <= a && *d >= a) {
                    overlap = true;
                    index = i;
                    break;
                }
            }

            if !overlap {
                new_ranges.push((a, b));
                break;
            } else {
                let (c, d) = new_ranges.remove(index);

                if a <= c && b >= c {
                    (a, b) = (a, b.max(d));
                } else {
                    (a, b) = (c, b.max(d));
                }
            }
        }
    }

    let mut beacons = sensors
        .iter()
        .map(|(_, b)| b)
        .filter(|(_, y)| *y == target_y)
        .collect_vec();

    beacons.sort();
    beacons.dedup();

    let beacons = beacons.len();

    Some(
        new_ranges
            .into_iter()
            .map(|(a, b)| (b - a + 1) as u32)
            .sum::<u32>()
            - beacons as u32,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sensors = vec![];

    for line in input.lines() {
        let (sensor, beacon) = line.split_once(':').unwrap();
        let sensor = &sensor[12..];
        let (x, y) = sensor.split_once(',').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y[3..].parse().unwrap();

        let (bx, by) = beacon.split_once(',').unwrap();
        let bx: i32 = bx[24..].parse().unwrap();
        let by: i32 = by[3..].parse().unwrap();

        sensors.push(((x, y), (bx, by)));
    }

    let distances = sensors
        .iter()
        .map(|((x, y), (bx, by))| (bx - x).abs() + (by - y).abs())
        .collect_vec();

    let max_xy = if sensors.len() < 15 { 20 } else { 4000000 };
    let min_xy = 0;

    for target_y in min_xy..=max_xy {
        let ranges = sensors
            .iter()
            .zip(distances.iter())
            .filter(|(((_, y), _), dist)| (target_y - y).abs() < **dist)
            .map(|(((x, y), _), dist)| {
                let range = *dist - (target_y - y).abs();
                (x - range, x + range)
            })
            .collect_vec();

        let mut new_ranges = vec![];

        for (mut a, mut b) in ranges {
            while let Some(index) = new_ranges
                .iter()
                .position(|(c, d)| (a <= *c && b >= *c - 1) || (*c <= a && *d >= a - 1))
            {
                let (c, d) = new_ranges.remove(index);
                (a, b) = (a.min(c), b.max(d));
            }
            new_ranges.push((a, b));
        }

        if new_ranges.len() > 1 {
            assert!(new_ranges.len() == 2);

            let (a, b) = new_ranges[0];
            let (c, d) = new_ranges[1];

            let x = if a + 2 == d {
                a + 1
            } else if a == d + 2 {
                d + 1
            } else if b + 2 == c {
                b + 1
            } else {
                c + 1
            };
            let y = target_y;

            return Some(x as u64 * 4000000 + y as u64);
        }
    }

    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
