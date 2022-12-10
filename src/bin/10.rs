pub fn part_one(input: &str) -> Option<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let mut is_adding = false;

    for line in input.lines() {
        match &line[0..4] {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                is_adding = true;
            }
            _ => unreachable!(),
        }

        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                sum += cycle * x;
            }
            _ => {}
        }

        if is_adding {
            is_adding = false;
            cycle += 1;

            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    sum += cycle * x;
                }
                _ => {}
            }

            x += line[5..].parse::<i32>().unwrap();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let mut is_adding = false;

    let mut crt = 0;
    let mut screen = vec![];

    for line in input.lines() {
        match &line[0..4] {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                is_adding = true;
            }
            _ => unreachable!(),
        }

        screen.push(crt == x + 1 || crt == x || crt == x - 1);
        if crt == 39 {
            crt = 0;
        } else {
            crt += 1;
        }

        if is_adding {
            is_adding = false;
            cycle += 1;

            screen.push(crt == x + 1 || crt == x || crt == x - 1);
            if crt == 39 {
                crt = 0;
            } else {
                crt += 1;
            }

            x += line[5..].parse::<i32>().unwrap();
        }
    }

    let mut output = String::new();
    for y in 0..6 {
        for x in 0..40 {
            output += if screen[x + y * 40] { "#" } else { "." }
        }
        output += "\n";
    }

    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n".to_string()));
    }
}
