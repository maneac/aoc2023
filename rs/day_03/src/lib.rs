use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 527369;
pub const PART_2: usize = 73074886;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_03.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    entries: HashMap<Point, Entry>,
    max_x: usize,
    max_y: usize,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        data.trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |mut acc, (y, line)| {
                let mut val = None;
                for (idx, c) in line.trim().char_indices() {
                    match c {
                        '.' => {
                            if let Some((x, entry)) = val.take() {
                                acc.entries.insert(Point { x, y }, entry);
                            }
                        }
                        '0'..='9' => {
                            if let Some((x, entry)) = val {
                                match entry {
                                    Entry::Symbol { .. } => unreachable!(),
                                    Entry::Number { value, length } => {
                                        val = Some((
                                            x,
                                            Entry::Number {
                                                value: (10 * value) + ((c as u8 - b'0') as usize),
                                                length: length + 1,
                                            },
                                        ))
                                    }
                                }
                            } else {
                                val = Some((
                                    idx,
                                    Entry::Number {
                                        value: (c as u8 - b'0') as usize,
                                        length: 1,
                                    },
                                ))
                            }
                        }
                        _ => {
                            if let Some((x, entry)) = val.take() {
                                acc.entries.insert(Point { x, y }, entry);
                            }
                            acc.entries
                                .insert(Point { x: idx, y }, Entry::Symbol { symbol: c });
                        }
                    }
                    acc.max_x = idx;
                }
                if let Some((x, entry)) = val.take() {
                    acc.entries.insert(Point { x, y }, entry);
                }
                acc.max_y = y;
                acc
            })
    }

    pub fn part_1(&self) -> usize {
        self.entries
            .iter()
            .filter_map(|(k, v)| match v {
                Entry::Symbol { .. } => None,
                Entry::Number { value, length } => Some((*k, *value, *length)),
            })
            .filter(|(k, _value, length)| {
                ((k.x.saturating_sub(1))..=(k.x + length).clamp(0, self.max_x)).any(|x| {
                    ((k.y.saturating_sub(1))..=(k.y + 1).clamp(0, self.max_y)).any(|y| {
                        matches!(
                            self.entries.get(&Point { x, y }),
                            Some(Entry::Symbol { .. })
                        )
                    })
                })
            })
            .map(|(_, value, _)| value)
            .sum()
    }

    pub fn part_2(&self) -> usize {
        let mut hashes = HashMap::new();

        self.entries
            .iter()
            .filter_map(|(k, v)| match v {
                Entry::Symbol { .. } => None,
                Entry::Number { value, length } => Some((*k, *value, *length)),
            })
            .for_each(|(k, value, length)| {
                for x in (k.x.saturating_sub(1))..=(k.x + length).clamp(0, self.max_x) {
                    for y in (k.y.saturating_sub(1))..=(k.y + 1).clamp(0, self.max_y) {
                        if matches!(
                            self.entries.get(&Point { x, y }),
                            Some(Entry::Symbol { symbol: '*' })
                        ) {
                            hashes
                                .entry(Point { x, y })
                                .and_modify(|entries: &mut Vec<usize>| {
                                    entries.push(value);
                                })
                                .or_insert(vec![value]);
                        }
                    }
                }
            });

        hashes
            .iter()
            .filter(|(_, entries)| entries.len() == 2)
            .map(|(_, entries)| entries.iter().product::<usize>())
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Entry {
    Symbol { symbol: char },
    Number { value: usize, length: usize },
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
                expected: 4361,
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
                expected: 467835,
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
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            Input {
                max_x: 9,
                max_y: 9,
                entries: HashMap::from([
                    (
                        Point { x: 0, y: 0 },
                        Entry::Number {
                            value: 467,
                            length: 3,
                        },
                    ),
                    (
                        Point { x: 5, y: 0 },
                        Entry::Number {
                            value: 114,
                            length: 3,
                        },
                    ),
                    (Point { x: 3, y: 1 }, Entry::Symbol { symbol: '*' }),
                    (
                        Point { x: 2, y: 2 },
                        Entry::Number {
                            value: 35,
                            length: 2,
                        },
                    ),
                    (
                        Point { x: 6, y: 2 },
                        Entry::Number {
                            value: 633,
                            length: 3,
                        },
                    ),
                    (Point { x: 6, y: 3 }, Entry::Symbol { symbol: '#' }),
                    (
                        Point { x: 0, y: 4 },
                        Entry::Number {
                            value: 617,
                            length: 3,
                        },
                    ),
                    (Point { x: 3, y: 4 }, Entry::Symbol { symbol: '*' }),
                    (Point { x: 5, y: 5 }, Entry::Symbol { symbol: '+' }),
                    (
                        Point { x: 7, y: 5 },
                        Entry::Number {
                            value: 58,
                            length: 2,
                        },
                    ),
                    (
                        Point { x: 2, y: 6 },
                        Entry::Number {
                            value: 592,
                            length: 3,
                        },
                    ),
                    (
                        Point { x: 6, y: 7 },
                        Entry::Number {
                            value: 755,
                            length: 3,
                        },
                    ),
                    (Point { x: 3, y: 8 }, Entry::Symbol { symbol: '$' }),
                    (Point { x: 5, y: 8 }, Entry::Symbol { symbol: '*' }),
                    (
                        Point { x: 1, y: 9 },
                        Entry::Number {
                            value: 664,
                            length: 3,
                        },
                    ),
                    (
                        Point { x: 5, y: 9 },
                        Entry::Number {
                            value: 598,
                            length: 3,
                        },
                    ),
                ]),
            },
        )
    }
}
