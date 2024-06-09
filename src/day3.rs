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

fn search_step(chars: &[char], start: usize, step: i32) -> usize {
    let mut index = start as i32 + step;

    loop {
        let char = match chars.get(index as usize) {
            None => break,
            Some(res) => res,
        };

        if !char.is_numeric() {
            break;
        }

        index += step;
    }

    (index - step) as usize
}

fn search_nearby_number(line: Option<&String>, index: usize, matched_numbers: &mut Vec<i32>) {
    if line.is_none() {
        return;
    }

    let chars = line
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&b| b as char)
        .collect::<Vec<char>>();

    let start_index = if index == 0 { 0 } else { index - 1 };
    let end_index = if index == chars.len() - 1 {
        chars.len() - 1
    } else {
        index + 1
    };

    let mut check_index = start_index;

    while check_index < end_index + 1 {
        let check_char = match chars.get(check_index) {
            None => break,
            Some(res) => res,
        };

        if !check_char.is_numeric() {
            check_index += 1;
            continue;
        }

        let search_end_index = search_step(&chars, check_index, 1);
        let search_start_index = search_step(&chars, check_index, -1);

        let number = &chars[search_start_index..search_end_index + 1]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("Parsable");

        matched_numbers.push(*number);

        check_index = search_end_index + 1;
    }
}

fn b() {
    let lines = filereading::get_lines("src/inputs/day3.txt")
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let mut sum = 0;

    for (line_index, line) in lines.iter().enumerate() {
        let bytes = line.as_bytes();

        for (char_index, byte) in bytes.iter().enumerate() {
            let character = *byte as char;

            if character != '*' {
                continue;
            }

            let mut matched_numbers: Vec<i32> = vec![];

            search_nearby_number(lines.get(line_index - 1), char_index, &mut matched_numbers);
            search_nearby_number(Some(line), char_index, &mut matched_numbers);
            search_nearby_number(lines.get(line_index + 1), char_index, &mut matched_numbers);

            if matched_numbers.len() == 2 {
                sum += matched_numbers[0] * matched_numbers[1];
            }
        }
    }
    println!("{sum}")
}

pub fn run() {
    b();
}
