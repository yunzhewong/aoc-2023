use std::{collections::VecDeque, thread::current};

use crate::filereading;

fn found_other(lines: &[String], index: i32, start: usize, end: usize) -> bool {
    if index < 0 || index > lines.len() as i32 - 1 {
        return false;
    }

    let safe_index = index as usize;
    let search_bytes = &lines[safe_index].as_bytes()[start..end];
    for byte_ref in search_bytes {
        let search_char = *byte_ref as char;
        if !search_char.is_numeric() && search_char != '.' {
            return true;
        }
    }
    false
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day3.txt")
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let mut sum = 0;

    for (line_index, line) in lines.iter().enumerate() {
        let mut char_index = 0;
        let bytes = line.as_bytes();
        while char_index < line.len() {
            let current_char = bytes[char_index] as char;

            if !current_char.is_numeric() {
                char_index += 1;
                continue;
            }

            let mut number_string: Vec<char> = vec![current_char];
            loop {
                let check_index = char_index + number_string.len();
                if check_index > line.len() - 1 {
                    break;
                }
                let next_char = bytes[check_index] as char;
                if !next_char.is_numeric() {
                    break;
                }
                number_string.push(next_char)
            }

            let start = if char_index > 0 { char_index - 1 } else { 0 };
            let possible_end = char_index + number_string.len() + 1;
            let end = if possible_end >= line.len() {
                line.len() - 1
            } else {
                possible_end
            };

            let previous_other = found_other(&lines, line_index as i32 - 1, start, end);
            let current_other = found_other(&lines, line_index as i32, start, end);
            let next_other = found_other(&lines, line_index as i32 + 1, start, end);
            let other = previous_other || current_other || next_other;

            if other {
                let collected_string = number_string.iter().collect::<String>();
                let number: i32 = collected_string.parse().expect("Number should be parsable");
                sum += number;
            }

            char_index += number_string.len()
        }
    }
    println!("{sum}")
}

pub fn run() {
    a();
}
