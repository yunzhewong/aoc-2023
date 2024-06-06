#![allow(dead_code)]

use std::str::FromStr;

use crate::filereading;

fn extract_id(line: String) -> (usize, String) {
    let colon_separated = line.split(':').collect::<Vec<&str>>();
    let first_section = colon_separated.first().expect("Expected a section");

    let rest = colon_separated[1..colon_separated.len()].join(":");
    let space_separted_first_section = first_section.split(' ').collect::<Vec<&str>>();

    let id_string = space_separted_first_section.get(1).expect("Expected Id");
    let parsed_id = id_string.parse::<usize>().expect("ID not parsable");

    (parsed_id, rest)
}

#[derive(Debug)]
struct GameRound {
    red: usize,
    green: usize,
    blue: usize,
}

fn maximum(x1: usize, x2: usize) -> usize {
    if x1 > x2 {
        x1
    } else {
        x2
    }
}

impl GameRound {
    fn get_minimum(&self, other: &GameRound) -> GameRound {
        let minimum_blue = maximum(self.blue, other.blue);
        let minimum_red = maximum(self.red, other.red);
        let minimum_green = maximum(self.green, other.green);

        GameRound {
            red: minimum_red,
            blue: minimum_blue,
            green: minimum_green,
        }
    }

    fn larger_than(&self, other: &GameRound) -> bool {
        if self.blue > other.blue {
            return true;
        }

        if self.red > other.red {
            return true;
        }

        if self.green > other.green {
            return true;
        }

        false
    }

    fn get_power(&self) -> usize {
        self.red * self.blue * self.green
    }
}
#[derive(Debug)]
struct ParseRoundError;
impl FromStr for GameRound {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let balls = s.split(',');
        let mut round = GameRound {
            red: 0,
            green: 0,
            blue: 0,
        };
        for ball in balls {
            let strings = ball.trim().split(' ').collect::<Vec<&str>>();
            let number_string = strings.first().ok_or(ParseRoundError)?;
            let number = number_string
                .parse::<usize>()
                .map_err(|_| ParseRoundError)?;
            let color = strings.get(1).ok_or(ParseRoundError)?;

            match *color {
                "red" => round.red = number,
                "green" => round.green = number,
                "blue" => round.blue = number,
                _ => return Err(ParseRoundError),
            }
        }

        Ok(round)
    }
}

const MAX_BALLS: GameRound = GameRound {
    red: 12,
    green: 13,
    blue: 14,
};

fn a() {
    let lines = filereading::get_lines("src/inputs/day2.txt");

    let mut sum: usize = 0;

    for line in lines.into_iter() {
        let (id, rest) = extract_id(line.expect("Line does not exist"));

        let rounds = rest
            .split(';')
            .map(|round| GameRound::from_str(round).expect("Round not parsed"));

        let mut impossible = false;
        for round in rounds {
            if round.larger_than(&MAX_BALLS) {
                impossible = true;
                break;
            }
        }

        if !impossible {
            sum += id
        }
    }

    println!("{sum}")
}

fn b() {
    let lines = filereading::get_lines("src/inputs/day2.txt");

    let mut sum: usize = 0;

    for line in lines.into_iter() {
        let (_id, rest) = extract_id(line.expect("Line does not exist"));

        let mut rounds = rest
            .split(';')
            .map(|round| GameRound::from_str(round).expect("Round not parsed"));

        let mut lowest_round = rounds.next().expect("At least one round expected");
        for round in rounds {
            lowest_round = lowest_round.get_minimum(&round)
        }
        sum += lowest_round.get_power()
    }

    println!("{sum}")
}
pub fn run() {
    b()
}
