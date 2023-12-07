use regex::Regex;

#[path = "../utils.rs"]
mod utils;

#[derive(Copy, Clone)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    fn get_end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn subtract(&self, other_range: SeedRange) -> Option<SeedRange> {
        let self_end = self.get_end();
        let other_range_end = other_range.get_end();

        let there_is_no_overlap = self_end < other_range.start || self.start > other_range_end;

        if there_is_no_overlap {
            return Some(self.clone());
        }

        let overlap_start = self.start.max(other_range.start);
        let overlap_end = self_end.min(other_range_end);
        let overlap_length = overlap_end - overlap_start + 1;

        let self_is_entirely_within_other = self.length == overlap_length;
        if self_is_entirely_within_other {
            return None;
        }

        let self_starts_first = self.start < other_range.start;

        let new_range_start = if self_starts_first {
            self.start
        } else {
            other_range_end + 1
        };
        let new_range_end = if self_starts_first {
            other_range.start - 1
        } else {
            self_end
        };

        let new_range_length = new_range_end - new_range_start + 1;

        return Some(SeedRange {
            start: new_range_start,
            length: new_range_length,
        });
    }
}

advent_of_code::solution!(5);

struct SeedMap {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

enum MapProcessSeedRangeOutput {
    NoOverlap,
    CompleteOverlap(SeedRange),
    // First range is the shifted range, second
    // is the unaffected remainder
    PartialOverlap(SeedRange, SeedRange),
}

impl SeedMap {
    fn apply_delta(&self, subject: u64) -> u64 {
        let abs_delta = self.destination_start.abs_diff(self.source_start);

        if self.source_start < self.destination_start {
            return subject + abs_delta;
        }

        return subject - abs_delta;
    }

    fn get_source_end(&self) -> u64 {
        self.source_start + self.range_length - 1
    }

    fn from_line(line: &str) -> Option<Self> {
        let resource_map_line_pattern =
            Regex::new("^([0-9]+)\\s+([0-9]+)\\s+([0-9]+)").expect("Invalid Regex");

        let captures_opt = resource_map_line_pattern.captures(line);

        match captures_opt {
            Some(captures) => Some(SeedMap {
                destination_start: captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                source_start: captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                range_length: captures.get(3).unwrap().as_str().parse::<u64>().unwrap(),
            }),
            _ => None,
        }
    }

    fn number_is_within_range(&self, target: u64) -> bool {
        match self.source_start.checked_add(self.range_length - 1) {
            Some(end_of_mapping) => target >= self.source_start && target <= end_of_mapping,
            None => false,
        }
    }

    fn get_destination_of_seed_number(&self, target: u64) -> u64 {
        let is_within_mapping = self.number_is_within_range(target);

        if !is_within_mapping {
            return target;
        }

        let mapped_destination = self.destination_start + (target - self.source_start);
        return mapped_destination;
    }

    fn process_seed_range(&self, range: SeedRange) -> MapProcessSeedRangeOutput {
        let self_source_end = self.get_source_end();

        let no_overlap_exists =
            self_source_end < range.start || self.source_start > range.get_end();

        if no_overlap_exists {
            return MapProcessSeedRangeOutput::NoOverlap;
        }

        let overlap_starts_at = self.source_start.max(range.start);
        let overlap_ends_at = self_source_end.min(range.get_end());
        let overlap_length = overlap_ends_at - overlap_starts_at + 1;
        let overlap_source_range = SeedRange {
            start: overlap_starts_at,
            length: overlap_length,
        };
        let overlap_destination_range = SeedRange {
            start: self.apply_delta(overlap_starts_at),
            length: overlap_length,
        };

        let remainder_opt = range.subtract(overlap_source_range);

        match remainder_opt {
            Some(remainder) => {
                MapProcessSeedRangeOutput::PartialOverlap(overlap_destination_range, remainder)
            }
            _ => MapProcessSeedRangeOutput::CompleteOverlap(overlap_destination_range),
        }
    }
}

struct SeedMapSet {
    maps: Vec<SeedMap>,
}

impl SeedMapSet {
    fn new(maps: Vec<SeedMap>) -> Self {
        SeedMapSet { maps }
    }

    fn get_map_for_seed_number(&self, seed_number: u64) -> Option<&SeedMap> {
        self.maps
            .iter()
            .rfind(|map| map.number_is_within_range(seed_number))
    }

    fn get_number_next_destination(&self, seed_number: u64) -> u64 {
        match self.get_map_for_seed_number(seed_number) {
            Some(map) => map.get_destination_of_seed_number(seed_number),
            _ => seed_number,
        }
    }

