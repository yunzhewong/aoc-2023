use std::{cmp::max, i64::MAX, str::FromStr};

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

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    length: i64,
}

impl Map {
    fn get(&self, val: i64) -> Option<i64> {
        if val < self.source_start || val >= self.source_start + self.range_length {
            return None;
        }

        let offset = val - self.source_start;

        Some(self.destination_start + offset)
    }

    fn get_range(&self, range: &Range) -> Option<(Range, Vec<Range>)> {
        if range.start + range.length <= self.source_start
            || self.source_start + self.range_length <= range.length
        {
            return None;
        }

        let range_end_index = range.start + range.length - 1;
        let source_end_index = self.source_start + self.range_length - 1;

        let map_start = max(range.start, self.source_start);
        let map_end = std::cmp::min(range_end_index, source_end_index);

        let map_length = map_end - map_start + 1;
        let mapped_range = Range {
            start: map_start,
            length: map_length,
        };

        let mut uncaptured: Vec<Range> = vec![];
        let uncaptured_start_length = map_start - range.start;

        if uncaptured_start_length > 0 {
            uncaptured.push(Range {
                start: range.start,
                length: uncaptured_start_length,
            })
        }

        let uncaptured_end_length = range_end_index - map_end;

        if uncaptured_end_length > 0 {
            uncaptured.push(Range {
                start: map_end,
                length: uncaptured_end_length,
            });
        }

        Some((mapped_range, uncaptured))
    }
}

fn get_map_groups() -> (Vec<i64>, Vec<Vec<Map>>) {
    let mut lines = filereading::get_lines("src/inputs/day5e.txt");

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

fn b() {
    let (seeds, map_groups) = get_map_groups();

    let mut seed_ranges: Vec<Range> = vec![];
    for i in 0..(seeds.len() / 2) {
        seed_ranges.push(Range {
            start: seeds[i],
            length: seeds[i + 1],
        })
    }

    let mut current_ranges: Vec<Range> = seed_ranges;
    let mut next_ranges: Vec<Range> = vec![];
    for map_group in map_groups {
        for range in current_ranges.iter() {
            let mut check_ranges = vec![range.clone()];
            let mut matched_res: Option<Range> = None;
            for map in map_group.iter() {
                let mut new_ranges: Vec<Range> = vec![];

                for checked_range in check_ranges {
                    match map.get_range(&checked_range) {
                        None => {}
                        Some((res_range, uncaptured)) => {
                            new_ranges.extend(uncaptured);
                            matched_res = Some(res_range);
                            break;
                        }
                    }
                }

                check_ranges = new_ranges;
            }

            match matched_res {
                None => next_ranges.push(range.clone()),
                Some(res) => next_ranges.push(res),
            }
        }

        current_ranges.clone_from(&next_ranges);
        next_ranges.clear();

        println!("{:?}", current_ranges)
    }
}

pub fn run() {
    b();
}
