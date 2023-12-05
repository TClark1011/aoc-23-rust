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

    fn get_destination(&self, destination: u32) -> u32 {
        let end_of_mapping_opt = self.source_range_start.checked_add(self.range_length - 1);
        match end_of_mapping_opt {
            Some(end_of_mapping) => {
                let is_within_mapping =
                    destination >= self.source_range_start && destination <= end_of_mapping;

                if !is_within_mapping {
                    return destination;
                }

                let mapped_destination =
                    self.destination_range_start + (destination - self.source_range_start);
                return mapped_destination;
            }
            _ => destination,
        }
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
}

impl Almanac {
    fn get_seed_number_outputs(&self) -> Vec<u32> {
        self.seed_numbers
            .iter()
            .map(|seed_number| {
                self.resource_map_sets.iter().fold(
                    seed_number.clone(),
                    |result, resource_map_set| {
                        get_destination_from_multiple_resource_maps(resource_map_set, result)
                    },
                )
            })
            .collect()
    }
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

    let almanac = Almanac {
        seed_numbers: seed_numbers,
        resource_map_sets: resource_map_sets,
    };

    let results = almanac.get_seed_number_outputs();

    let lowest_result = results
        .iter()
        .fold(u32::MAX, |min_result, &current_result| {
            min_result.min(current_result)
        });

    Some(lowest_result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
