advent_of_code::solution!(4);

fn parse_scratch_card_numbers(raw_numbers: &str) -> Vec<u32> {
    raw_numbers
        .split(" ")
        .map(|number_text| number_text.trim())
        .filter_map(|number_text| number_text.parse::<u32>().ok())
        .collect()
}

struct ScratchCard {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl ScratchCard {
    fn from_line(line: &str) -> Self {
        let (_, game_text) = line.split_once(":").unwrap();
        let (winning_numbers_text, your_numbers_text) = game_text.split_once("|").unwrap();

        return ScratchCard {
            winning_numbers: parse_scratch_card_numbers(winning_numbers_text),
            your_numbers: parse_scratch_card_numbers(your_numbers_text),
        };
    }

    fn derive_score(&self) -> u32 {
        self.your_numbers.iter().fold(0, |sum, current_number| {
            match (sum, self.winning_numbers.contains(current_number)) {
                (0, true) => 1,
                (non_zero_sum, true) => non_zero_sum * 2,
                _ => sum,
            }
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let scratch_cards: Vec<ScratchCard> = lines
        .iter()
        .map(|line| ScratchCard::from_line(line))
        .collect();

    let total_score = scratch_cards
        .iter()
        .fold(0, |sum, card| sum + card.derive_score());

    Some(total_score)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
