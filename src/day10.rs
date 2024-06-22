use core::panic;
use std::collections::{hash_set, HashSet};

use crate::filereading;

pub fn run() {
    b()
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    row_index: i32,
    col_index: i32,
}

impl Position {
    fn str(&self) -> String {
        format!("{}_{}", self.row_index, self.col_index)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_char(map: &[String], position: &Position) -> Option<char> {
    if position.row_index < 0 || position.col_index < 0 {
        return None;
    }

    let row = position.row_index as usize;
    let col = position.col_index as usize;

    let str = map.get(row);
    match str {
        None => None,
        Some(res) => res.chars().nth(col),
    }
}
#[derive(Debug)]
enum IncorrectEntry {
    WrongDirection,
    Ignore,
}

fn get_exit_direction(
    character: char,
    entry_direction: Direction,
) -> Result<Direction, IncorrectEntry> {
    let direction = match character {
        '|' => match entry_direction {
            Direction::Up | Direction::Down => entry_direction,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        '-' => match entry_direction {
            Direction::Left | Direction::Right => entry_direction,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        'L' => match entry_direction {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        'J' => match entry_direction {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        '7' => match entry_direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        'F' => match entry_direction {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => return Err(IncorrectEntry::WrongDirection),
        },
        _ => return Err(IncorrectEntry::Ignore),
    };

    Ok(direction)
}

fn move_in_direction(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position {
            row_index: position.row_index - 1,
            ..position.clone()
        },
        Direction::Down => Position {
            row_index: position.row_index + 1,
            ..position.clone()
        },
        Direction::Right => Position {
            col_index: position.col_index + 1,
            ..position.clone()
        },
        Direction::Left => Position {
            col_index: position.col_index - 1,
            ..position.clone()
        },
    }
}

fn read_map() -> (Vec<String>, Position) {
    let lines = filereading::get_lines("src/inputs/day10.txt");

    let mut map: Vec<String> = vec![];
    let mut start_position: Option<Position> = None;
    for (line_index, line) in lines.map_while(Result::ok).enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == 'S' {
                start_position = Some(Position {
                    row_index: line_index as i32,
                    col_index: char_index as i32,
                });
            }
        }
        map.push(line);
    }

    (map, start_position.unwrap())
}

fn identify_first_move(map: &[String], start_position: &Position) -> (Position, Direction) {
    let options = [
        Direction::Down,
        Direction::Right,
        Direction::Up,
        Direction::Left,
    ];

    let mut first_step: Option<(Position, Direction)> = None;
    for option in options {
        let next_position = move_in_direction(start_position, &option);
        let char = get_char(map, &next_position);
        if char.is_none() {
            continue;
        }
        let next_direction = get_exit_direction(char.unwrap(), option.clone());

        if next_direction.is_ok() {
            first_step = Some((start_position.clone(), option))
        }
    }

    first_step.unwrap()
}

fn a() {
    let (map, start_position) = read_map();
    let (mut current_position, mut current_direction) = identify_first_move(&map, &start_position);

    let mut step_count = 0;

    loop {
        let next_position = move_in_direction(&current_position, &current_direction);
        if next_position == start_position {
            step_count += 1;
            break;
        }
        let char = get_char(&map, &next_position).unwrap();

        let next_direction = get_exit_direction(char, current_direction.clone()).expect("Expected");

        current_position = next_position;
        current_direction = next_direction;
        step_count += 1;
    }

    let path = step_count / 2;
    println!("{path}");
}

fn identify_start_character(map: &[String], start_position: &Position) -> char {
    let mut options: Vec<Direction> = vec![];

    if check_matching_options(map, start_position, Direction::Down, &['|', 'L', 'J']) {
        options.push(Direction::Down)
    }

    if check_matching_options(map, start_position, Direction::Up, &['|', '7', 'F']) {
        options.push(Direction::Up)
    }

    if check_matching_options(map, start_position, Direction::Right, &['-', 'J', '7']) {
        options.push(Direction::Right)
    }

    if check_matching_options(map, start_position, Direction::Left, &['F', 'L', '-']) {
        options.push(Direction::Left)
    }

    match &options[..] {
        [Direction::Down, Direction::Right] => 'F',
        [Direction::Down, Direction::Up] => '|',
        [Direction::Down, Direction::Left] => '7',
        [Direction::Up, Direction::Right] => 'L',
        [Direction::Up, Direction::Left] => 'J',
        [Direction::Right, Direction::Left] => '-',
        _ => panic!(),
    }
}

fn check_matching_options(
    map: &[String],
    start_position: &Position,
    direction: Direction,
    options: &[char],
) -> bool {
    let next_position = move_in_direction(start_position, &direction);
    let next_char = get_char(map, &next_position);

    if let Some(res) = next_char {
        if options.contains(&res) {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
enum LoopState {
    Border,
    Inside,
    Outside,
}

fn b() {
    let (map, start_position) = read_map();
    let (mut current_position, mut current_direction) = identify_first_move(&map, &start_position);
    let mut hashset: HashSet<String> = HashSet::new();

    loop {
        let next_position = move_in_direction(&current_position, &current_direction);
        if next_position == start_position {
            break;
        }

        hashset.insert(next_position.str());
        let char = get_char(&map, &next_position).unwrap();

        let next_direction = get_exit_direction(char, current_direction.clone()).expect("Expected");

        current_position = next_position;
        current_direction = next_direction;
    }

    let start_char = identify_start_character(&map, &start_position);
    hashset.insert(start_position.str());

    let mut count = 0;
    for (row_index, line) in map.iter().enumerate() {
        let mut previous_char = '.';
        let mut previous_state = LoopState::Outside;
        let mut state = LoopState::Outside;
        println!("NEW LINE");
        for (col_index, char) in line.chars().enumerate() {
            let position = Position {
                row_index: row_index as i32,
                col_index: col_index as i32,
            };

            let on_loop = hashset.contains(&position.str());

            // println!("{char} {:?}", state);

            if !on_loop {
                if let LoopState::Inside = state {
                    count += 1;
                }

                continue;
            }

            let mut check_char = char;
            if check_char == 'S' {
                check_char = start_char;
            }

            state = match check_char {
                '|' => match state {
                    LoopState::Inside => LoopState::Outside,
                    LoopState::Outside => LoopState::Inside,
                    _ => panic!(),
                },
                '-' => match state {
                    LoopState::Border => LoopState::Border,
                    _ => panic!(),
                },
                'L' | 'F' => {
                    previous_char = check_char;
                    previous_state = state.clone();
                    match state {
                        LoopState::Outside => LoopState::Border,
                        LoopState::Inside => LoopState::Border,
                        _ => panic!(),
                    }
                }
                'J' => match state {
                    LoopState::Border => {
                        if previous_char == 'L' {
                            previous_state.clone()
                        } else {
                            match previous_state.clone() {
                                LoopState::Inside => LoopState::Outside,
                                LoopState::Outside => LoopState::Inside,
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                },
                '7' => match state {
                    LoopState::Border => {
                        if previous_char == 'F' {
                            previous_state.clone()
                        } else {
                            match previous_state.clone() {
                                LoopState::Inside => LoopState::Outside,
                                LoopState::Outside => LoopState::Inside,
                                _ => panic!(),
                            }
                        }
                    }
                    _ => panic!(),
                },
                _ => state,
            };
        }
    }

    println!("{count}")
}
