use std::collections::VecDeque;

use crate::filereading;

fn get_numbers(line: &str) -> (Vec<i32>, Vec<i32>) {
    let card_value = line.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let user_numbers = card_value[0]
        .split(' ')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<i32>>();

    let winning_numbers = card_value[1]
        .split(' ')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<i32>>();

    (user_numbers, winning_numbers)
}

fn identify_wins(user_numbers: Vec<i32>, winning_numbers: Vec<i32>) -> i32 {
    let mut winning_count = 0;
    for number in winning_numbers {
        for &user_number in &user_numbers {
            if number == user_number {
                winning_count += 1;
            }
        }
    }
    winning_count
}

fn a() {
    let lines = filereading::get_lines("src/inputs/day4.txt");

    let mut sum = 0;
    for line in lines.map_while(Result::ok) {
        let (user_numbers, winning_numbers) = get_numbers(&line);
        let winning_count = identify_wins(user_numbers, winning_numbers);
        if winning_count > 0 {
            sum += 2_i32.pow(winning_count as u32 - 1);
        }
    }
    println!("{sum}")
}

fn b() {
    let lines = filereading::get_lines("src/inputs/day4.txt");

    let mut count = 0;
    let mut copies: VecDeque<usize> = VecDeque::new();
    for line in lines.map_while(Result::ok) {
        let (user_numbers, winning_numbers) = get_numbers(&line);
        let winning_count = identify_wins(user_numbers, winning_numbers);

        let card_copies = copies.pop_front().unwrap_or(1);
        if winning_count > 0 {
            for i in 0..winning_count {
                match copies.get(i as usize) {
                    Some(_) => copies[i as usize] += card_copies,
                    None => copies.push_back(card_copies + 1),
                }
            }
        }
        count += card_copies;
    }
    println!("{count}")
}

pub fn run() {
    b();
}
