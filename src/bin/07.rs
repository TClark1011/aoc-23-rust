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

#[repr(u32)]
enum Hand {
    FiveOfKind {
        card: Card,
    } = 6,
    FourOfKind {
        four_kind_card: Card,
    } = 5,
    FullHouse {
        three_matching_card: Card,
    } = 4,
    ThreeOfKind {
        three_matching_card: Card,
    } = 3,
    TwoPair {
        pair_one_card: Card,
        pair_two_card: Card,
    } = 2,
    OnePair {
        pair_card: Card,
    } = 1,
    HighCard {
        highest_card: Card,
    } = 0,
}

fn parse_hand_type_from_cards(cards: Vec<Card>) -> Hand {
    if cards.len() != 5 {
        panic!("A hand of cards have precisely 5 cards")
    }

    let highest_card: Card = cards.iter().fold(Card::Two, |result, &current_card| {
        if current_card as isize > result as isize {
            return current_card;
        }

        return result;
    });

    let unique_cards: Vec<&Card> = cards.iter().unique().collect_vec();

    let number_of_unique_cards = unique_cards.len();

    return match number_of_unique_cards {
        1 => Hand::FiveOfKind { card: cards[0] },
        2 => {
            let (high_cards, low_cards): (Vec<Card>, Vec<Card>) =
                cards.iter().partition(|&&card| card == highest_card);

            let (most_cards, less_cards) = if high_cards.len() > low_cards.len() {
                (high_cards, low_cards)
            } else {
                (low_cards, high_cards)
            };

            match (most_cards.len(), less_cards.len()) {
                (4, _) => Hand::FourOfKind {
                    four_kind_card: most_cards[0],
                },
                (3, 2) => Hand::ThreeOfKind {
                    three_matching_card: most_cards[0],
                },
                (3, 1) => Hand::FullHouse {
                    three_matching_card: most_cards[0],
                },
                _ => panic!("impossible result"),
            }
        }
        3 => {
            let sorted_card_counts: Vec<(_, _)> = cards
                .iter()
                .counts()
                .drain()
                .sorted_by(|&a, &b| b.1.cmp(&a.1))
                .collect();

            match (
                sorted_card_counts[0],
                sorted_card_counts[1],
                sorted_card_counts[2],
            ) {
                ((&pair_one, 2), (&pair_two, 2), (_, 1)) => Hand::TwoPair {
                    pair_one_card: pair_one.clone(),
                    pair_two_card: pair_two.clone(),
                },
                ((&set_of_3, 3), (_, 1), (_, 1)) => Hand::ThreeOfKind {
                    three_matching_card: set_of_3.clone(),
                },
                _ => panic!("impossible result (more unique cards than previously determined)"),
            }
        }
        4 => {
            let sorted_card_counts: Vec<(_, _)> = cards.iter().counts().drain().collect();

            Hand::OnePair {
                pair_card: (*sorted_card_counts[0].0).clone(),
            }
        }
        5 => Hand::HighCard { highest_card },
        _ => panic!("Not finished"),
    };
}

fn derive_hand_values(hand: Hand)

fn parse_line(line: &str) -> (Hand, u32) {
    let (card_characters, bid_number_text): (&str, &str) =
        line.split_once(" ").expect("Line should match structure");

    let cards: Vec<Card> = card_characters
        .chars()
        .map(|the_char| parse_card_value(the_char))
        .collect();

    let hand = parse_hand_type_from_cards(cards);

    let bid = bid_number_text
        .parse::<u32>()
        .expect("Line should match structure");

    return (hand, bid);
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<(_, _)> = input.lines().map(|line| parse_line(line)).collect();

    None
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
    fn test_part_two() {
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
