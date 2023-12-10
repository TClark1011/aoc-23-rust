advent_of_code::solution!(6);

#[derive(Clone, Copy)]
struct BoatRace {
    remaining_duration_ms: u64,
    current_speed: u64, // millimeters per millisecond
    record_distance: u64,
}

impl BoatRace {
    fn new(total_duration: u64, record_distance: u64) -> Self {
        BoatRace {
            remaining_duration_ms: total_duration,
            current_speed: 0,
            record_distance,
        }
    }

    fn hold_button(&self) -> BoatRace {
        BoatRace {
            remaining_duration_ms: self.remaining_duration_ms - 1,
            current_speed: self.current_speed + 1,
            record_distance: self.record_distance,
        }
    }

    fn estimate_distance(&self) -> u64 {
        self.current_speed * self.remaining_duration_ms
    }

    fn will_beat_record_now(&self) -> bool {
        self.estimate_distance() > self.record_distance
    }

    /**
     * Returns all the different durations for which you can
     * be holding the button that would beat the current
     * record
     */
    fn get_all_win_possibilities(&self) -> Vec<u64> {
        let mut win_possibilities: Vec<u64> = Vec::default();

        let mut t = self.clone();
        for n in 0..self.remaining_duration_ms {
            t = t.hold_button();
            if t.will_beat_record_now() {
                win_possibilities.push(n);
            }
        }

        return win_possibilities;
    }
}

fn parse_pt1_data_line(line: &str) -> Vec<u64> {
    let (_, all_numbers_text) = line.split_once(":").unwrap();

    all_numbers_text
        .split(" ")
        .filter_map(|t| t.parse::<u64>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let times = parse_pt1_data_line(lines[0]);
    let distances = parse_pt1_data_line(lines[1]);

    let mut races: Vec<BoatRace> = Vec::default();
    for i in 0..times.len() {
        let duration = times[i];
        let record_distance = distances[i];
        races.push(BoatRace::new(duration, record_distance))
    }

    let win_possibility_amount_product: u64 = races
        .iter()
        .map(|race| u64::try_from(race.get_all_win_possibilities().len()).unwrap())
        .fold(1, |result, win_possibilities| win_possibilities * result);

    Some(win_possibility_amount_product)
}

fn parse_pt2_data_line(line: &str) -> u64 {
    let (_, all_numbers_text) = line.split_once(":").unwrap();

    all_numbers_text.replace(" ", "").parse::<u64>().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let time = parse_pt2_data_line(lines[0]);
    let distance = parse_pt2_data_line(lines[1]);

    let race = BoatRace::new(time, distance);
    let number_of_ways_to_win = u64::try_from(race.get_all_win_possibilities().len()).unwrap();

    Some(number_of_ways_to_win)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
