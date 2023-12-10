use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1789635132;
pub const PART_2: usize = 913;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_09.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    sequences: Vec<Vec<isize>>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            sequences: data
                .trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .split_ascii_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.sequences
            .iter()
            .map(|sequence| next(sequence))
            .sum::<isize>() as usize
    }

    pub fn part_2(&self) -> usize {
        self.sequences
            .iter()
            .map(|sequence| prev(sequence))
            .sum::<isize>() as usize
    }
}

fn next(sequence: &[isize]) -> isize {
    let diffs = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    if diffs.iter().all(|&diff| diff == 0) {
        return sequence.last().copied().unwrap();
    }

    sequence.last().copied().unwrap() + next(&diffs)
}

fn prev(sequence: &[isize]) -> isize {
    let diffs = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    if diffs.iter().all(|&diff| diff == 0) {
        return sequence[0];
    }

    sequence[0] - prev(&diffs)
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
        fn example() {
            run(&Case {
                input: super::example().0,
                expected: super::example().1,
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
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 114,
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
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 2,
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

    fn example() -> (&'static str, Input) {
        (
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
            Input {
                sequences: vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45],
                ],
            },
        )
    }
}
