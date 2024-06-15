use crate::filereading;

pub fn run() {
    a();
}

struct Race {
    time: i32,
    distance: i32,
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

fn get_races() -> Vec<Race> {
    let mut lines = filereading::get_lines("src/inputs/day6.txt");

    let times = extract_numbers(lines.next().unwrap().unwrap());
    let distances = extract_numbers(lines.next().unwrap().unwrap());

    times
        .into_iter()
        .enumerate()
        .map(|(i, time)| Race {
            time,
            distance: distances[i],
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
