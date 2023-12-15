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
struct MapTile {
    category: TileCategory,
    x: usize,
    y: usize,
}

fn parse_part_one_input_to_galaxies(input: &str) -> Vec<MapTile> {
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

    let mut extra_col_offset: usize = 0;
    let expanded_tile_category_grid: Vec<Vec<TileCategory>> = base_tile_category_grid
        .into_iter()
        .flat_map(|row| {
            return if row.contains(&TileCategory::Galaxy) {
                vec![row]
            } else {
                vec![row.clone(), row]
            };
        })
        .map(|row| {
            row.into_iter()
                .enumerate()
                .flat_map(|(col_index, tile_category)| {
                    return if empty_cols.contains(&(col_index + extra_col_offset)) {
                        // extra_col_offset += 1;
                        // IF THIS WORKS<, REMOVE `extra_col_offset` ENTIRELY
                        vec![tile_category, tile_category]
                    } else {
                        vec![tile_category]
                    };
                })
                .collect()
        })
        .collect();

    let mut double_enumerated_grid: Vec<(usize, usize, TileCategory)> = vec![];
    for (row_index, row) in expanded_tile_category_grid.iter().enumerate() {
        for (col_index, &tile_category) in row.iter().enumerate() {
            double_enumerated_grid.push((col_index, row_index, tile_category));
        }
    }

    return double_enumerated_grid
        .into_iter()
        .filter(|(_, _, category)| category == &TileCategory::Galaxy)
        .map(|(x, y, category)| MapTile { category, x, y })
        .collect();
}

fn get_distance_between_tiles(tile_a: MapTile, tile_b: MapTile) -> usize {
    let x_diff = tile_a.x.abs_diff(tile_b.x);
    let y_diff = tile_a.y.abs_diff(tile_b.y);

    return x_diff + y_diff;
}

pub fn part_one(input: &str) -> Option<u32> {
    let galaxies = parse_part_one_input_to_galaxies(input);
    let galaxy_pairs = pairs(galaxies);
    let distances: Vec<usize> = galaxy_pairs
        .into_iter()
        .map(|(a, b)| get_distance_between_tiles(a, b))
        .collect();

    let sum = distances.into_iter().fold(0 as u32, |total, current| {
        total + u32::try_from(current).unwrap()
    });

    return Some(sum);
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
        assert_eq!(result, Some(374));
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
