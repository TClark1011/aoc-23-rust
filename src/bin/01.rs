advent_of_code::solution!(1);

static DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn get_first_digit_in_string(input: &str) -> Option<u32> {
    let mut result: Option<u32> = None;
    let mut index_of_result: Option<usize> = None;
    for (index, character) in input.chars().enumerate() {
        let parsed_digit = character.to_string().parse::<u32>();
        match parsed_digit {
            Ok(digit) => {
                result = Some(digit);
                index_of_result = Some(index);
            }
            _ => {}
        };
        if result != None {
            break;
        }
    }

    for (digit, digit_word) in DIGIT_WORDS.iter().enumerate() {
        match (input.find(digit_word), index_of_result) {
            (Some(index_of_digit_word_substring), Some(result_index)) => {
                if index_of_digit_word_substring < result_index {
                    index_of_result = Some(index_of_digit_word_substring);
                    result = Some(digit as u32);
                }
            }
            (Some(index_of_digit_word), None) => {
                index_of_result = Some(index_of_digit_word);
                result = Some(digit as u32);
            }
            _ => {}
        };
    }

    result
}
pub fn get_last_digit_in_string(input: &str) -> Option<u32> {
    let mut result: Option<u32> = None;
    let mut index_of_result: Option<usize> = None;
    for (character_index, character) in input.chars().enumerate() {
        let parsed_digit = character.to_string().parse::<u32>();
        match parsed_digit {
            Ok(digit) => {
                result = Some(digit);
                index_of_result = Some(character_index);
            }
            _ => {}
        };
    }

    for (digit, digit_word) in DIGIT_WORDS.iter().enumerate() {
        match (input.rfind(digit_word), index_of_result) {
            (Some(index_of_digit_word_substring), Some(result_index)) => {
                if index_of_digit_word_substring > result_index {
                    index_of_result = Some(index_of_digit_word_substring);
                    result = Some(digit as u32);
                }
            }
            (Some(index_of_digit_word), None) => {
                index_of_result = Some(index_of_digit_word);
                result = Some(digit as u32);
            }
            _ => {}
        };
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();

    let numbers_on_each_line: Vec<(u32, u32)> = lines
        .iter()
        .map(|&line| {
            let first_digit = get_first_digit_in_string(line).unwrap();

            let reversed_line_string = line.chars().rev().collect::<String>();
            let last_digit = get_first_digit_in_string(reversed_line_string.as_str()).unwrap();
            (first_digit, last_digit)
        })
        .collect();

    let sum: u32 = numbers_on_each_line
        .iter()
        .fold(0, |sum, (first_digit, last_digit)| {
            (first_digit * 10) + last_digit + sum
        });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();

    let numbers_on_each_line: Vec<(u32, u32)> = lines
        .iter()
        .map(|&line| {
            let first_digit = get_first_digit_in_string(line).unwrap();
            let last_digit = get_last_digit_in_string(line).unwrap();

            (first_digit, last_digit)
        })
        .collect();

    let sum: u32 = numbers_on_each_line
        .iter()
        .fold(0, |sum, (first_digit, last_digit)| {
            (first_digit * 10) + last_digit + sum
        });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
