advent_of_code::solution!(11);

fn pairs<T: Copy>(base: Vec<T>) -> Vec<(T, T)>
where {
    let mut result: Vec<(T, T)> = vec![];

    for i in 0..base.len() {
        for j in i + 1..base.len() {
            result.push((base[i], base[j]));
        }
    }

    return result;
}

#[derive(PartialEq, Copy, Clone)]
enum TileCategory {
    Empty,
    Galaxy,
}

fn parse_tile_character(character: char) -> TileCategory {
    match character {
        '.' => TileCategory::Empty,
        '#' => TileCategory::Galaxy,
        _ => panic!("bad tile character"),
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
}

fn parse_input_to_galaxy_points(input: &str, empty_areas_count_as: u64) -> Vec<Point> {
    let base_tile_category_grid: Vec<Vec<TileCategory>> = input
        .lines()
        .map(|line| line.chars().map(parse_tile_character).collect())
        .collect();

    let num_rows = base_tile_category_grid.len();
    let num_cols = base_tile_category_grid[0].len();

    let mut empty_cols: Vec<usize> = vec![];
    for col in 0..num_cols {
        let mut col_empty = true;
        for row in 0..num_rows {
            let item = &base_tile_category_grid[row][col];
            if item == &TileCategory::Galaxy {
                col_empty = false;
                break;
            }
        }
        if col_empty {
            empty_cols.push(col);
        }
    }

    let empty_rows: Vec<usize> = base_tile_category_grid
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            return if row.contains(&TileCategory::Galaxy) {
                None
            } else {
                Some(row_index)
            };
        })
        .collect();

    let mut double_enumerated_grid: Vec<(usize, usize, TileCategory)> = vec![];
    for (row_index, row) in base_tile_category_grid.iter().enumerate() {
        for (col_index, &tile_category) in row.iter().enumerate() {
            double_enumerated_grid.push((col_index, row_index, tile_category));
        }
    }

    // Instead of ever inserting the extra columns and rows into the grid, instead
    // we just offset each point by the number of extra rows/columns they would be
    // affected by
    let points: Vec<Point> = double_enumerated_grid
        .into_iter()
        .filter(|(_, _, category)| category == &TileCategory::Galaxy)
        .map(|(x, y, _)| {
            let after_num_empty_rows = empty_rows
                .clone()
                .into_iter()
                .filter(|&row_index| row_index < y)
                .collect::<Vec<_>>()
                .len();
            let after_num_empty_cols = empty_cols
                .clone()
                .into_iter()
                .filter(|&col_index| col_index < x)
                .collect::<Vec<_>>()
                .len();
            let y_adjustment =
                u64::try_from(after_num_empty_rows).unwrap() * (empty_areas_count_as - 1);
            let x_adjustment =
                u64::try_from(after_num_empty_cols).unwrap() * (empty_areas_count_as - 1);

            let result = Point {
                x: u64::try_from(x).unwrap() + x_adjustment,
                y: u64::try_from(y).unwrap() + y_adjustment,
            };

            println!();

            return result;
        })
        .collect();

    return points;
}

fn get_distance_between_tiles(tile_a: Point, tile_b: Point) -> u64 {
    let x_diff = tile_a.x.abs_diff(tile_b.x);
    let y_diff = tile_a.y.abs_diff(tile_b.y);

    return x_diff + y_diff;
}

pub fn part_one(input: &str) -> Option<u64> {
    let galaxies = parse_input_to_galaxy_points(input, 2);
    let galaxy_pairs = pairs(galaxies);
    let distances: Vec<u64> = galaxy_pairs
        .into_iter()
        .map(|(a, b)| get_distance_between_tiles(a, b))
        .collect();

    let sum = distances.into_iter().fold(0 as u64, |total, current| {
        total + u64::try_from(current).unwrap()
    });

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxies = parse_input_to_galaxy_points(input, 1000000);
    let galaxy_pairs = pairs(galaxies);
    let distances: Vec<u64> = galaxy_pairs
        .into_iter()
        .map(|(a, b)| get_distance_between_tiles(a, b))
        .collect();

    let sum = distances.into_iter().fold(0 as u64, |total, current| {
        total + u64::try_from(current).unwrap()
    });

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn run_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }

    // No part 2 example is provided

    #[test]
    fn run_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.is_some());
        println!("{}", result.unwrap());
    }
}
