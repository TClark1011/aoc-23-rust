use std::thread::current;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn invert_direction(direction: Direction) -> Direction {
    return match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    };
}

#[derive(Copy, Clone, PartialEq)]
enum TileCategory {
    Ground,
    Pipe(Direction, Direction),
    Start,
}

fn parse_tile_character(character: char) -> TileCategory {
    return match character {
        '|' => TileCategory::Pipe(Direction::North, Direction::South),
        '-' => TileCategory::Pipe(Direction::East, Direction::West),
        'L' => TileCategory::Pipe(Direction::North, Direction::East),
        'J' => TileCategory::Pipe(Direction::North, Direction::West),
        '7' => TileCategory::Pipe(Direction::South, Direction::West),
        'F' => TileCategory::Pipe(Direction::South, Direction::East),
        'S' => TileCategory::Start,
        '.' => TileCategory::Ground,
        _ => panic!("invalid tile character"),
    };
}

#[derive(Copy, Clone)]
struct Tile {
    category: TileCategory,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(category: TileCategory, x: usize, y: usize) -> Self {
        Tile { category, x, y }
    }

    fn is_connected_to(&self, other: Tile, direction: Direction) -> bool {
        return match (self.category, other.category) {
            (TileCategory::Pipe(self_a, self_b), TileCategory::Pipe(other_a, other_b)) => {
                let inverse_direction = invert_direction(direction);
                return (self_a == direction || self_b == direction)
                    && (other_a == inverse_direction || other_b == inverse_direction);
            }
            (TileCategory::Start, TileCategory::Pipe(_, _)) => true,
            (TileCategory::Pipe(_, _), TileCategory::Start) => true,
            _ => false,
        };
    }

    fn get_distance_from_point(&self, x: usize, y: usize) -> usize {
        let x_diff = self.x.abs_diff(x);
        let y_diff = self.y.abs_diff(y);

        return x_diff + y_diff;
    }
}

struct PipeMap {
    tile_rows: Vec<Vec<Tile>>,
}

impl PipeMap {
    fn parse_input(input: &str) -> Self {
        let tile_rows: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .map(parse_tile_character)
                    .enumerate()
                    .map(move |(column_index, category)| {
                        Tile::new(category, column_index, row_index)
                    })
                    .collect()
            })
            .collect();
        return PipeMap { tile_rows };
    }

    fn get_tile_at_coordinate(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tile_rows.get(y).map(|row| row.get(x)).unwrap_or(None)
    }

    fn get_pipes_connected_to_coordinates(&self, x: usize, y: usize) -> Vec<Tile> {
        let mut connections: Vec<Tile> = vec![];
        let this_tile = self.get_tile_at_coordinate(x, y).unwrap();

        if x > 0 {
            match self.get_tile_at_coordinate(x - 1, y) {
                Some(&left) => {
                    if this_tile.is_connected_to(left, Direction::West) {
                        connections.push(left);
                    }
                }
                _ => {}
            }
        }

        match self.get_tile_at_coordinate(x + 1, y) {
            Some(&right) => {
                if this_tile.is_connected_to(right, Direction::East) {
                    connections.push(right);
                }
            }
            _ => {}
        };

        if y > 0 {
            match self.get_tile_at_coordinate(x, y - 1) {
                Some(&above) => {
                    if this_tile.is_connected_to(above, Direction::North) {
                        connections.push(above);
                    }
                }
                _ => {}
            }
        }

        match self.get_tile_at_coordinate(x, y + 1) {
            Some(&below) => {
                if this_tile.is_connected_to(below, Direction::South) {
                    connections.push(below);
                }
            }
            _ => {}
        }

        return connections;
    }

    fn get_start_coordinates(&self) -> (usize, usize) {
        let (x, y, _) = self
            .tile_rows
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                let a = row_index.clone();
                row.iter()
                    .enumerate()
                    .map(move |(column_index, tile)| (a, column_index, tile))
            })
            .find(|(_, _, tile)| tile.category == TileCategory::Start)
            .unwrap();

        return (x, y);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = PipeMap::parse_input(input);

    let (start_x, start_y) = map.get_start_coordinates();

    let start_tile = map
        .get_tile_at_coordinate(start_x, start_y)
        .unwrap()
        .clone();

    let mut connections: Vec<Tile> = vec![start_tile];
    let mut visited: Vec<Tile> = vec![];

    let mut current_distance: usize = 0;
    loop {
        let next_connections: Vec<Tile> = connections
            .iter()
            .flat_map(|tile| map.get_pipes_connected_to_coordinates(tile.x, tile.y))
            .filter(|tile| {
                !visited
                    .iter()
                    .any(|visited_tile| (visited_tile.x == tile.x && visited_tile.y == tile.y))
            })
            .collect();

        visited.append(&mut connections);
        if next_connections.len() > 0 {
            connections = vec![next_connections[0]];
        }

        // We run until the path loops back to the start
        if connections
            .iter()
            .any(|c| (c.get_distance_from_point(start_x, start_y) <= 1))
            && current_distance > 2
        {
            break;
        }
        current_distance += 1;
    }

    return Some((current_distance / 2) + 1);
}

pub fn part_two(_input: &str) -> Option<u32> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
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
        assert_eq!(result, None);
    }

    #[test]
    fn run_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
