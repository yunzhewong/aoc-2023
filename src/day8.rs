use core::panic;
use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

use crate::filereading;

pub fn run() {
    b();
}

struct Directions {
    left: String,
    right: String,
}

fn add_directions(map: &mut HashMap<String, Directions>, line: String) {
    let key = String::from(&line[0..3]);

    let left: String = String::from(&line[7..10]);
    let right: String = String::from(&line[12..15]);

    map.insert(key, Directions { left, right });
}

fn get_round_access_command(commands: &[char], index: usize) -> char {
    let mut get_index = index;
    while get_index >= commands.len() {
        get_index -= commands.len()
    }

    commands[get_index]
}

fn read_commands(lines: &mut Lines<BufReader<File>>) -> Vec<char> {
    let commands = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    lines.next();

    commands
}

fn construct_map(lines: Lines<BufReader<File>>) -> HashMap<String, Directions> {
    let mut map: HashMap<String, Directions> = HashMap::new();

    for line in lines.map_while(Result::ok) {
        add_directions(&mut map, line);
    }

    map
}

fn get_next_direction(location: &str, map: &HashMap<String, Directions>, command: char) -> String {
    let directions = map.get(location).expect("Expected Directions");

    match command {
        'L' => String::from(&directions.left),
        'R' => String::from(&directions.right),
        _ => panic!(),
    }
}

fn a() {
    let mut lines = filereading::get_lines("src/inputs/day8.txt");

    let commands = read_commands(&mut lines);
    let map = construct_map(lines);

    let mut location = String::from("AAA");
    let mut command_index = 0;
    while location != "ZZZ" {
        let command = get_round_access_command(&commands, command_index);
        location = get_next_direction(&location, &map, command);

        command_index += 1;
    }

    println!("{command_index}")
}

fn get_final_char(location: &str) -> char {
    location.chars().nth(2).expect("3 Characters long")
}

fn at_target_locations(locations: &Vec<String>) -> bool {
    for location in locations {
        if get_final_char(location) != 'Z' {
            return false;
        }
    }
    true
}

fn check_encountered(encountered_positions: &Vec<(String, usize)>, current: &str) -> Option<usize> {
    for (old_location, index) in encountered_positions {
        if old_location == current {
            return Some(*index);
        }
    }
    None
}

#[derive(Clone, Debug)]
struct Loop {
    first_encounter: usize,
    length: usize,
}

fn loops_found(loops: &[Option<Loop>]) -> bool {
    let flattened: Vec<_> = loops.iter().flatten().collect();
    flattened.len() == loops.len()
}

fn valid_iteration(loops: &[Loop], iteration: usize) -> bool {
    for loop_info in loops {
        if iteration - loop_info.first_encounter % loop_info.length != 0 {
            return false;
        }
    }
    true
}

fn b() {
    let mut lines = filereading::get_lines("src/inputs/day8.txt");

    let commands = read_commands(&mut lines);
    let map = construct_map(lines);

    let mut locations: Vec<String> = vec![];
    for key in map.keys() {
        if get_final_char(key) == 'A' {
            locations.push(String::from(key))
        }
    }

    let mut encountered_positions: Vec<Vec<(String, usize)>> = vec![vec![]; locations.len()];
    let mut command_index = 0;
    let mut loops: Vec<Option<Loop>> = vec![None; locations.len()];
    println!("{:?}", locations);

    while !at_target_locations(&locations) {
        let command = get_round_access_command(&commands, command_index);

        for (i, location) in locations.iter_mut().enumerate() {
            *location = get_next_direction(location, &map, command);

            if get_final_char(location) == 'Z' {
                match check_encountered(&encountered_positions[i], location) {
                    Some(first_encounter) => {
                        if loops[i].is_none() {
                            loops[i] = Some(Loop {
                                first_encounter,
                                length: command_index - first_encounter,
                            })
                        }
                    }
                    None => encountered_positions[i].push((location.to_string(), command_index)),
                }
            }
        }

        if loops_found(&loops) {
            break;
        }

        command_index += 1;
    }

    let mut loops = loops.into_iter().map(|f| f.unwrap()).collect::<Vec<Loop>>();

    let loop1 = &loops[0];
    let loop2 = &loops[1];

    println!("{:?}, {:?}", loop1, loop2);

    let length_diff = loop1.length as i32 - loop2.length as i32;
    let start_diff = loop2.first_encounter as i32 - loop1.first_encounter as i32;

    let div = {
        let res = start_diff / length_diff;

        if res > 0 {
            res as usize
        } else {
            (start_diff - res).unsigned_abs() as usize
        }
    };

    println!("{length_diff} {start_diff} {div}");

    let final_value = loop2.first_encounter + div * loop2.length;

    println!("{final_value}");

    let remainder = (final_value - loop1.first_encounter) % loop1.length;

    println!("{remainder}")

    // start1, length1
    // start2, length2

    // 0 length1,
    // start2 - start1, length2

    // a * length1 mod length2 = start2 - start1

    // a * length1 =  k * length2  +  (start2 - start1)
}