    fn pass_seed_range_through_maps(&self, range: SeedRange) -> Vec<SeedRange> {
        let mut new_ranges: Vec<SeedRange> = vec![];
        let mut current_range_opt: Option<SeedRange> = Some(range);

        for map in self.maps.iter() {
            if current_range_opt.is_none() {
                break;
            }
            let current_range = current_range_opt.unwrap();

            let process_output = map.process_seed_range(current_range);

            match process_output {
                MapProcessSeedRangeOutput::CompleteOverlap(new_range) => {
                    new_ranges.push(new_range);
                    current_range_opt = None;
                }
                MapProcessSeedRangeOutput::PartialOverlap(shifted, remainder) => {
                    new_ranges.push(shifted);
                    current_range_opt = Some(remainder);
                }
                MapProcessSeedRangeOutput::NoOverlap => {}
            }
        }

        match current_range_opt {
            Some(range) => new_ranges.push(range),
            _ => {}
        }

        return new_ranges;
    }
}

fn parse_seed_number_line(line: &str) -> Option<Vec<u64>> {
    let seed_number_line_pattern = Regex::new("^seeds: (.*)").expect("invalid regex");

    match seed_number_line_pattern.captures(line) {
        Some(captures) => captures.get(1).map(|the_match| {
            the_match
                .as_str()
                .split(" ")
                .map(|num_text| num_text.trim())
                .map(|num_text| num_text.parse::<u64>().expect("invalid seed number"))
                .collect()
        }),
        _ => None,
    }
}

struct Almanac {
    map_sets: Vec<SeedMapSet>,
}

impl Almanac {
    fn new(resource_map_sets: Vec<SeedMapSet>) -> Self {
        Almanac {
            map_sets: resource_map_sets,
        }
    }

    fn get_location_for_seed_number(&self, seed_number: u64) -> u64 {
        let location = &self
            .map_sets
            .iter()
            .fold(seed_number, |current_destination, map_set| {
                map_set.get_number_next_destination(current_destination)
            });

        return location.clone();
    }

    fn get_destination_ranges_for_seed_range(
        &self,
        seed_range: SeedRange,
    ) -> (Vec<SeedRange>, SeedRange) {
        (
            self.map_sets
                .iter()
                .fold(vec![seed_range], |ranges, map_set| {
                    ranges
                        .iter()
                        .flat_map(|&range| map_set.pass_seed_range_through_maps(range))
                        .collect()
                }),
            seed_range,
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let label_line_pattern = Regex::new("^.*:$").expect("invalid_regex");

    let lines: Vec<&str> = input.lines().collect();

    let seed_numbers: Vec<u64> = parse_seed_number_line(lines[0])
        .expect("Failed to read seed numbers from first input line");

    let grouped_lines = utils::split_vector(lines, |line| label_line_pattern.is_match(line));
    let resource_map_sets: Vec<SeedMapSet> = grouped_lines
        .iter()
        .map(|line_group| {
            line_group
                .iter()
                .filter_map(|line| SeedMap::from_line(line))
                .collect::<Vec<SeedMap>>()
        })
        .filter(|resource_map_group| resource_map_group.len() > 0)
        .map(SeedMapSet::new)
        .collect();

    let almanac = Almanac::new(resource_map_sets);

    let results: Vec<u64> = seed_numbers
        .iter()
        .map(|&seed_number| almanac.get_location_for_seed_number(seed_number))
        .collect();

    let lowest_result = results
        .iter()
        .fold(u64::MAX, |min_result, &current_result| {
            min_result.min(current_result)
        });

    Some(lowest_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let label_line_pattern = Regex::new("^.*:$").expect("invalid_regex");

    let lines: Vec<&str> = input.lines().collect();

    let base_seed_numbers: Vec<u64> = parse_seed_number_line(lines[0])
        .expect("Failed to read seed numbers from first input line");

    let seed_ranges: Vec<SeedRange> = base_seed_numbers
        .chunks(2)
        .map(|seed_range| SeedRange {
            start: seed_range[0],
            length: seed_range[1],
        })
        .collect();

    let grouped_lines = utils::split_vector(lines, |line| label_line_pattern.is_match(line));
    let seed_map_sets: Vec<SeedMapSet> = grouped_lines
        .iter()
        .map(|line_group| {
            line_group
                .iter()
                .filter_map(|line| SeedMap::from_line(line))
                .collect::<Vec<SeedMap>>()
        })
        .filter(|resource_map_group| resource_map_group.len() > 0)
        .map(SeedMapSet::new)
        .collect();

    let almanac = Almanac::new(seed_map_sets);

    let final_ranges_groups: Vec<_> = seed_ranges
        .iter()
        .map(|&range| almanac.get_destination_ranges_for_seed_range(range))
        .collect();

    let final_ranges: Vec<SeedRange> = final_ranges_groups
        .into_iter()
        .flat_map(|(range, _)| range)
        .collect();

    let smallest_range_start: u64 = final_ranges
        .iter()
        .fold(u64::MAX, |min, current| min.min(current.start));

    Some(smallest_range_start)
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
