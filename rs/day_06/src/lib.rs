use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 219849;
pub const PART_2: usize = 29432455;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_06.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    races: Vec<Race>,
    combined_race: Race,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let (time, distance) = data.trim().split_once('\n').unwrap();

        let (races, time, dist) = time
            .trim()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .zip(
                distance
                    .trim()
                    .strip_prefix("Distance:")
                    .unwrap()
                    .trim()
                    .split_ascii_whitespace(),
            )
            .fold(
                (Vec::default(), String::new(), String::new()),
                |(mut races, mut time, mut dist), (t, d)| {
                    races.push(Race {
                        time: t.parse().unwrap(),
                        distance: d.parse().unwrap(),
                    });
                    time.push_str(t);
                    dist.push_str(d);
                    (races, time, dist)
                },
            );

        let time = time.parse().unwrap();
        let dist = dist.parse().unwrap();

        Self {
            races,
            combined_race: Race {
                time,
                distance: dist,
            },
        }
    }

    pub fn part_1(&self) -> usize {
        self.races
            .iter()
            .map(|race| {
                let min = ((race.time as f64
                    - ((race.time.pow(2) - (4 * race.distance)) as f64).sqrt())
                    / 2.0)
                    .floor() as usize;

                let max = ((race.time as f64
                    + ((race.time.pow(2) - (4 * race.distance)) as f64).sqrt())
                    / 2.0)
                    .ceil() as usize;

                max - (min + 1)
            })
            .product()
    }

    pub fn part_2(&self) -> usize {
        let min = ((self.combined_race.time as f64
            - ((self.combined_race.time.pow(2) - (4 * self.combined_race.distance)) as f64).sqrt())
            / 2.0)
            .floor() as usize;

        let max = ((self.combined_race.time as f64
            + ((self.combined_race.time.pow(2) - (4 * self.combined_race.distance)) as f64).sqrt())
            / 2.0)
            .ceil() as usize;

        max - (min + 1)
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Race {
    time: usize,
    distance: usize,
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
                expected: 288,
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
                expected: 71503,
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
        Time:      7  15   30
        Distance:  9  40  200",
            Input {
                races: vec![
                    Race {
                        time: 7,
                        distance: 9,
                    },
                    Race {
                        time: 15,
                        distance: 40,
                    },
                    Race {
                        time: 30,
                        distance: 200,
                    },
                ],
                combined_race: Race {
                    time: 71530,
                    distance: 940200,
                },
            },
        )
    }
}
