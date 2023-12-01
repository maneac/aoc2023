use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 54927;
pub const PART_2: usize = 54581;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_01.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input<'i> {
    lines: Vec<&'i str>,
}

impl<'i> Input<'i> {
    pub fn from_data(data: &'i str) -> Self {
        Self {
            lines: data.trim().split_ascii_whitespace().collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.lines
            .iter()
            .map(|line| {
                let mut iter = line.chars().filter(char::is_ascii_digit);
                let first_digit = iter.next().unwrap() as u8 - b'0';
                let last_digit = iter
                    .last()
                    .map(|c| c as u8 - b'0')
                    .unwrap_or_else(|| first_digit);
                ((first_digit * 10) + last_digit) as usize
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        self.lines
            .iter()
            .map(|line| {
                let mut first_digit = line
                    .char_indices()
                    .filter_map(|(idx, c)| {
                        if c.is_numeric() {
                            Some((idx, c as u8 - b'0'))
                        } else {
                            None
                        }
                    })
                    .next();
                let mut last_digit = line
                    .char_indices()
                    .filter_map(|(idx, c)| {
                        if c.is_numeric() {
                            Some((idx, c as u8 - b'0'))
                        } else {
                            None
                        }
                    })
                    .last();

                for (p_idx, pattern) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .enumerate()
                {
                    if let Some(idx) = line.find(pattern) {
                        if first_digit.is_none() || first_digit.as_ref().unwrap().0 > idx {
                            first_digit = Some((idx, p_idx as u8 + 1))
                        }
                    }
                    if let Some(idx) = line.rfind(pattern) {
                        if last_digit.is_none() || last_digit.as_ref().unwrap().0 < idx {
                            last_digit = Some((idx, p_idx as u8 + 1))
                        }
                    }
                }

                ((first_digit.unwrap().1 * 10) + last_digit.unwrap().1) as usize
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input<'c>,
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

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 142,
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

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example2().1,
                expected: 281,
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

    fn example() -> (&'static str, Input<'static>) {
        (
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
            Input {
                lines: vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"],
            },
        )
    }

    fn example2() -> (&'static str, Input<'static>) {
        (
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
            Input {
                lines: vec![
                    "two1nine",
                    "eightwothree",
                    "abcone2threexyz",
                    "xtwone3four",
                    "4nineeightseven2",
                    "zoneight234",
                    "7pqrstsixteen",
                ],
            },
        )
    }
}
