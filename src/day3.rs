use std::{collections::VecDeque, thread::current};

use crate::filereading;

fn check_alone(line_bytes: &Option<&[u8]>, number_start: usize, number_length: usize) -> bool {
    if line_bytes.is_none() {
        return true;
    }

    let line_bytes = line_bytes.unwrap();

    let start = if number_start == 0 {
        0
    } else {
        number_start - 1
    };
    let number_end = number_start + number_length;
    let end = if number_end >= line_bytes.len() - 1 {
        number_end
    } else {
        number_end + 1
    };
    let search_bytes = &line_bytes[start..end];
    for byte_ref in search_bytes {
        let search_char = *byte_ref as char;
        if !search_char.is_numeric() && search_char != '.' {
            return false;
        }
    }
    true
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day3.txt")
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    for (line_index, line) in lines.iter().enumerate() {
        let previous_bytes = if line_index == 0 {
            None
        } else {
            lines.get(line_index - 1).map(|l| l.as_bytes())
        };
        let next_bytes = lines.get(line_index + 1).map(|l| l.as_bytes());

        let mut char_index = 0;
        let bytes = line.as_bytes();
        while char_index < line.len() {
            let current_char = bytes[char_index] as char;
            let parse_check = current_char.to_string().parse::<usize>();

            if parse_check.is_err() {
                char_index += 1;
                continue;
            }

            let mut number_length = 1;
            let mut value = parse_check.unwrap();
            loop {
                let check_index = char_index + number_length;
                if check_index > line.len() - 1 {
                    break;
                }
                let parse_res = (bytes[check_index] as char).to_string().parse::<usize>();
                match parse_res {
                    Ok(next_val) => {
                        value = 10 * value + next_val;
                        number_length += 1
                    }
                    Err(_) => break,
                }
            }

            let previous_alone = check_alone(&previous_bytes, char_index, number_length);
            let next_alone = check_alone(&next_bytes, char_index, number_length);
            let mut alone = previous_alone || next_alone;
            println!("{value} {alone}");

            char_index += number_length
        }
    }
}

pub fn run() {
    a();
}
