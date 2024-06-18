#![allow(dead_code)]
use crate::filereading;

pub fn run() {
    b();
}

struct Race {
    time: i64,
    distance: i64,
}

fn extract_numbers(line: String) -> Vec<i32> {
    let sections: Vec<&str> = line.split(':').collect();

    let numbers = sections[1]
        .trim()
        .split(' ')
        .filter_map(|res| res.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    numbers
}

fn get_times_distances() -> (Vec<i32>, Vec<i32>) {
    let mut lines = filereading::get_lines("src/inputs/day6.txt");

    let times = extract_numbers(lines.next().unwrap().unwrap());
    let distances = extract_numbers(lines.next().unwrap().unwrap());

    (times, distances)
}

fn get_races() -> Vec<Race> {
    let (times, distances) = get_times_distances();
    times
        .into_iter()
        .enumerate()
        .map(|(i, time)| Race {
            time: time.into(),
            distance: distances[i].into(),
        })
        .collect::<Vec<Race>>()
}

fn a() {
    let races = get_races();

    let mut product = 1;

    for race in races {
        let mut would_win = 0;

        for charge_time in 1..race.time {
            let running_time = race.time - charge_time;
            let speed = charge_time;

            let distance = running_time * speed;

            if distance > race.distance {
                would_win += 1;
            }
        }

        product *= would_win
    }

    println!("{product}")
}

fn aggregate_numbers(numbers: Vec<i32>) -> i64 {
    let mut final_string = String::from("");

    for number in numbers.iter() {
        final_string.push_str(&number.to_string())
    }

    final_string.parse::<i64>().unwrap()
}

fn b() {
    let (times, distances) = get_times_distances();

    let total_time = aggregate_numbers(times);
    let total_distance = aggregate_numbers(distances);

    println!("{total_time}, {total_distance}");

    let mut would_win = 0;

    for charge_time in 1..total_time {
        let running_time = total_time - charge_time;
        let speed = charge_time;

        let distance = running_time * speed;

        if distance > total_distance {
            would_win += 1;
        }
    }

    println!("{would_win}")
}
