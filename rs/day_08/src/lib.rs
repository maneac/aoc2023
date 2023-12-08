use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 12361;
pub const PART_2: usize = 18215611419223;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_08.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    directions: Vec<bool>,
    network: HashMap<[char; 3], ([char; 3], [char; 3])>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let (dirs, remainder) = data.trim().split_once("\n\n").unwrap();

        let network = remainder
            .trim()
            .lines()
            .fold(HashMap::new(), |mut acc, line| {
                let (label, others) = line.trim().split_once(" = ").unwrap();
                let label = label
                    .char_indices()
                    .fold([char::default(); 3], |mut acc, (idx, c)| {
                        acc[idx] = c;
                        acc
                    });

                let p: &[_] = &['(', ')'];
                let (lhs, rhs) = others.trim().trim_matches(p).split_once(", ").unwrap();

                let lhs = lhs
                    .char_indices()
                    .fold([char::default(); 3], |mut acc, (idx, c)| {
                        acc[idx] = c;
                        acc
                    });
                let rhs = rhs
                    .char_indices()
                    .fold([char::default(); 3], |mut acc, (idx, c)| {
                        acc[idx] = c;
                        acc
                    });

                acc.insert(label, (lhs, rhs));
                acc
            });

        Self {
            directions: dirs.chars().map(|c| c == 'R').collect(),
            network,
        }
    }

    pub fn part_1(&self) -> usize {
        let mut node = ['A', 'A', 'A'];

        for (count, dir) in self.directions.iter().cycle().enumerate() {
            if node == ['Z', 'Z', 'Z'] {
                return count;
            }

            node = self
                .network
                .get(&node)
                .copied()
                .map(|(lhs, rhs)| if *dir { rhs } else { lhs })
                .unwrap();
        }

        unreachable!()
    }

    pub fn part_2(&self) -> usize {
        self.network
            .keys()
            .filter(|label| label[2] == 'A')
            .map(|node| {
                let mut node = *node;

                for (count, dir) in self.directions.iter().cycle().enumerate() {
                    if node[2] == 'Z' {
                        return count;
                    }

                    node = self
                        .network
                        .get(&node)
                        .copied()
                        .map(|(lhs, rhs)| if *dir { rhs } else { lhs })
                        .unwrap();
                }
                unreachable!()
            })
            .fold(None, |acc, n| {
                if let Some(acc) = acc {
                    Some(lcm(acc, n))
                } else {
                    Some(n)
                }
            })
            .unwrap_or_default()
    }
}

fn lcm(x: usize, y: usize) -> usize {
    (x / gcd(x, y)) * y
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
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
                expected: 2,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 6,
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
                data: super::example_3().1,
                expected: 6,
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
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
            Input {
                directions: vec![true, false],
                network: HashMap::from([
                    (['A', 'A', 'A'], (['B', 'B', 'B'], ['C', 'C', 'C'])),
                    (['B', 'B', 'B'], (['D', 'D', 'D'], ['E', 'E', 'E'])),
                    (['C', 'C', 'C'], (['Z', 'Z', 'Z'], ['G', 'G', 'G'])),
                    (['D', 'D', 'D'], (['D', 'D', 'D'], ['D', 'D', 'D'])),
                    (['E', 'E', 'E'], (['E', 'E', 'E'], ['E', 'E', 'E'])),
                    (['G', 'G', 'G'], (['G', 'G', 'G'], ['G', 'G', 'G'])),
                    (['Z', 'Z', 'Z'], (['Z', 'Z', 'Z'], ['Z', 'Z', 'Z'])),
                ]),
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
            Input {
                directions: vec![false, false, true],
                network: HashMap::from([
                    (['A', 'A', 'A'], (['B', 'B', 'B'], ['B', 'B', 'B'])),
                    (['B', 'B', 'B'], (['A', 'A', 'A'], ['Z', 'Z', 'Z'])),
                    (['Z', 'Z', 'Z'], (['Z', 'Z', 'Z'], ['Z', 'Z', 'Z'])),
                ]),
            },
        )
    }

    fn example_3() -> (&'static str, Input) {
        (
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
            Input {
                directions: vec![false, true],
                network: HashMap::from([
                    (['1', '1', 'A'], (['1', '1', 'B'], ['X', 'X', 'X'])),
                    (['1', '1', 'B'], (['X', 'X', 'X'], ['1', '1', 'Z'])),
                    (['1', '1', 'Z'], (['1', '1', 'B'], ['X', 'X', 'X'])),
                    (['2', '2', 'A'], (['2', '2', 'B'], ['X', 'X', 'X'])),
                    (['2', '2', 'B'], (['2', '2', 'C'], ['2', '2', 'C'])),
                    (['2', '2', 'C'], (['2', '2', 'Z'], ['2', '2', 'Z'])),
                    (['2', '2', 'Z'], (['2', '2', 'B'], ['2', '2', 'B'])),
                    (['X', 'X', 'X'], (['X', 'X', 'X'], ['X', 'X', 'X'])),
                ]),
            },
        )
    }
}
