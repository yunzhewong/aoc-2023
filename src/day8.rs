use core::panic;
use std::collections::HashMap;

use crate::filereading;

pub fn run() {
    a();
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

fn get_round_access_command(commands: &Vec<char>, index: usize) -> char {
    let mut get_index = index;
    while get_index >= commands.len() {
        get_index -= commands.len()
    }

    commands[get_index]
}

fn a() {
    let mut lines = filereading::get_lines("src/inputs/day8.txt");

    let commands = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    lines.next();

    let mut map: HashMap<String, Directions> = HashMap::new();

    for line in lines.map_while(Result::ok) {
        add_directions(&mut map, line);
    }

    let mut location = String::from("AAA");
    let mut command_index = 0;
    while location != "ZZZ" {
        let directions = map.get(&location).expect("Expected Directions");

        let command = get_round_access_command(&commands, command_index);

        match command {
            'L' => location.clone_from(&directions.left),
            'R' => location.clone_from(&directions.right),
            _ => panic!(),
        }

        command_index += 1;
    }

    println!("{command_index}")
}
