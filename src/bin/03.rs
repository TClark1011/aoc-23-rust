use regex::Regex;

advent_of_code::solution!(3);

fn number_digits(n: u32) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn diff_is_within(a: i32, b: i32, diff_limit: i32) -> bool {
    (a.abs() - b.abs()).abs() <= diff_limit
}

#[derive(PartialEq, Clone)]
enum SchematicContent {
    Number(u32),
    Symbol(String),
    Gear,
}

struct SchematicPiece {
    content: SchematicContent,
    row_index: usize,
    first_character_column_index: usize,
}

impl SchematicPiece {
    fn content_length(&self) -> usize {
        match self.content {
            SchematicContent::Number(num) => usize::try_from(number_digits(num)).unwrap(),
            _ => 1,
        }
    }

    fn get_final_character_column_index(&self) -> usize {
        return self.first_character_column_index
            + usize::try_from(self.content_length() - 1).unwrap();
    }

    fn is_adjacent_to(&self, other_piece: &SchematicPiece) -> bool {
        let row_index_delta =
            i32::try_from(other_piece.row_index).unwrap() - i32::try_from(self.row_index).unwrap();

        if row_index_delta.abs() > 1 {
            return false;
        }

        let index_to_left_of_start = if self.first_character_column_index > 0 {
            self.first_character_column_index - 1
        } else {
            self.first_character_column_index
        };

        let left_is_good = other_piece.get_final_character_column_index() >= index_to_left_of_start;
        let right_is_good =
            other_piece.first_character_column_index <= self.get_final_character_column_index() + 1;

        left_is_good && right_is_good
    }
}

struct Schematic {
    pieces: Vec<SchematicPiece>,
}

impl Schematic {
    fn from_lines(lines: Vec<&str>) -> Self {
        let schematic_pieces: Vec<SchematicPiece> = lines
            .iter()
            .map(|line| {
                Regex::new("([0-9]+)|([^.])")
                    .unwrap()
                    .find_iter(line)
                    .map(|a_match| (a_match.start(), a_match.as_str()))
                    .collect::<Vec<(usize, &str)>>()
            })
            .enumerate()
            .map(|(row_index, line_segments)| {
                line_segments
                    .iter()
                    .map(|(first_character_column_index, segment)| {
                        let piece_type = match (segment, segment.parse::<u32>()) {
                            (&"*", _) => SchematicContent::Gear,
                            (_, Ok(num)) => SchematicContent::Number(num),
                            (_, Err(_)) => SchematicContent::Symbol(segment.to_string()),
                        };

                        SchematicPiece {
                            content: piece_type,
                            row_index: row_index,
                            first_character_column_index: first_character_column_index.clone(),
                        }
                    })
                    .collect()
            })
            .flat_map(|a: Vec<SchematicPiece>| a)
            .collect();

        Schematic {
            pieces: schematic_pieces,
        }
    }

    fn get_valid_number_pieces(&self) -> Vec<&SchematicPiece> {
        let (number_pieces, symbol_pieces): (Vec<_>, Vec<_>) =
            self.pieces.iter().partition(|piece| match piece.content {
                SchematicContent::Number(_) => true,
                _ => false,
            });

        number_pieces
            .iter()
            .filter(|number_piece| {
                symbol_pieces
                    .iter()
                    .filter(|symbol_piece| {
                        diff_is_within(
                            i32::try_from(number_piece.row_index).unwrap(),
                            i32::try_from(symbol_piece.row_index).unwrap(),
                            1,
                        )
                    })
                    .any(|symbol_piece| symbol_piece.is_adjacent_to(number_piece))
            })
            .map(|&a| a)
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_lines(input.lines().collect());

    let number_part_pieces = schematic.get_valid_number_pieces();

    let sum_of_part_numbers = number_part_pieces.iter().fold(0, |sum, piece| {
        sum + match piece.content {
            SchematicContent::Number(num) => num,
            _ => 0,
        }
    });

    Some(sum_of_part_numbers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = Schematic::from_lines(input.lines().collect());

    let number_part_pieces = schematic.get_valid_number_pieces();

    let gear_ratios: Vec<u32> = schematic
        .pieces
        .iter()
        .filter(|piece| piece.content == SchematicContent::Gear)
        .filter_map(|gear_piece| {
            let adjacent_number_parts = number_part_pieces
                .iter()
                .filter(|number_part_piece| number_part_piece.is_adjacent_to(gear_piece))
                .collect::<Vec<&&SchematicPiece>>();

            if adjacent_number_parts.len() == 2 {
                return Some(adjacent_number_parts.iter().fold(0, |sum, part| {
                    match (sum, part.content.clone()) {
                        (0, SchematicContent::Number(num)) => num,
                        (non_zero_sum, SchematicContent::Number(num)) => non_zero_sum * num,
                        _ => 0,
                    }
                }));
            }

            None
        })
        .collect();

    let gear_ratios_sum = gear_ratios
        .iter()
        .fold(0, |sum, gear_ratio| sum + gear_ratio);

    // let sum_of_part_numbers = number_part_pieces.iter().fold(0, |sum, piece| {
    //     sum + match piece.content {
    //         SchematicContent::Number(num) => num,
    //         _ => 0,
    //     }
    // });

    Some(gear_ratios_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
