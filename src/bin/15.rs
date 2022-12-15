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

    let relevant_sensors = sensors
        .iter()
        .zip(distances.iter())
        .cartesian_product(sensors.iter().zip(distances.iter()))
        .filter(|((((x, y), _), dist1), (((a, b), _), dist2))| {
            (x - a).abs() + (y - b).abs() == *dist1 + *dist2 + 2
        })
        .map(|(a, _)| a)
        .collect_vec();

    let points = relevant_sensors
        .into_iter()
        .flat_map(|(((x, y), _), dist)| {
            vec![(x - y + dist, x + y + dist), (x - y - dist, x + y - dist)]
        })
        .collect_vec();

    let mut left = points
        .iter()
        .cartesian_product(points.iter())
        .filter(|((x, _), (a, _))| (x - a).abs() == 2)
        .map(|((x, _), (a, _))| (x + a) / 2)
        .collect_vec();

    let mut right = points
        .iter()
        .cartesian_product(points.iter())
        .filter(|((_, y), (_, b))| (y - b).abs() == 2)
        .map(|((_, y), (_, b))| (y + b) / 2)
        .collect_vec();

    left.sort();
    left.dedup();
    right.sort();
    right.dedup();

    let mut candidates = left
        .into_iter()
        .cartesian_product(right.into_iter())
        .map(|(left, right)| ((left + right) / 2, (right - left) / 2))
        .collect_vec();

    if candidates.len() == 1 {
        let (x, y) = candidates[0];
        Some(x as u64 * 4000000 + y as u64)
    } else {
        let max_xy = if sensors.len() < 15 { 20 } else { 4000000 };

        candidates.retain(|(x, y)| *x >= 0 && *y >= 0 && *x <= max_xy && *y <= max_xy);
        candidates.retain(|(x, y)| {
            sensors
                .iter()
                .map(|(a, _)| a)
                .zip(distances.iter())
                .all(|((a, b), dist)| (x - a).abs() + (y - b).abs() > *dist)
        });

        let (x, y) = candidates[0];
        Some(x as u64 * 4000000 + y as u64)
    }
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
