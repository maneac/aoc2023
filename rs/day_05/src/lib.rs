use std::{fs::read_to_string, ops::Range, path::Path};

pub const PART_1: usize = 389056265;
pub const PART_2: usize = 137516820;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_05.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let mut out = Self::default();
        let mut section_header = None;
        for chunk in data.trim().split("\n\n") {
            if let Some(seeds) = chunk.strip_prefix("seeds: ") {
                out.seeds = seeds
                    .trim()
                    .split_ascii_whitespace()
                    .map(|seed| seed.parse().unwrap())
                    .collect();
                continue;
            }
            let mut ranges = Vec::new();
            for line in chunk.trim().lines() {
                if section_header.is_none() {
                    section_header = Some(line.trim());
                    continue;
                }
                let nums = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .take(3)
                    .collect::<Vec<_>>();

                ranges.push((nums[0]..(nums[0] + nums[2]), nums[1]..(nums[1] + nums[2])));
            }
            match section_header {
                Some("seed-to-soil map:") => out.seed_to_soil = ranges,
                Some("soil-to-fertilizer map:") => out.soil_to_fertilizer = ranges,
                Some("fertilizer-to-water map:") => out.fertilizer_to_water = ranges,
                Some("water-to-light map:") => out.water_to_light = ranges,
                Some("light-to-temperature map:") => out.light_to_temperature = ranges,
                Some("temperature-to-humidity map:") => out.temperature_to_humidity = ranges,
                Some("humidity-to-location map:") => out.humidity_to_location = ranges,
                Some(v) => unreachable!("{v}"),
                None => unreachable!(),
            }
            section_header = None;
        }
        out
    }

    pub fn part_1(&self) -> usize {
        self.seeds
            .iter()
            .map(|&seed| {
                let soil = self
                    .seed_to_soil
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&seed) {
                            Some(dest.start + (seed - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(seed);

                let fertilizer = self
                    .soil_to_fertilizer
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&soil) {
                            Some(dest.start + (soil - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(soil);

                let water = self
                    .fertilizer_to_water
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&fertilizer) {
                            Some(dest.start + (fertilizer - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(fertilizer);

                let light = self
                    .water_to_light
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&water) {
                            Some(dest.start + (water - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(water);

                let temperature = self
                    .light_to_temperature
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&light) {
                            Some(dest.start + (light - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(light);

                let humidity = self
                    .temperature_to_humidity
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&temperature) {
                            Some(dest.start + (temperature - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(temperature);

                let location = self
                    .humidity_to_location
                    .iter()
                    .find_map(|(dest, src)| {
                        if src.contains(&humidity) {
                            Some(dest.start + (humidity - src.start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(humidity);

                location
            })
            .min()
            .unwrap_or_default()
    }

    pub fn part_2(&self) -> usize {
        self.seeds
            .iter()
            .step_by(2)
            .zip(self.seeds.iter().skip(1).step_by(2))
            .map(|(&start, &len)| {
                (start..(start + len))
                    .map(|seed| {
                        let soil = self
                            .seed_to_soil
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&seed) {
                                    Some(dest.start + (seed - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(seed);

                        let fertilizer = self
                            .soil_to_fertilizer
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&soil) {
                                    Some(dest.start + (soil - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(soil);

                        let water = self
                            .fertilizer_to_water
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&fertilizer) {
                                    Some(dest.start + (fertilizer - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(fertilizer);

                        let light = self
                            .water_to_light
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&water) {
                                    Some(dest.start + (water - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(water);

                        let temperature = self
                            .light_to_temperature
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&light) {
                                    Some(dest.start + (light - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(light);

                        let humidity = self
                            .temperature_to_humidity
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&temperature) {
                                    Some(dest.start + (temperature - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(temperature);

                        let location = self
                            .humidity_to_location
                            .iter()
                            .find_map(|(dest, src)| {
                                if src.contains(&humidity) {
                                    Some(dest.start + (humidity - src.start))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(humidity);

                        location
                    })
                    .min()
                    .unwrap_or_default()
            })
            .min()
            .unwrap_or_default()
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
                expected: 35,
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
                expected: 46,
            })
        }

        #[test]
        #[ignore = "takes too long :("]
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
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            Input {
                seeds: vec![79, 14, 55, 13],
                seed_to_soil: vec![
                    (50..(50 + 2), (98..(98 + 2))),
                    (52..(52 + 48), (50..(50 + 48))),
                ],
                soil_to_fertilizer: vec![
                    (0..37, 15..(15 + 37)),
                    (37..(37 + 2), 52..(52 + 2)),
                    (39..(39 + 15), 0..15),
                ],
                fertilizer_to_water: vec![
                    (49..(49 + 8), 53..(53 + 8)),
                    (0..42, 11..(11 + 42)),
                    (42..(42 + 7), 0..7),
                    (57..(57 + 4), 7..(7 + 4)),
                ],
                water_to_light: vec![(88..(88 + 7), 18..(18 + 7)), (18..(18 + 70), 25..(25 + 70))],
                light_to_temperature: vec![
                    (45..(45 + 23), 77..(77 + 23)),
                    (81..(81 + 19), 45..(45 + 19)),
                    (68..(68 + 13), 64..(64 + 13)),
                ],
                temperature_to_humidity: vec![(0..1, 69..(69 + 1)), (1..(1 + 69), 0..69)],
                humidity_to_location: vec![
                    (60..(60 + 37), 56..(56 + 37)),
                    (56..(56 + 4), 93..(93 + 4)),
                ],
            },
        )
    }
}
