use crate::filereading;

pub fn run() {
    a()
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    row_index: i32,
    col_index: i32,
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

fn get_exit_direction(character: char, entry_direction: Direction) -> Option<Direction> {
    let direction = match character {
        '|' => match entry_direction {
            Direction::Up | Direction::Down => entry_direction,
            _ => return None,
        },
        '-' => match entry_direction {
            Direction::Left | Direction::Right => entry_direction,
            _ => return None,
        },
        'L' => match entry_direction {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => return None,
        },
        'J' => match entry_direction {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => return None,
        },
        '7' => match entry_direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            _ => return None,
        },
        'F' => match entry_direction {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => return None,
        },
        _ => return None,
    };

    Some(direction)
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

fn a() {
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

    let start_position = start_position.unwrap();

    let options = [
        Direction::Down,
        Direction::Right,
        Direction::Up,
        Direction::Left,
    ];

    let mut first_step: Option<(Position, Direction)> = None;
    for option in options {
        let next_position = move_in_direction(&start_position, &option);
        let char = get_char(&map, &next_position);
        if char.is_none() {
            continue;
        }
        let next_direction = get_exit_direction(char.unwrap(), option.clone());

        if next_direction.is_some() {
            first_step = Some((start_position.clone(), option))
        }
    }

    let (mut current_pos, mut entry_dir) = first_step.unwrap();
    let mut step_count = 0;

    loop {
        let next_position = move_in_direction(&current_pos, &entry_dir);
        if next_position == start_position {
            step_count += 1;
            break;
        }
        let char = get_char(&map, &next_position).unwrap();

        // println!("{:?} {:?}", char, next_position);
        let next_direction = get_exit_direction(char, entry_dir.clone()).expect("Expected");

        current_pos = next_position;
        entry_dir = next_direction;
        step_count += 1;
    }

    let path = step_count / 2;
    println!("{path}");
}
