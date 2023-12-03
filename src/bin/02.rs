use regex::Regex;
use std::cmp;

advent_of_code::solution!(2);

#[derive(PartialEq, Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

enum CubeColorError {
    InvalidCubeColor,
}

fn get_cube_color_from_color_text(color_text: &str) -> Result<CubeColor, CubeColorError> {
    match color_text {
        "red" => Ok(CubeColor::Red),
        "blue" => Ok(CubeColor::Blue),
        "green" => Ok(CubeColor::Green),
        _ => Err(CubeColorError::InvalidCubeColor),
    }
}

struct GameMaxCubeRule {
    color: CubeColor,
    max_allowed: u32,
}

#[derive(Debug)]
struct CubeRevealColorCount {
    color: CubeColor,
    amount: u32,
}

#[derive(Debug)]
struct CubeGame {
    game_id: u32,
    reveals: Vec<Vec<CubeRevealColorCount>>,
}

fn cube_game_is_within_rules(game: &CubeGame, game_rules: &[GameMaxCubeRule; 3]) -> bool {
    game.reveals.iter().all(|color_reveals| {
        color_reveals.iter().all(|color_reveal| {
            match game_rules
                .iter()
                .find(|rule| rule.color == color_reveal.color)
            {
                Some(rule) => rule.max_allowed >= color_reveal.amount,
                _ => false,
            }
        })
    })
}

enum GetCubeGameMinimumColorCubeCountError {
    CouldNotFindMinimumForColor,
}

fn get_cube_game_minimum_color_cube_count(
    color: CubeColor,
    game: &CubeGame,
) -> Result<u32, GetCubeGameMinimumColorCubeCountError> {
    let minimum_required_cubes_opt: Option<u32> = game
        .reveals
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .fold(None, |current_max_cubes_opt, reveal| {
            match (current_max_cubes_opt, reveal.color == color) {
                (Some(current_max_cubes), true) => Some(cmp::max(current_max_cubes, reveal.amount)),
                (None, true) => Some(reveal.amount),
                _ => current_max_cubes_opt,
            }
        });

    match minimum_required_cubes_opt {
        Some(minimum_required_cubes) => Ok(minimum_required_cubes),
        _ => Err(GetCubeGameMinimumColorCubeCountError::CouldNotFindMinimumForColor),
    }
}

enum GetCubeGamePowerError {
    CouldNotFindAllColors,
}

fn get_cube_game_power(game: &CubeGame) -> Result<u32, GetCubeGamePowerError> {
    let red_min_cubes_res = get_cube_game_minimum_color_cube_count(CubeColor::Red, game);
    let green_min_cubes_res = get_cube_game_minimum_color_cube_count(CubeColor::Green, game);
    let blue_min_cubes_res = get_cube_game_minimum_color_cube_count(CubeColor::Blue, game);

    match (red_min_cubes_res, green_min_cubes_res, blue_min_cubes_res) {
        (Ok(red_min_cubes), Ok(green_min_cubes), Ok(blue_min_cubes)) => {
            Ok(red_min_cubes * green_min_cubes * blue_min_cubes)
        }
        _ => Err(GetCubeGamePowerError::CouldNotFindAllColors),
    }
}

fn get_line_game_id(line: &str) -> Option<u32> {
    let game_id_regex = Regex::new(r"^Game ([0-9]+)").unwrap();
    match game_id_regex.captures(line) {
        Some(captures) => captures
            .get(1)
            .map(|the_match| the_match.as_str())
            .map(|the_id| the_id.parse::<u32>().unwrap()),
        _ => None,
    }
}

fn get_reveal_data_from_text(reveal_text: &str) -> Option<CubeRevealColorCount> {
    let reveal_text_regex = Regex::new(r"^([0-9]+) (red|blue|green)").unwrap();

    match reveal_text_regex.captures(reveal_text.trim()) {
        Some(captures) => {
            let amount_capture = captures.get(1).map(|ac| ac.as_str());
            let color_text_capture = captures.get(2).map(|ctc| ctc.as_str());

            match (
                amount_capture.map(|amount_string| amount_string.parse::<u32>()),
                color_text_capture.map(|color_text| get_cube_color_from_color_text(color_text)),
            ) {
                (Some(Ok(amount)), Some(Ok(cube_color))) => Some(CubeRevealColorCount {
                    color: cube_color,
                    amount: amount,
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

fn get_line_cube_color_reveals(line: &str) -> Option<Vec<Vec<CubeRevealColorCount>>> {
    let all_reveals_text_opt = line.split(":").last();

    match all_reveals_text_opt {
        Some(all_reveals_text) => {
            let reveal_batches_text: Vec<&str> = all_reveals_text.split(";").collect();
            let individual_reveals_text: Vec<Vec<&str>> = reveal_batches_text
                .iter()
                .map(|r| r.split(",").collect())
                .collect();
            let reveal_batches: Vec<Vec<CubeRevealColorCount>> = individual_reveals_text
                .iter()
                .map(|color_reveals| {
                    color_reveals
                        .iter()
                        .filter_map(|&color_reveal_text| {
                            get_reveal_data_from_text(color_reveal_text)
                        })
                        .collect()
                })
                .collect();

            Some(reveal_batches)
        }
        _ => None,
    }
}

#[derive(Debug)]
enum CubeGameLineParseError {
    InvalidLineText,
}

fn get_cube_game_from_line(line: &str) -> Result<CubeGame, CubeGameLineParseError> {
    let game_id_opt = get_line_game_id(line);
    let game_cube_reveals_opt = get_line_cube_color_reveals(line);

    match (game_id_opt, game_cube_reveals_opt) {
        (Some(game_id), Some(game_reveals)) => Ok(CubeGame {
            game_id: game_id,
            reveals: game_reveals,
        }),
        _ => Err(CubeGameLineParseError::InvalidLineText),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game_rules = [
        GameMaxCubeRule {
            color: CubeColor::Red,
            max_allowed: 12,
        },
        GameMaxCubeRule {
            color: CubeColor::Green,
            max_allowed: 13,
        },
        GameMaxCubeRule {
            color: CubeColor::Blue,
            max_allowed: 14,
        },
    ];

    let lines = input.lines().collect::<Vec<_>>();
    let games: Vec<CubeGame> = lines
        .iter()
        .map(|line| get_cube_game_from_line(line).unwrap())
        .collect();

    let valid_games: Vec<&CubeGame> = games
        .iter()
        .filter(|&game| cube_game_is_within_rules(game, &game_rules))
        .collect();

    let valid_game_ids_sum = valid_games.iter().fold(0, |sum, game| game.game_id + sum);

    Some(valid_game_ids_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let games: Vec<CubeGame> = lines
        .iter()
        .map(|line| get_cube_game_from_line(line).unwrap())
        .collect();

    let game_powers: Vec<u32> = games
        .iter()
        .filter_map(|game| get_cube_game_power(game).ok())
        .collect();

    let game_powers_sum: u32 = game_powers.iter().fold(0, |sum, power| sum + power);

    Some(game_powers_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
