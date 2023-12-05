use regex::Regex;

advent_of_code::solution!(4);

fn parse_scratch_card_numbers(raw_numbers: &str) -> Vec<u32> {
    raw_numbers
        .split(" ")
        .map(|number_text| number_text.trim())
        .filter_map(|number_text| number_text.parse::<u32>().ok())
        .collect()
}

struct ScratchCard {
    number: usize,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl ScratchCard {
    fn from_line(line: &str) -> Option<Self> {
        let (card_label, game_text) = line.split_once(":").unwrap();
        let (winning_numbers_text, your_numbers_text) = game_text.split_once("|").unwrap();

        let card_number_regex = Regex::new("^Card\\s+([0-9]+)").expect("Invalid Regex");
        let card_number_match_opt = card_number_regex.captures(card_label);

        match card_number_match_opt {
            Some(matches) => Some(ScratchCard {
                number: matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                winning_numbers: parse_scratch_card_numbers(winning_numbers_text),
                your_numbers: parse_scratch_card_numbers(your_numbers_text),
            }),
            _ => None,
        }

        // return ScratchCard {
        //     number: game_number,
        //     winning_numbers: parse_scratch_card_numbers(winning_numbers_text),
        //     your_numbers: parse_scratch_card_numbers(your_numbers_text),
        // };
    }

    fn derive_your_amount_of_winning_numbers(&self) -> usize {
        self.your_numbers
            .iter()
            .filter(|&&your_number| self.winning_numbers.contains(&your_number))
            .collect::<Vec<&u32>>()
            .len()
    }

    fn derive_score(&self) -> u32 {
        let amount_of_winning_numbers = self.derive_your_amount_of_winning_numbers();

        if amount_of_winning_numbers == 0 {
            return 0;
        }

        let base: u32 = 2;
        let power = u32::try_from(amount_of_winning_numbers).unwrap() - 1;

        let points: u32 = base.pow(power);

        points
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let scratch_cards: Vec<ScratchCard> = lines
        .iter()
        .filter_map(|line| ScratchCard::from_line(line))
        .collect();

    let total_score = scratch_cards
        .iter()
        .fold(0, |sum, card| sum + card.derive_score());

    Some(total_score)
}

struct Part2Game {
    card_number_copies: Vec<usize>,
    cards: Vec<ScratchCard>,
}

impl Part2Game {
    fn from_scratch_cards(cards: Vec<ScratchCard>) -> Self {
        Part2Game {
            card_number_copies: vec![],
            cards: cards,
        }
    }

    fn run_game(&mut self) -> usize {
        self.cards.iter().for_each(|scratch_card| {
            let times_must_run = 1 + self
                .card_number_copies
                .iter()
                .filter(|&&copy_card_number| scratch_card.number == copy_card_number)
                .collect::<Vec<&usize>>()
                .len();

            let amount_of_winning_numbers = scratch_card.derive_your_amount_of_winning_numbers();
            let number_of_cards = self.cards.len().clone();

            for _ in 0..times_must_run {
                for n in 0..amount_of_winning_numbers {
                    let card_number_to_copy = n + scratch_card.number + 1;
                    if card_number_to_copy <= number_of_cards {
                        self.card_number_copies.push(card_number_to_copy);
                    }
                }
            }
        });

        self.card_number_copies.len() + self.cards.len()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let scratch_cards: Vec<ScratchCard> = lines
        .iter()
        .filter_map(|line| ScratchCard::from_line(line))
        .collect();

    let mut game = Part2Game::from_scratch_cards(scratch_cards);

    let total_number_of_cards = game.run_game();

    Some(u32::try_from(total_number_of_cards).unwrap())
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
        assert_eq!(result, Some(30));
    }
}
