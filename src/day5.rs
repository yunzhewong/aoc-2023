#![allow(dead_code)]
use std::{
    cmp::{max, min},
    i64::MAX,
    str::FromStr,
};

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
        let map_start_index = self.source_start;
        let map_end_index = self.source_start + self.range_length - 1;

        let range_start_index = range.start;
        let range_end_index = range.start + range.length - 1;

        println!("{map_start_index} {map_end_index} {range_start_index} {range_end_index}");

        let overlap_start_index = max(map_start_index, range_start_index);
        let overlap_end_index = min(map_end_index, range_end_index);

        if overlap_end_index < overlap_start_index {
            return None;
        }

        let range_prior_start_index = min(overlap_start_index, range_start_index);
        let range_prior_end_index = overlap_start_index - 1;

        let range_posterior_start_index = overlap_end_index + 1;
        let range_posterior_end_index = max(overlap_end_index, range_end_index);

        let offset = self.destination_start - self.source_start;
        let new_start = overlap_start_index + offset;

        println!("Mapped!: {overlap_start_index} -> {new_start}");
        let overlap_range = Range {
            start: new_start,
            length: overlap_end_index - overlap_start_index + 1,
        };

        let mut ignored = vec![];

        let prior_ignored_length = range_prior_end_index - range_prior_start_index + 1;
        let posterior_ignored_length = range_posterior_end_index - range_posterior_start_index + 1;

        if prior_ignored_length > 0 {
            ignored.push(Range {
                start: range_prior_start_index,
                length: prior_ignored_length,
            })
        }

        if posterior_ignored_length > 0 {
            ignored.push(Range {
                start: range_posterior_start_index,
                length: posterior_ignored_length,
            })
        }

        println!("{:?} {:?}", overlap_range, ignored);

        Some((overlap_range, ignored))
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

fn b() {
    let (seeds, map_groups) = get_map_groups();

    let mut seed_ranges: Vec<Range> = vec![];
    for i in 0..(seeds.len() / 2) {
        seed_ranges.push(Range {
            start: seeds[2 * i],
            length: seeds[2 * i + 1],
        })
    }

    let mut current_ranges = seed_ranges;
    let mut next_ranges: Vec<Range> = vec![];
    for map_group in map_groups {
        for range in current_ranges.iter() {
            let mut check_sections: Vec<Range> = vec![range.clone()];

            while let Some(range_to_check) = check_sections.pop() {
                let mut utilised: bool = false;
                for group in map_group.iter() {
                    if let Some((new_range, uncovered)) = group.get_overlap(&range_to_check) {
                        check_sections.extend(uncovered);
                        next_ranges.push(new_range);
                        utilised = true;
                        break;
                    }
                }

                if !utilised {
                    next_ranges.push(range_to_check)
                }
            }
        }

        // let mut total_sum = 0;
        // for range in next_ranges.iter() {
        //     total_sum += range.length;
        // }
        // println!("Length: {:?}", total_sum);
        current_ranges.clone_from(&next_ranges);
        next_ranges.clear()
    }

    let mut minimum = i64::MAX;
    for range in current_ranges {
        minimum = min(range.start, minimum)
    }
    println!("{minimum}")
}

pub fn run() {
    b();
}
