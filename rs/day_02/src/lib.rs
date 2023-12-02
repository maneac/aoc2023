use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1867;
pub const PART_2: usize = 84538;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_02.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    games: Vec<Vec<Cubes>>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cubes {
    red: u8,
    green: u8,
    blue: u8,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let data = data.trim();

        Self {
            games: data
                .lines()
                .map(|line| {
                    let rhs = line.split_once(": ").unwrap().1;
                    rhs.split("; ")
                        .map(|chunk| {
                            chunk.split(',').fold(
                                Cubes {
                                    red: 0,
                                    green: 0,
                                    blue: 0,
                                },
                                |mut acc, part| {
                                    let (num, colour) = part.trim().split_once(' ').unwrap();
                                    let num = num.parse::<u8>().unwrap();
                                    match colour {
                                        "blue" => acc.blue = num,
                                        "green" => acc.green = num,
                                        "red" => acc.red = num,
                                        _ => panic!(),
                                    }
                                    acc
                                },
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.games
            .iter()
            .enumerate()
            .filter_map(|(idx, games)| {
                if games
                    .iter()
                    .all(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
                {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        self.games
            .iter()
            .map(|games| {
                let maxs = games.iter().fold(
                    Cubes {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |mut acc, cube| {
                        acc.blue = acc.blue.max(cube.blue);
                        acc.red = acc.red.max(cube.red);
                        acc.green = acc.green.max(cube.green);
                        acc
                    },
                );

                maxs.blue as usize * maxs.red as usize * maxs.green as usize
            })
            .sum()
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
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 2286,
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
            "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            Input {
                games: vec![
                    vec![
                        Cubes {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Cubes {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Cubes {
                            green: 2,
                            red: 0,
                            blue: 0,
                        },
                    ],
                    vec![
                        Cubes {
                            red: 0,
                            green: 2,
                            blue: 1,
                        },
                        Cubes {
                            red: 1,
                            green: 3,
                            blue: 4,
                        },
                        Cubes {
                            green: 1,
                            red: 0,
                            blue: 1,
                        },
                    ],
                    vec![
                        Cubes {
                            green: 8,
                            blue: 6,
                            red: 20,
                        },
                        Cubes {
                            blue: 5,
                            red: 4,
                            green: 13,
                        },
                        Cubes {
                            green: 5,
                            red: 1,
                            blue: 0,
                        },
                    ],
                    vec![
                        Cubes {
                            green: 1,
                            red: 3,
                            blue: 6,
                        },
                        Cubes {
                            green: 3,
                            red: 6,
                            blue: 0,
                        },
                        Cubes {
                            green: 3,
                            blue: 15,
                            red: 14,
                        },
                    ],
                    vec![
                        Cubes {
                            red: 6,
                            blue: 1,
                            green: 3,
                        },
                        Cubes {
                            blue: 2,
                            red: 1,
                            green: 2,
                        },
                    ],
                ],
            },
        )
    }
}
