use std::collections::HashSet;

use crate::filereading;

pub fn run() {
    a()
}
#[derive(Debug)]
struct Universe {
    orig_row: i32,
    orig_col: i32,
    row: i32,
    col: i32,
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day11.txt");
    let line_strings = lines.map_while(Result::ok).collect::<Vec<String>>();

    let total_rows = line_strings.len();
    let total_cols = line_strings.first().unwrap().len();

    let mut rows: HashSet<usize> = HashSet::from_iter(0..total_rows);
    let mut cols: HashSet<usize> = HashSet::from_iter(0..total_cols);

    let mut universes: Vec<Universe> = vec![];

    for (row_index, line) in line_strings.iter().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            if char == '#' {
                if rows.contains(&row_index) {
                    rows.remove(&row_index);
                }

                if cols.contains(&col_index) {
                    cols.remove(&col_index);
                }

                universes.push(Universe {
                    orig_row: row_index as i32,
                    orig_col: col_index as i32,
                    row: row_index as i32,
                    col: col_index as i32,
                })
            }
        }
    }

    for row in rows {
        for universe in universes.iter_mut() {
            if universe.orig_row > (row as i32) {
                universe.row += 1;
            }
        }
    }

    for col in cols {
        for universe in universes.iter_mut() {
            if universe.orig_col > (col as i32) {
                universe.col += 1;
            }
        }
    }

    let mut total = 0;
    for pair_1_index in 0..universes.len() {
        for pair_2_index in (pair_1_index + 1)..universes.len() {
            let universe_1 = &universes[pair_1_index];
            let universe_2 = &universes[pair_2_index];

            let distance =
                (universe_2.col - universe_1.col).abs() + (universe_2.row - universe_1.row).abs();
            total += distance;
        }
    }
    println!("{total}");
}
