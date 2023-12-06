use std::collections::HashMap;

use regex::Regex;

#[path = "../utils.rs"]
mod utils;

advent_of_code::solution!(5);

struct ResourceMap {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

impl ResourceMap {
    fn from_line(line: &str) -> Option<Self> {
        let resource_map_line_pattern =
            Regex::new("^([0-9]+)\\s+([0-9]+)\\s+([0-9]+)").expect("Invalid Regex");

        let captures_opt = resource_map_line_pattern.captures(line);

        match captures_opt {
            Some(captures) => Some(ResourceMap {
                destination_range_start: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                source_range_start: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                range_length: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            }),
            _ => None,
        }
    }

    fn number_is_within_range(&self, target: u32) -> bool {
        match self.source_range_start.checked_add(self.range_length - 1) {
            Some(end_of_mapping) => target >= self.source_range_start && target <= end_of_mapping,
            None => false,
        }
    }

    fn get_destination(&self, target: u32) -> u32 {
        let is_within_mapping = self.number_is_within_range(target);

        if !is_within_mapping {
            return target;
        }

        let mapped_destination = self.destination_range_start + (target - self.source_range_start);
        return mapped_destination;
    }
}

fn get_destination_from_multiple_resource_maps(
    resource_maps: &Vec<ResourceMap>,
    original_destination: u32,
) -> u32 {
    let mut destination_history = vec![original_destination];
    let mut final_destination = original_destination;

    resource_maps.iter().for_each(|resource_map| {
        let current_destination = resource_map.get_destination(original_destination);
        destination_history.push(current_destination);

        if current_destination != final_destination && current_destination != original_destination {
            final_destination = current_destination
        }
    });

    return final_destination;
}

fn parse_seed_number_line(line: &str) -> Option<Vec<u32>> {
    let seed_number_line_pattern = Regex::new("^seeds: (.*)").expect("invalid regex");

    match seed_number_line_pattern.captures(line) {
        Some(captures) => captures.get(1).map(|the_match| {
            the_match
                .as_str()
                .split(" ")
                .map(|num_text| num_text.trim())
                .map(|num_text| num_text.parse::<u32>().expect("invalid seed number"))
                .collect()
        }),
        _ => None,
    }
}

struct Almanac {
    seed_numbers: Vec<u32>,
    resource_map_sets: Vec<Vec<ResourceMap>>,
    solved_seed_numbers: HashMap<u32, u32>, // for memoisation
}

impl Almanac {
    fn create(seed_numbers: Vec<u32>, resource_map_sets: Vec<Vec<ResourceMap>>) -> Self {
        Almanac {
            seed_numbers: seed_numbers,
            resource_map_sets: resource_map_sets,
            solved_seed_numbers: HashMap::new(),
        }
    }

    fn get_seed_number_outputs(&mut self) -> Vec<u32> {
        self.seed_numbers
            .iter()
            .map(
                |seed_number| match self.solved_seed_numbers.get(seed_number) {
                    Some(&already_solved) => {
                        println!("Hit seed number cache: {}", already_solved);
                        return already_solved;
                    }
                    _ => {
                        let result = self.resource_map_sets.iter().fold(
                            seed_number.clone(),
                            |result, resource_map_set| {
                                get_destination_from_multiple_resource_maps(
                                    resource_map_set,
                                    result,
                                )
                            },
                        );

                        self.solved_seed_numbers.insert(seed_number.clone(), result);

                        return result;
                    }
                },
            )
            .collect()
    }
}

struct Range {
    start: u32,
    size: u32,
}

impl Range {
    fn map<F, T>(&self, mapper: F) -> Vec<T>
    where
        F: Fn(u32) -> T,
        T: Clone,
    {
        let mut results: Vec<T> = vec![];

        for offset in 0..self.size {
            results.push(mapper(self.start + offset));
        }

        return results;
    }
}

fn parse_seed_number_range((start_number, size): (u32, u32)) -> Vec<u32> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![start_number];
    }
    let range: Vec<u32> = (start_number..(start_number + size - 1)).collect();
    return range;
}

pub fn part_one(input: &str) -> Option<u32> {
    let label_line_pattern = Regex::new("^.*:$").expect("invalid_regex");

    let lines: Vec<&str> = input.lines().collect();

    let seed_numbers: Vec<u32> = parse_seed_number_line(lines[0])
        .expect("Failed to read seed numbers from first input line");

    let grouped_lines = utils::split_vector(lines, |line| label_line_pattern.is_match(line));
    let resource_map_sets: Vec<Vec<ResourceMap>> = grouped_lines
        .iter()
        .map(|line_group| {
            line_group
                .iter()
                .filter_map(|line| ResourceMap::from_line(line))
                .collect::<Vec<ResourceMap>>()
        })
        .filter(|resource_map_group| resource_map_group.len() > 0)
        .collect();

    let mut almanac = Almanac::create(seed_numbers, resource_map_sets);

    let results = almanac.get_seed_number_outputs();

    let lowest_result = results
        .iter()
        .fold(u32::MAX, |min_result, &current_result| {
            min_result.min(current_result)
        });

    Some(lowest_result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let label_line_pattern = Regex::new("^.*:$").expect("invalid_regex");

    let lines: Vec<&str> = input.lines().collect();

    let base_seed_numbers: Vec<u32> = parse_seed_number_line(lines[0])
        .expect("Failed to read seed numbers from first input line");

    let seed_ranges: Vec<Range> = base_seed_numbers
        .chunks(2)
        .map(|seed_range| Range {
            start: seed_range[0],
            size: seed_range[1],
        })
        .collect();

    let grouped_lines = utils::split_vector(lines, |line| label_line_pattern.is_match(line));
    let resource_map_sets: Vec<Vec<ResourceMap>> = grouped_lines
        .iter()
        .map(|line_group| {
            line_group
                .iter()
                .filter_map(|line| ResourceMap::from_line(line))
                .collect::<Vec<ResourceMap>>()
        })
        .filter(|resource_map_group| resource_map_group.len() > 0)
        .collect();

    let results: Vec<u32> = seed_ranges
        .iter()
        .flat_map(|seed_range| {
            seed_range.map(|seed_number| {
                resource_map_sets
                    .iter()
                    .fold(seed_number, |destination, resource_maps| {
                        get_destination_from_multiple_resource_maps(resource_maps, destination)
                    })
            })
        })
        .collect();

    let lowest_result = results
        .iter()
        .fold(u32::MAX, |min_result, &current_result| {
            min_result.min(current_result)
        });

    Some(lowest_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two_final() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());

        if let Some(result_value) = result {
            println!("RESULT: {}", result_value);
        }
    }
}
