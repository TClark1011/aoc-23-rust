use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

fn assert_card_hand_len(cards_len: usize) {
    if cards_len < 4 || cards_len > 5 {
        panic!("Card hands must consist of 4 or 5 cards")
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

fn parse_card_value(character: char) -> Card {
    match character {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => panic!("Invalid camel card value"),
    }
}

fn get_card_strength(card: Card, use_joker_rule: bool) -> isize {
    match (card, use_joker_rule) {
        (Card::J, true) => 0,
        (_, _) => (card as isize) + 1,
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum HandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn parse_hand_type_from_cards(cards: Vec<Card>) -> HandType {
    let sorted_card_counts: Vec<_> = cards
        .iter()
        .counts()
        .values()
        .cloned()
        .sorted()
        .rev()
        .collect();

    match (
        sorted_card_counts.get(0),
        sorted_card_counts.get(1),
        sorted_card_counts.get(2),
        sorted_card_counts.get(3),
        sorted_card_counts.get(4),
    ) {
        (_, None, None, None, None) => HandType::FiveOfKind,
        (Some(_), Some(1), None, None, None) => HandType::FourOfKind,
        (Some(_), Some(2), None, None, None) => HandType::FullHouse,
        (Some(_), Some(1), Some(1), None, None) => HandType::ThreeOfKind,
        (Some(_), Some(2), Some(1), None, None) => HandType::TwoPair,
        (Some(_), Some(1), Some(1), Some(1), None) => HandType::OnePair,
        (Some(1), Some(1), Some(1), Some(1), Some(1)) => HandType::HighCard,
        _ => panic!("waa"),
    }
}

fn filter_out_jokers(cards: &Vec<Card>, do_it: bool) -> Vec<Card> {
    if !do_it {
        return cards.clone();
    }
    return cards
        .clone()
        .into_iter()
        .filter(|&card| card != Card::J)
        .collect();
}

fn compare_hands(hand_1: &Vec<Card>, hand_2: &Vec<Card>, use_joker_rule: bool) -> Ordering {
    assert_card_hand_len(hand_1.len());
    assert_card_hand_len(hand_2.len());

    let hand_type_1 = parse_hand_type_from_cards(filter_out_jokers(hand_1, use_joker_rule));
    let hand_type_2 = parse_hand_type_from_cards(filter_out_jokers(hand_2, use_joker_rule));

    if hand_type_1 != hand_type_2 {
        return Ord::cmp(&(hand_type_1 as isize), &(hand_type_2 as isize));
    }

    for card_index in 0..5 {
        let card_1 = hand_1[card_index].clone();
        let card_2 = hand_2[card_index].clone();

        if card_1 != card_2 {
            return Ord::cmp(
                &get_card_strength(card_1, use_joker_rule),
                &get_card_strength(card_2, use_joker_rule),
            );
        }
    }

    panic!("Unable to determine definitive ranking")
}

fn parse_part_one_line(line: &str) -> (Vec<Card>, u32) {
    let (card_characters, bid_number_text): (&str, &str) =
        line.split_once(" ").expect("Line should match structure");

    let cards: Vec<Card> = card_characters
        .chars()
        .map(|the_char| parse_card_value(the_char))
        .collect();

    let bid = bid_number_text
        .parse::<u32>()
        .expect("Line should match structure");

    return (cards, bid);
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<(_, _)> = input
        .lines()
        .map(|line| parse_part_one_line(line))
        .collect();
    let sorted_data: Vec<&(_, _)> = data
        .iter()
        .sorted_by(|a, b| compare_hands(&a.0, &b.0, false))
        .collect();

    let total_winnings: u32 = sorted_data
        .iter()
        .enumerate()
        .map(|(rank_index, (_, bid))| {
            u32::try_from(rank_index + 1)
                .unwrap()
                .checked_mul(bid.clone())
                .unwrap()
        })
        .sum();

    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<(_, _)> = input
        .lines()
        .map(|line| parse_part_one_line(line))
        .collect();
    let sorted_data: Vec<&(_, _)> = data
        .iter()
        .sorted_by(|a, b| compare_hands(&a.0, &b.0, true))
        .collect();

    let total_winnings: u32 = sorted_data
        .iter()
        .enumerate()
        .map(|(rank_index, (_, bid))| {
            u32::try_from(rank_index + 1)
                .unwrap()
                .checked_mul(bid.clone())
                .unwrap()
        })
        .sum();

    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        assert_eq!(result, Some(250254244));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
