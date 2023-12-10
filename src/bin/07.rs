use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

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

fn parse_hand_type_from_cards(cards: &Vec<Card>) -> HandType {
    if cards.len() != 5 {
        panic!("A hand of cards have precisely 5 cards")
    }

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
        (Some(5), _, _, _, _) => HandType::FiveOfKind,
        (Some(4), _, _, _, _) => HandType::FourOfKind,
        (Some(3), Some(2), _, _, _) => HandType::FullHouse,
        (Some(3), Some(1), Some(1), _, _) => HandType::ThreeOfKind,
        (Some(2), Some(2), Some(1), _, _) => HandType::TwoPair,
        (Some(2), Some(1), Some(1), Some(1), _) => HandType::OnePair,
        (Some(1), Some(1), Some(1), Some(1), Some(1)) => HandType::HighCard,
        _ => panic!("impossible result (card card counts)"),
    }

    // return match number_of_unique_cards {
    //     1 => HandType::FiveOfKind,
    //     2 => {
    //         match (
    //             sorted_card_counts[0],
    //             sorted_card_counts[1],
    //             sorted_card_counts.get(1),
    //         ) {
    //             (4, _, _) => HandType::FourOfKind,
    //             (3, 2, _) => HandType::ThreeOfKind,
    //             (3, 1, Some(1)) => HandType::FullHouse,
    //             _ => panic!("impossible result"),
    //         }
    //     }
    //     3 => {
    //         let sorted_card_counts: Vec<(_, _)> = cards
    //             .iter()
    //             .counts()
    //             .drain()
    //             .sorted_by(|&a, &b| b.1.cmp(&a.1))
    //             .collect();

    //         match (
    //             sorted_card_counts[0],
    //             sorted_card_counts[1],
    //             sorted_card_counts[2],
    //         ) {
    //             ((&pair_one, 2), (&pair_two, 2), (_, 1)) => HandType::TwoPair,
    //             ((&set_of_3, 3), (_, 1), (_, 1)) => HandType::ThreeOfKind,
    //             _ => panic!("impossible result (more unique cards than previously determined)"),
    //         }
    //     }
    //     4 => {
    //         let sorted_card_counts: Vec<(_, _)> = cards.iter().counts().drain().collect();

    //         HandType::OnePair
    //     }
    //     5 => HandType::HighCard,
    //     _ => panic!("impossible result (more than 5 unique cards)"),
    // };
}

enum HandComparisonResult {
    FirstWins,
    SecondWins,
}

fn compare_hands(hand_1: &Vec<Card>, hand_2: &Vec<Card>) -> Ordering {
    if hand_1.len() != 5 || hand_2.len() != 5 {
        panic!("A hand must consist of 5 cards")
    }

    let hand_type_1 = parse_hand_type_from_cards(hand_1);
    let hand_type_2 = parse_hand_type_from_cards(hand_2);

    if hand_type_1 != hand_type_2 {
        return Ord::cmp(&(hand_type_1 as isize), &(hand_type_2 as isize));
    }

    for card_index in 0..5 {
        let card_1 = hand_1[card_index].clone();
        let card_2 = hand_2[card_index].clone();

        if card_1 != card_2 {
            return Ord::cmp(&(card_1 as isize), &(card_2 as isize));
        }
    }

    panic!("Unable to determine definitive ranking")
}

fn parse_line(line: &str) -> (Vec<Card>, u32) {
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
    let data: Vec<(_, _)> = input.lines().map(|line| parse_line(line)).collect();
    let sorted_data: Vec<&(_, _)> = data
        .iter()
        .sorted_by(|a, b| compare_hands(&a.0, &b.0))
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
    None
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
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
