use std::{cmp::Ordering, str::FromStr, thread::current};

use crate::filereading;

pub fn run() {
    a();
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Eq, Clone)]
struct Card {
    label: usize,
}

const CARD_OPTIONS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Card {
    fn from_char(c: char) -> Card {
        for (i, option) in CARD_OPTIONS.iter().enumerate() {
            if *option == c {
                return Card { label: i };
            }
        }

        panic!();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Hand {
    FiveOfAKind {
        quint: Card,
    },
    FourOfAKind {
        quad: Card,
        single: Card,
    },
    FullHouse {
        trip: Card,
        doub: Card,
    },
    ThreeOfAKind {
        trip: Card,
        rest: Vec<Card>,
    },
    TwoPair {
        double_1: Card,
        double_2: Card,
        last: Card,
    },
    OnePair {
        double: Card,
        rest: Vec<Card>,
    },
    HighCard {
        cards: Vec<Card>,
    },
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::FiveOfAKind { quint: quint1 }, Hand::FiveOfAKind { quint: quint2 }) => {
                quint2.cmp(quint1)
            }
            (Hand::FiveOfAKind { .. }, _) => Ordering::Greater,
            (_, Hand::FiveOfAKind { .. }) => Ordering::Less,
            (
                Hand::FourOfAKind {
                    quad: quad1,
                    single: single1,
                },
                Hand::FourOfAKind {
                    quad: quad2,
                    single: single2,
                },
            ) => {
                if quad1.label == quad2.label {
                    single2.cmp(single1)
                } else {
                    quad2.cmp(quad1)
                }
            }
            (Hand::FourOfAKind { .. }, _) => Ordering::Greater,
            (_, Hand::FourOfAKind { .. }) => Ordering::Less,
            (
                Hand::FullHouse {
                    trip: trip1,
                    doub: doub1,
                },
                Hand::FullHouse {
                    trip: trip2,
                    doub: doub2,
                },
            ) => {
                if trip1.label == trip2.label {
                    doub2.cmp(doub1)
                } else {
                    trip2.cmp(trip1)
                }
            }
            (Hand::FullHouse { .. }, _) => Ordering::Greater,
            (_, Hand::FullHouse { .. }) => Ordering::Less,
            (
                Hand::ThreeOfAKind {
                    trip: trip1,
                    rest: rest1,
                },
                Hand::ThreeOfAKind {
                    trip: trip2,
                    rest: rest2,
                },
            ) => {
                if trip1.label != trip2.label {
                    trip2.cmp(trip1)
                } else {
                    for i in 0..rest1.len() {
                        if rest1[i] != rest2[i] {
                            return rest2.cmp(rest1);
                        }
                    }
                    Ordering::Equal
                }
            }
            (Hand::ThreeOfAKind { .. }, _) => Ordering::Greater,
            (_, Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (
                Hand::TwoPair {
                    double_1: double_1_1,
                    double_2: double_2_1,
                    last: last1,
                },
                Hand::TwoPair {
                    double_1: double_1_2,
                    double_2: double_2_2,
                    last: last2,
                },
            ) => {
                if double_1_1.label != double_1_2.label {
                    double_1_2.cmp(double_1_1)
                } else if double_2_1.label != double_2_2.label {
                    double_2_2.cmp(double_2_1)
                } else {
                    last2.cmp(last1)
                }
            }
            (Hand::TwoPair { .. }, _) => Ordering::Greater,
            (_, Hand::TwoPair { .. }) => Ordering::Less,
            (
                Hand::OnePair {
                    double: double1,
                    rest: rest1,
                },
                Hand::OnePair {
                    double: double2,
                    rest: rest2,
                },
            ) => {
                if double1.label != double2.label {
                    double2.cmp(double1)
                } else {
                    for i in 0..rest1.len() {
                        if rest1[i] != rest2[i] {
                            return rest2.cmp(rest1);
                        }
                    }
                    Ordering::Equal
                }
            }
            (Hand::OnePair { .. }, _) => Ordering::Greater,
            (_, Hand::OnePair { .. }) => Ordering::Less,
            (Hand::HighCard { cards: cards1 }, Hand::HighCard { cards: cards2 }) => {
                for i in 0..cards1.len() {
                    if cards1[i] != cards2[i] {
                        return cards2.cmp(cards1);
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Multiples {
    card: Card,
    count: i32,
}

impl FromStr for Hand {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s.chars().map(Card::from_char).collect::<Vec<Card>>();
        cards.sort();

        let mut multiples: Vec<Multiples> = vec![];
        let mut current_multiples: Option<Multiples> = None;
        for card in cards.iter() {
            match current_multiples {
                None => {
                    current_multiples = Some(Multiples {
                        card: card.clone(),
                        count: 1,
                    })
                }
                Some(ref mut val) => {
                    if val.card == *card {
                        *val = Multiples {
                            count: val.count + 1,
                            card: val.card.clone(),
                        };
                    } else {
                        multiples.push(Multiples {
                            count: val.count,
                            card: val.card.clone(),
                        });
                        *val = Multiples {
                            card: card.clone(),
                            count: 1,
                        }
                    }
                }
            }
        }

        if let Some(val) = current_multiples {
            multiples.push(Multiples {
                count: val.count,
                card: val.card,
            });
        }

        multiples.sort_by(|a, b| b.count.cmp(&a.count));

        let hand = match &multiples[..] {
            [first] => Hand::FiveOfAKind {
                quint: first.card.clone(),
            },
            [first, second] => match first.count {
                4 => Hand::FourOfAKind {
                    quad: first.card.clone(),
                    single: second.card.clone(),
                },
                3 => Hand::FullHouse {
                    trip: first.card.clone(),
                    doub: second.card.clone(),
                },
                _ => Hand::HighCard {
                    cards: vec![first.card.clone(), second.card.clone()],
                },
            },
            [first, second, third] => match first.count {
                3 => Hand::ThreeOfAKind {
                    trip: first.card.clone(),
                    rest: vec![second.card.clone(), third.card.clone()],
                },
                2 => Hand::TwoPair {
                    double_1: first.card.clone(),
                    double_2: second.card.clone(),
                    last: third.card.clone(),
                },
                _ => Hand::HighCard {
                    cards: vec![first.card.clone(), second.card.clone()],
                },
            },
            [first, second, third, fourth] => Hand::OnePair {
                double: first.card.clone(),
                rest: vec![second.card.clone(), third.card.clone(), fourth.card.clone()],
            },
            _ => Hand::HighCard {
                cards: multiples.iter().map(|m| m.card.clone()).collect(),
            },
        };

        Ok(hand)
    }
}

#[derive(Debug, Eq)]
struct Round {
    hand: Hand,
    bid: i32,
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl FromStr for Round {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(' ').collect::<Vec<&str>>();

        Ok(Round {
            hand: Hand::from_str(sections[0]).unwrap(),
            bid: sections[1].parse::<i32>().unwrap(),
        })
    }
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day7.txt");

    let mut rounds: Vec<Round> = lines
        .map_while(Result::ok)
        .map(|l| Round::from_str(&l).unwrap())
        .collect();

    rounds.sort();

    let mut total = 0;
    for (rank, round) in rounds.iter().enumerate() {
        let good_rank: i32 = rank.try_into().unwrap();
        println!("{rank} {:?}", round);
        total += (good_rank + 1) * round.bid
    }

    println!("{:?}", total)
}
