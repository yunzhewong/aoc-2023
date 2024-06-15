use std::{cmp::max, i64::MAX, str::FromStr};

use crate::filereading;

#[derive(Clone, Debug)]
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

    fn get_overlap(&self, range: &Range) -> Option<(Range, Vec<Range>)> {
        let range_end_index = range.start + range.length - 1;
        let map_end_index = self.source_start + self.range_length - 1;

        let overlap_start_index = max(range.start, self.source_start);
        let overlap_end_index = std::cmp::min(range_end_index, map_end_index);

        println!("{:?} {:?}", range, self);
        println!("{overlap_start_index} {overlap_end_index}");

        let overlap_length = overlap_end_index - overlap_start_index + 1;

        if overlap_length < 0 {
            return None;
        }
        let offset = self.destination_start - self.source_start;
        let changed_range = Range {
            start: overlap_start_index + offset,
            length: overlap_length,
        };

        let mut others: Vec<Range> = vec![];

        let length_missed_before = overlap_start_index - range.start;

        if length_missed_before > 0 {
            others.push(Range {
                start: range.start,
                length: length_missed_before,
            })
        }

        let length_missed_after = range_end_index - overlap_end_index;

        if length_missed_after > 0 {
            others.push(Range {
                start: overlap_end_index + 1,
                length: length_missed_after,
            })
        }

        Some((changed_range, others))
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
        for &range in current_ranges.iter() {
            let mut ranges_to_map = vec![range];

            while ranges_to_map.len() > 0 {
                let range_to_map = ranges_to_map[0];

                let mut mapped_range: Option<Range> = None;
                for map in map_group.iter() {
                    match map.get_overlap(&range_to_map) {
                        None => {}
                        Some((res_range, uncaptured)) => {
                            ranges_to_map.extend(uncaptured);
                            mapped_range = Some(res_range);
                            break;
                        }
                    }
                }

                match mapped_range {
                    None => next_ranges.push(range_to_map.clone()),
                    Some(res) => next_ranges.push(res),
                }
            }
        }

        current_ranges.clone_from(&next_ranges);
        next_ranges.clear();
    }
}

pub fn run() {
    b();
}
