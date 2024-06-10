use std::{i64::MAX, str::FromStr};

use crate::filereading;

#[derive(Clone)]
struct Map {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}
#[derive(Debug)]
struct MapError;

impl FromStr for Map {
    type Err = MapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Ok(Map {
            destination_start: numbers[0],
            source_start: numbers[1],
            range_length: numbers[2],
        })
    }
}

impl Map {
    fn get(&self, val: i64) -> Option<i64> {
        if val < self.source_start || val >= self.source_start + self.range_length {
            return None;
        }

        let offset = val - self.source_start;

        Some(self.destination_start + offset)
    }
}

fn get_map_groups() -> (Vec<i64>, Vec<Vec<Map>>) {
    let mut lines = filereading::get_lines("src/inputs/day5.txt");

    let first_line = lines.next().unwrap().unwrap();

    let seed_strings = first_line.split(':').collect::<Vec<&str>>()[1];
    let seeds = seed_strings
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    lines.next();

    let mut map_groups: Vec<Vec<Map>> = vec![];
    let mut current_map: Vec<Map> = vec![];
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            map_groups.push(current_map.clone());
            current_map.clear();
            continue;
        }
        if line.contains(':') {
            continue;
        }
        current_map.push(Map::from_str(&line).unwrap());
    }

    map_groups.push(current_map);

    (seeds, map_groups)
}

fn a() {
    let (seeds, map_groups) = get_map_groups();

    let mut current_group = seeds;
    let mut next_group: Vec<i64> = vec![];
    for map_group in map_groups {
        for &value in current_group.iter() {
            let mut matched_res: Option<i64> = None;
            for map in map_group.iter() {
                match map.get(value) {
                    None => {}
                    Some(res) => matched_res = Some(res),
                };
            }

            match matched_res {
                None => next_group.push(value),
                Some(res) => next_group.push(res),
            }
        }

        current_group.clone_from(&next_group);
        next_group.clear();
    }

    let mut min = MAX;

    for val in current_group {
        if val < min {
            min = val;
        }
    }
    println!("{min}")
}

pub fn run() {
    a();
}
