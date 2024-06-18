#![allow(dead_code)]
use std::{cmp::Ordering, str::FromStr};

use crate::filereading;

pub fn run() {
    b();
}

const CARD_OPTIONS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARD_OPTIONS_WITH_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn create_card(label: char, options: &[char]) -> Card {
    for (i, option) in options.iter().enumerate() {
        if *option == label {
            return Card { label: i };
        }
    }

    panic!();
}

fn identify_multiples(cards: Vec<Card>) -> Vec<Multiples> {
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

    multiples
}

fn get_hand_type(multiples: Vec<Multiples>) -> HandType {
    match &multiples[..] {
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
    }
}

fn group_cards(mut cards: Vec<Card>) -> (Vec<Card>, Vec<Multiples>) {
    let original_copy = cards.clone();
    cards.sort();

    let mut multiples = identify_multiples(cards);
    multiples.sort_by(|a, b| b.count.cmp(&a.count));

    (original_copy, multiples)
}

fn parse_cards(s: &str, options: &[char]) -> Vec<Card> {
    s.chars()
        .map(|c| create_card(c, options))
        .collect::<Vec<Card>>()
}

fn create_basic_hand(s: &str) -> Hand {
    let cards = parse_cards(s, &CARD_OPTIONS);
    let (original_copy, multiples) = group_cards(cards);
    let hand_type = get_hand_type(multiples);

    Hand {
        hand_type,
        cards: original_copy,
    }
}

fn create_joker_hand(s: &str) -> Hand {
    let cards = parse_cards(s, &CARD_OPTIONS_WITH_JOKER);
    let (original_copy, mut multiples) = group_cards(cards);

    let mut joker_index = None;
    for (i, multiple) in multiples.iter().enumerate() {
        if multiple.card.label == CARD_OPTIONS_WITH_JOKER.len() - 1 {
            joker_index = Some(i);
        }
    }

    if let Some(i) = joker_index {
        if multiples.len() != 1 {
            let joker_multiple = multiples.remove(i);

            multiples[0].count += joker_multiple.count
        }
    }

    let hand_type = get_hand_type(multiples);

    Hand {
        hand_type,
        cards: original_copy,
    }
}

fn identify_round_sections(s: &str) -> (&str, i32) {
    let sections = s.split(' ').collect::<Vec<&str>>();

    let hand_section = sections[0];
    let bid_section = sections[1].parse::<i32>().unwrap();

    (hand_section, bid_section)
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Eq, Clone, PartialEq)]
struct Card {
    label: usize,
}

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
            hand: create_basic_hand(sections[0]),
            bid: sections[1].parse::<i32>().unwrap(),
        })
    }
}

fn count_rounds(mut rounds: Vec<Round>) {
    rounds.sort();
    rounds.reverse();

    for round in rounds.iter() {
        println!("{:?}", round);
    }

    let mut total = 0;
    for (rank, round) in rounds.iter().enumerate() {
        let good_rank: i32 = rank.try_into().unwrap();
        total += (good_rank + 1) * round.bid
    }

    println!("{:?}", total)
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day7.txt");

    let rounds: Vec<Round> = lines
        .map_while(Result::ok)
        .map(|l| {
            let (hand_section, bid) = identify_round_sections(&l);
            Round {
                hand: create_basic_hand(hand_section),
                bid,
            }
        })
        .collect();

    count_rounds(rounds)
}

fn b() {
    let lines = filereading::get_lines("src/inputs/day7.txt");

    let rounds: Vec<Round> = lines
        .map_while(Result::ok)
        .map(|l| {
            let (hand_section, bid) = identify_round_sections(&l);
            Round {
                hand: create_joker_hand(hand_section),
                bid,
            }
        })
        .collect();

    count_rounds(rounds)
}
