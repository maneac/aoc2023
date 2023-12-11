use std::{fs::read_to_string, path::Path};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub const PART_1: usize = 10173804;
pub const PART_2: usize = 634324905172;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_11.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    galaxies: Vec<(usize, usize)>,
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let data = data.trim();
        let col_count = data.lines().next().unwrap().trim().len();
        let row_count = data.lines().count();

        let (empty_cols, empty_rows, galaxies) = data.trim().lines().enumerate().fold(
            (vec![true; col_count], vec![true; row_count], Vec::new()),
            |(mut empty_cols, mut empty_rows, mut acc), (y, line)| {
                for (x, _) in line.trim().char_indices().filter(|&(_, c)| c == '#') {
                    empty_cols[x] = false;
                    empty_rows[y] = false;
                    acc.push((x, y));
                }
                (empty_cols, empty_rows, acc)
            },
        );

        Self {
            galaxies,
            empty_cols: empty_cols
                .iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(|(idx, _)| idx)
                .collect(),
            empty_rows: empty_rows
                .iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(|(idx, _)| idx)
                .collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        self.distances::<1>()
    }

    pub fn part_2(&self) -> usize {
        self.distances::<1_000_000>()
    }

    fn distances<const EXPANSION: usize>(&self) -> usize {
        let expansion = (EXPANSION - 1).clamp(1, usize::MAX);

        self.galaxies
            .par_iter()
            .enumerate()
            .flat_map(|(idx, &(lhs_x, lhs_y))| {
                self.galaxies
                    .par_iter()
                    .skip(idx + 1)
                    .map(move |&(rhs_x, rhs_y)| {
                        let min_x = lhs_x.min(rhs_x);
                        let min_y = lhs_y.min(rhs_y);
                        let max_x = lhs_x.max(rhs_x);
                        let max_y = lhs_y.max(rhs_y);

                        (max_x - min_x)
                            + (min_x..max_x)
                                .filter_map(|x| {
                                    self.empty_cols.binary_search(&x).map(|_| expansion).ok()
                                })
                                .sum::<usize>()
                            + (max_y - min_y)
                            + (min_y..max_y)
                                .filter_map(|y| {
                                    self.empty_rows.binary_search(&y).map(|_| expansion).ok()
                                })
                                .sum::<usize>()
                    })
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
                expected: 374,
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
            assert_eq!(1030, super::example().1.distances::<10>());
            assert_eq!(8410, super::example().1.distances::<100>());
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
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            Input {
                galaxies: vec![
                    (3, 0),
                    (7, 1),
                    (0, 2),
                    (6, 4),
                    (1, 5),
                    (9, 6),
                    (7, 8),
                    (0, 9),
                    (4, 9),
                ],
                empty_cols: vec![2, 5, 8],
                empty_rows: vec![3, 7],
            },
        )
    }
}
