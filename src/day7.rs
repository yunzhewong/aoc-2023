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

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp(&other.cards[i]);
                }
            }
            Ordering::Equal
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

        let original_copy = cards.clone();
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

        let handType = match &multiples[..] {
            [_] => HandType::FiveOfAKind,
            [first, _] => match first.count {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => HandType::HighCard,
            },
            [first, _, _] => match first.count {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => HandType::HighCard,
            },
            [_, _, _, _] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Ok(Hand {
            hand_type: handType,
            cards: original_copy,
        })
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
    rounds.reverse();

    let mut total = 0;
    for (rank, round) in rounds.iter().enumerate() {
        let good_rank: i32 = rank.try_into().unwrap();
        println!("{rank} {:?}", round);
        total += (good_rank + 1) * round.bid
    }

    println!("{:?}", total)
}
