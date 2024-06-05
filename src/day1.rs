use crate::read;

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

    let lines = read::get_lines(PATH);
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

fn b() {
    let mut sum = 0;
    let options = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = read::get_lines(PATH);
    for line in lines.map_while(Result::ok) {
        let mut possible_pair: Option<NumberPair> = None;

        let chars: Vec<char> = line.chars().collect();
        for (start, character) in chars.iter().enumerate() {
            for (option_index, option) in options.iter().enumerate() {
                let end = start + option.len();
                if end > chars.len() {
                    continue;
                }
                let section = chars[start..end].iter().collect::<String>();
                if *option == section {
                    let value = option_index + 1;
                    match possible_pair {
                        Some(ref mut pair) => pair.update_val(value),
                        None => possible_pair = Some(NumberPair::first_val(value)),
                    }
                }
            }

            let parsed_value = character.to_string().parse::<usize>();

            if let Ok(val) = parsed_value {
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

pub fn run() {
    b()
}
