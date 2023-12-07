use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 248559379;
pub const PART_2: usize = 249631254;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_07.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    hand_bids: Vec<(Hand, usize)>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        Self {
            hand_bids: data
                .trim()
                .lines()
                .map(|line| {
                    let (letters, bid) = line.trim().split_once(' ').unwrap();
                    let mut cards = [Card::Joker; 5];
                    letters.chars().zip(&mut cards).for_each(|(card, e)| {
                        *e = Card::try_from(card).unwrap();
                    });
                    (Hand::from_cards(cards), bid.parse().unwrap())
                })
                .collect(),
        }
    }

    pub fn part_1(&self) -> usize {
        let mut hands = self.hand_bids.clone();
        hands.sort();
        hands
            .into_iter()
            .enumerate()
            .fold(0usize, |acc, (rank, (_, bid))| acc + (bid * (rank + 1)))
    }

    pub fn part_2(&self) -> usize {
        let mut hands = self.hand_bids.clone();
        hands.iter_mut().for_each(|(hand, _)| {
            hand.with_jokers();
        });
        hands.sort();
        hands
            .into_iter()
            .enumerate()
            .fold(0usize, |acc, (rank, (_, bid))| acc + (bid * (rank + 1)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp != std::cmp::Ordering::Equal {
            type_cmp
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl Hand {
    fn from_cards(cards: [Card; 5]) -> Self {
        let card_counts = cards.iter().fold(HashMap::new(), |mut acc, &card| {
            acc.entry(card).and_modify(|e| *e += 1).or_insert(1u8);
            acc
        });

        let hand_type = counts_to_type(card_counts);

        Self { cards, hand_type }
    }

    fn with_jokers(&mut self) {
        for card in self.cards.iter_mut() {
            if card != &Card::Jack {
                continue;
            }
            *card = Card::Joker;
        }

        let (joker_count, card_counts) =
            self.cards
                .iter()
                .fold((0, HashMap::new()), |(count, mut acc), &card| {
                    if card == Card::Joker {
                        (count + 1, acc)
                    } else {
                        acc.entry(card).and_modify(|e| *e += 1).or_insert(1u8);
                        (count, acc)
                    }
                });

        self.hand_type = best(joker_count, card_counts);
    }
}

fn best(joker_count: u8, mut card_counts: HashMap<Card, u8>) -> HandType {
    match joker_count {
        0 => counts_to_type(card_counts),
        1 | 3 => {
            card_counts
                .values_mut()
                .max()
                .map(|v| *v += joker_count)
                .unwrap_or_default();
            counts_to_type(card_counts)
        }
        2 => [
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King,
            Card::Ace,
        ]
        .into_iter()
        .map(|card| {
            let mut card_counts = card_counts.clone();
            card_counts.entry(card).and_modify(|v| *v += 1).or_insert(1);
            best(joker_count - 1, card_counts)
        })
        .max()
        .unwrap_or(HandType::HighCard),
        4 | 5 => HandType::FiveOfAKind,
        _ => unreachable!(),
    }
}

fn counts_to_type(card_counts: HashMap<Card, u8>) -> HandType {
    match card_counts.len() {
        1 => HandType::FiveOfAKind,
        2 => match *card_counts.values().next().unwrap() {
            1 | 4 => HandType::FourOfAKind,
            2 | 3 => HandType::FullHouse,
            _ => unreachable!(),
        },
        3 => {
            if card_counts.values().max().copied().unwrap_or_default() == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    #[default]
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(format!("invalid character: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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
                expected: 6440,
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
                expected: 5905,
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
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            Input {
                hand_bids: vec![
                    (
                        Hand {
                            cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                            hand_type: HandType::OnePair,
                        },
                        765,
                    ),
                    (
                        Hand {
                            cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                            hand_type: HandType::ThreeOfAKind,
                        },
                        684,
                    ),
                    (
                        Hand {
                            cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                            hand_type: HandType::TwoPair,
                        },
                        28,
                    ),
                    (
                        Hand {
                            cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                            hand_type: HandType::TwoPair,
                        },
                        220,
                    ),
                    (
                        Hand {
                            cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                            hand_type: HandType::ThreeOfAKind,
                        },
                        483,
                    ),
                ],
            },
        )
    }
}
