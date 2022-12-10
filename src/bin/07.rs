#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    files: Vec<(usize, String)>,
}

fn dir_size(dir: &Dir) -> usize {
    let mut size = 0;

    size += dir.subdirs.iter().map(|d| dir_size(d)).sum::<usize>();
    size += dir.files.iter().map(|f| f.0).sum::<usize>();

    size
}

fn part_one_(dir: &Dir) -> usize {
    let mut total = dir.subdirs.iter().map(|d| part_one_(d)).sum::<usize>();

    let size = dir_size(dir);

    if size <= 100000 {
        total += size;
    }

    total
}

fn part_two_(dir: &Dir, min_size: usize) -> usize {
    let mut best = dir
        .subdirs
        .iter()
        .map(|d| part_two_(d, min_size))
        .min()
        .unwrap_or(usize::MAX);

    let size = dir_size(dir);

    if size < best && size >= min_size {
        best = size;
    }

    best
}

fn parse_dir(input: &str) -> Dir {
    let mut stack = vec!["/".to_string()];
    let mut files = Dir {
        name: "/".to_string(),
        subdirs: vec![],
        files: vec![],
    };
    let mut lines = input.lines().peekable();

    loop {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };

        if line.starts_with('$') {
            let (command, arg) = line[2..].split_at(2);
            let arg = arg.trim();

            match command {
                "cd" => match arg {
                    "/" => {
                        stack = vec!["/".to_string()];
                    }
                    ".." => {
                        stack.pop();
                    }
                    _ => {
                        stack.push(arg.to_string());
                    }
                },
                "ls" => loop {
                    let line = match lines.peek() {
                        Some(l) => {
                            if l.starts_with('$') {
                                break;
                            }
                            lines.next().unwrap()
                        }
                        None => break,
                    };

                    let mut dir = &mut files;

                    for d in stack.iter().skip(1) {
                        match dir.subdirs.iter().find(|a| a.name == *d) {
                            Some(_) => dir = dir.subdirs.iter_mut().find(|a| a.name == *d).unwrap(),
                            None => {
                                dir.subdirs.push(Dir {
                                    name: d.to_string(),
                                    subdirs: vec![],
                                    files: vec![],
                                });
                                dir = dir.subdirs.last_mut().unwrap()
                            }
                        }
                    }

                    let parts = line.split_once(' ').unwrap();

                    match parts.0 {
                        "dir" => {
                            dir.subdirs.push(Dir {
                                name: parts.1.to_string(),
                                subdirs: vec![],
                                files: vec![],
                            });
                        }
                        _ => dir
                            .files
                            .push((parts.0.parse().unwrap(), parts.1.to_string())),
                    }
                },
                _ => unreachable!(),
            };
        }
    }

    files
}

pub fn part_one(input: &str) -> Option<usize> {
    let files = parse_dir(input);
    Some(part_one_(&files))
}

pub fn part_two(input: &str) -> Option<usize> {
    let files = parse_dir(input);

    let min_size = dir_size(&files) - 40000000;
    Some(part_two_(&files, min_size))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
