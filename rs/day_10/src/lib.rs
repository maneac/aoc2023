use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 7063;
pub const PART_2: usize = 589;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_10.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Input {
    pipes: HashMap<(usize, usize), Pipe>,
    max_x: usize,
    max_y: usize,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let pipes = data
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                max_y = max_y.max(y);
                max_x = max_x.max(line.trim().len() - 1);
                line.trim()
                    .char_indices()
                    .filter_map(move |(x, c)| match c {
                        'S' => Some(((x, y), Pipe::Start)),
                        '|' => Some(((x, y), Pipe::Vertical)),
                        '-' => Some(((x, y), Pipe::Horizontal)),
                        'L' => Some(((x, y), Pipe::NorthEast)),
                        'J' => Some(((x, y), Pipe::NorthWest)),
                        '7' => Some(((x, y), Pipe::SouthWest)),
                        'F' => Some(((x, y), Pipe::SouthEast)),
                        '.' => None,
                        _ => unreachable!(),
                    })
            })
            .collect();

        Self {
            pipes,
            max_x,
            max_y,
        }
    }

    pub fn part_1(&self) -> usize {
        self.visit().len() / 2
    }

    pub fn part_2(&self) -> usize {
        let perimeter = self.visit();

        let mut count = 0;
        for y in 0..self.max_y {
            let mut inside = false;
            for x in 0..self.max_x {
                if perimeter.contains(&(x, y)) {
                    let pipe = self.pipes.get(&(x, y)).unwrap();

                    if [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest].contains(pipe) {
                        inside = !inside;
                    }
                    continue;
                }

                if inside {
                    count += 1
                }
            }
        }
        count
    }

    fn visit(&self) -> HashSet<(usize, usize)> {
        let (&start, &node) = self.pipes.iter().find(|(_, &v)| v == Pipe::Start).unwrap();

        let mut visited = HashSet::new();
        let mut considerations = vec![(0, start, node)];
        while let Some((count, (x, y), node)) = considerations.pop() {
            visited.insert((x, y));

            let possibilities = match node {
                Pipe::Start => {
                    vec![
                        (
                            (x, y.saturating_sub(1)),
                            vec![Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest],
                        ),
                        (
                            (x, (y + 1).clamp(0, self.max_y)),
                            vec![Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest],
                        ),
                        (
                            (x.saturating_sub(1), y),
                            vec![Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
                        ),
                        (
                            ((x + 1).clamp(0, self.max_x), y),
                            vec![Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
                        ),
                    ]
                }
                Pipe::Vertical => {
                    vec![
                        (
                            (x, y.saturating_sub(1)),
                            vec![Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest],
                        ),
                        (
                            (x, (y + 1).clamp(0, self.max_y)),
                            vec![Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest],
                        ),
                    ]
                }
                Pipe::Horizontal => {
                    vec![
                        (
                            (x.saturating_sub(1), y),
                            vec![Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
                        ),
                        (
                            ((x + 1).clamp(0, self.max_x), y),
                            vec![Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
                        ),
                    ]
                }
                Pipe::NorthEast => {
                    vec![
                        (
                            (x, y.saturating_sub(1)),
                            vec![Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest],
                        ),
                        (
                            ((x + 1).clamp(0, self.max_x), y),
                            vec![Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
                        ),
                    ]
                }
                Pipe::NorthWest => {
                    vec![
                        (
                            (x, y.saturating_sub(1)),
                            vec![Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest],
                        ),
                        (
                            (x.saturating_sub(1), y),
                            vec![Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
                        ),
                    ]
                }
                Pipe::SouthWest => {
                    vec![
                        (
                            (x, (y + 1).clamp(0, self.max_y)),
                            vec![Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest],
                        ),
                        (
                            (x.saturating_sub(1), y),
                            vec![Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast],
                        ),
                    ]
                }
                Pipe::SouthEast => {
                    vec![
                        (
                            (x, (y + 1).clamp(0, self.max_y)),
                            vec![Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest],
                        ),
                        (
                            ((x + 1).clamp(0, self.max_x), y),
                            vec![Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest],
                        ),
                    ]
                }
            };

            for ((new_x, new_y), pipes) in possibilities {
                if (new_x, new_y) == (x, y) {
                    continue;
                }
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }
                if let Some(&pipe) = self.pipes.get(&(new_x, new_y)) {
                    if !pipes.contains(&pipe) {
                        continue;
                    }
                    considerations.push((count + 1, (new_x, new_y), pipe));
                }
            }
        }

        visited
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example_1() {
            run(&Case {
                input: super::example_1().0,
                expected: super::example_1().1,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: super::example_2().0,
                expected: super::example_2().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: 4,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 8,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1())
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: Input::from_data(
                    "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
                ),
                expected: 4,
            })
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: Input::from_data(
                    ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
                ),
                expected: 8,
            })
        }

        #[test]
        fn example_5() {
            run(&Case {
                data: Input::from_data(
                    "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
                ),
                expected: 10,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2())
        }
    }

    fn example_1() -> (&'static str, Input) {
        (
            ".....
.S-7.
.|.|.
.L-J.
.....",
            Input {
                pipes: HashMap::from([
                    ((1, 1), Pipe::Start),
                    ((2, 1), Pipe::Horizontal),
                    ((3, 1), Pipe::SouthWest),
                    ((1, 2), Pipe::Vertical),
                    ((3, 2), Pipe::Vertical),
                    ((1, 3), Pipe::NorthEast),
                    ((2, 3), Pipe::Horizontal),
                    ((3, 3), Pipe::NorthWest),
                ]),
                max_x: 4,
                max_y: 4,
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
            Input {
                pipes: HashMap::from([
                    ((2, 0), Pipe::SouthEast),
                    ((3, 0), Pipe::SouthWest),
                    ((1, 1), Pipe::SouthEast),
                    ((2, 1), Pipe::NorthWest),
                    ((3, 1), Pipe::Vertical),
                    ((0, 2), Pipe::Start),
                    ((1, 2), Pipe::NorthWest),
                    ((3, 2), Pipe::NorthEast),
                    ((4, 2), Pipe::SouthWest),
                    ((0, 3), Pipe::Vertical),
                    ((1, 3), Pipe::SouthEast),
                    ((2, 3), Pipe::Horizontal),
                    ((3, 3), Pipe::Horizontal),
                    ((4, 3), Pipe::NorthWest),
                    ((0, 4), Pipe::NorthEast),
                    ((1, 4), Pipe::NorthWest),
                ]),
                max_x: 4,
                max_y: 4,
            },
        )
    }
}
