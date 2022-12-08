fn is_visible(trees: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> bool {
    let tree = trees[x][y];
    let mut visible = true;

    for i in 0..x {
        if trees[i][y] >= tree {
            visible = false;
        }
    }

    if visible {
        return true;
    } else {
        visible = true;
    }

    for i in (x + 1)..trees.len() {
        if trees[i][y] >= tree {
            visible = false;
        }
    }

    if visible {
        return true;
    } else {
        visible = true;
    }

    for j in 0..y {
        if trees[x][j] >= tree {
            visible = false;
        }
    }

    if visible {
        return true;
    } else {
        visible = true;
    }

    for j in (y + 1)..trees[x].len() {
        if trees[x][j] >= tree {
            visible = false;
            break;
        }
    }

    visible
}

fn visibility(trees: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> u32 {
    let tree = trees[x][y];
    let mut visibility = 1;

    let mut current_vis = 0;

    for i in (0..=x.saturating_sub(1)).rev() {
        current_vis += 1;
        if trees[i][y] >= tree {
            break;
        }
    }

    visibility *= current_vis;
    current_vis = 0;

    for i in (x + 1)..trees.len() {
        current_vis += 1;
        if trees[i][y] >= tree {
            break;
        }
    }

    visibility *= current_vis;
    current_vis = 0;

    for j in (0..=y.saturating_sub(1)).rev() {
        current_vis += 1;
        if trees[x][j] >= tree {
            break;
        }
    }

    visibility *= current_vis;
    current_vis = 0;

    for j in (y + 1)..trees[x].len() {
        current_vis += 1;
        if trees[x][j] >= tree {
            break;
        }
    }

    visibility *= current_vis;

    visibility
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            if is_visible(&trees, (x, y)) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut best = 0;

    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            best = best.max(visibility(&trees, (x, y)));
        }
    }

    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
