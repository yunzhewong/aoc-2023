#![allow(dead_code)]

use crate::filereading;

const PATH: &str = "src/inputs/day1.txt";
struct NumberPair {
    first: usize,
    last: usize,
}

impl NumberPair {
    fn first_val(first: usize) -> NumberPair {
        NumberPair { first, last: first }
    }

    fn update_val(&mut self, new_val: usize) {
        *self = NumberPair {
            last: new_val,
            ..*self
        }
    }

    fn get_total(&self) -> usize {
        self.first * 10 + self.last
    }
}

fn a() {
    let mut sum = 0;

    let lines = filereading::get_lines(PATH);
    for line in lines.map_while(Result::ok) {
        let mut possible_pair: Option<NumberPair> = None;
        for character in line.chars() {
            let parsed_number = character.to_string().parse::<usize>();

            if let Ok(val) = parsed_number {
                match possible_pair {
                    Some(ref mut pair) => pair.update_val(val),
                    None => possible_pair = Some(NumberPair::first_val(val)),
                }
            }
        }

        if let Some(pair) = possible_pair {
            sum += pair.get_total()
        }
    }
    println!("{sum}")
}

fn update_possible_pair(possible_pair: &mut Option<NumberPair>, val: usize) {
    match possible_pair {
        Some(ref mut pair) => pair.update_val(val),
        None => *possible_pair = Some(NumberPair::first_val(val)),
    }
}

fn b() {
    let mut sum = 0;
    let options = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = filereading::get_lines(PATH);
    for line in lines.map_while(Result::ok) {
        let mut possible_pair: Option<NumberPair> = None;

        let chars: Vec<char> = line.chars().collect();
        let mut char_index = 0;
        while char_index < chars.len() {
            let character = chars[char_index];
            let mut string_matched = false;

            for (i, option) in options.iter().enumerate() {
                let end = char_index + option.len();
                if end > chars.len() {
                    continue;
                }
                let section = chars[char_index..end].iter().collect::<String>();
                if *option == section {
                    let value = i + 1;
                    update_possible_pair(&mut possible_pair, value);
                    char_index += option.len() - 1;
                    string_matched = true;
                    break;
                }
            }

            if string_matched {
                continue;
            }

            let parsed_value = character.to_string().parse::<usize>();
            if let Ok(val) = parsed_value {
                update_possible_pair(&mut possible_pair, val)
            }
            char_index += 1
        }

        if let Some(pair) = possible_pair {
            sum += pair.get_total()
        }
    }

    println!("{sum}")
}

pub fn run() {
    b()
}
