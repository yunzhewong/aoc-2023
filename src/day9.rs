#![allow(dead_code)]
use crate::filereading;

pub fn run() {
    b()
}

fn complete(row: &Vec<i32>) -> bool {
    for number in row {
        if *number != 0 {
            return false;
        }
    }
    true
}

fn define_history(line: String) -> Vec<Vec<i32>> {
    let numbers = line
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut rows = vec![numbers];

    while !complete(&rows[rows.len() - 1]) {
        let mut next_row: Vec<i32> = vec![];
        let current_row = &rows[rows.len() - 1];

        for i in 0..current_row.len() - 1 {
            let difference = current_row[i + 1] - current_row[i];
            next_row.push(difference);
        }

        rows.push(next_row);
    }

    rows
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day9.txt");

    let mut total = 0;
    for line in lines.map_while(Result::ok) {
        let mut rows = define_history(line);

        rows.reverse();

        let mut iterable_rows = rows.into_iter();

        let mut last_difference = 0;
        iterable_rows.next();
        for row in iterable_rows {
            let final_value = row[row.len() - 1];
            last_difference += final_value;
        }

        total += last_difference
    }
    println!("{total}")
}

fn b() {
    let lines = filereading::get_lines("src/inputs/day9.txt");

    let mut total = 0;
    for line in lines.map_while(Result::ok) {
        let mut rows = define_history(line);

        rows.reverse();

        let mut iterable_rows = rows.into_iter();

        let mut first_difference = 0;
        iterable_rows.next();
        for row in iterable_rows {
            let final_value = row.first().unwrap();
            first_difference = final_value - first_difference;
        }

        total += first_difference
    }
    println!("{total}")
}
