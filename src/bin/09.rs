advent_of_code::solution!(9);

use itertools::Itertools;

fn predict_next_value(sequence: Vec<i64>) -> i64 {
    let final_index = sequence.len() - 1;
    let last_sequence_value = sequence[final_index];

    if sequence.iter().all_equal() {
        return last_sequence_value;
    }

    // let mut differences: Vec<i64> = vec![];
    // sequence
    //     .iter()
    //     .enumerate()
    //     .for_each(|(index, &current_value)| {
    //         if index != 0 && index < sequence.len() - 1 {
    //             let previous_value = sequence[index - 1];
    //             differences.push(current_value - previous_value);
    //         }
    //     });
    let differences: Vec<i64> =
        sequence
            .iter()
            .enumerate()
            .fold(vec![], |mut diffs, (index, current_item)| {
                if index != 0 {
                    let previous_item = sequence[index - 1];
                    diffs.push(current_item - previous_item);
                }
                return diffs;
            });

    return last_sequence_value + predict_next_value(differences);
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ")
        .map(|segment| segment.parse::<i64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let sequences: Vec<Vec<i64>> = input.lines().map(parse_line).collect();
    let next_values: Vec<i64> = sequences.into_iter().map(predict_next_value).collect();
    let sum: i64 = next_values.iter().sum();

    return Some(sum);
}

fn predict_previous_value(sequence: Vec<i64>) -> i64 {
    let reversed: Vec<i64> = sequence.into_iter().rev().collect();

    return predict_next_value(reversed);
}

pub fn part_two(input: &str) -> Option<i64> {
    let sequences: Vec<Vec<i64>> = input.lines().map(parse_line).collect();
    let previous_values: Vec<i64> = sequences.into_iter().map(predict_previous_value).collect();
    let sum: i64 = previous_values.iter().sum();

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn run_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn run_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
