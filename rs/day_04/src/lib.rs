use std::{collections::HashSet, fs::read_to_string, path::Path};

pub const PART_1: usize = 27454;
pub const PART_2: usize = 6857330;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_04.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    cards: Vec<(HashSet<usize>, HashSet<usize>)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            cards: data
                .trim()
                .lines()
                .map(|line| {
                    let (lhs, rhs) = line
                        .trim()
                        .split_once(':')
                        .unwrap()
                        .1
                        .split_once('|')
                        .unwrap();
                    (
                        lhs.trim()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect(),
                        rhs.trim()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.cards
            .iter()
            .map(|(winning_nums, have)| {
                let count = winning_nums.intersection(have).count();
                if count > 0 {
                    1 << (count - 1)
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        let mut counts = vec![1usize; self.cards.len()];

        self.cards
            .iter()
            .enumerate()
            .for_each(|(idx, (winning_nums, have))| {
                let count = counts[idx];

                let winnings = winning_nums.intersection(have).count();
                if winnings == 0 {
                    return;
                }

                for i in 0..winnings {
                    counts[idx + i + 1] += count;
                }
            });

        counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
                expected: 13,
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
                expected: 30,
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            Input {
                cards: vec![
                    (
                        HashSet::from([41, 48, 83, 86, 17]),
                        HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
                    ),
                    (
                        HashSet::from([13, 32, 20, 16, 61]),
                        HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]),
                    ),
                    (
                        HashSet::from([1, 21, 53, 59, 44]),
                        HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]),
                    ),
                    (
                        HashSet::from([41, 92, 73, 84, 69]),
                        HashSet::from([59, 84, 76, 51, 58, 5, 54, 83]),
                    ),
                    (
                        HashSet::from([87, 83, 26, 28, 32]),
                        HashSet::from([88, 30, 70, 12, 93, 22, 82, 36]),
                    ),
                    (
                        HashSet::from([31, 18, 13, 56, 72]),
                        HashSet::from([74, 77, 10, 23, 35, 67, 36, 11]),
                    ),
                ],
            },
        )
    }
}
