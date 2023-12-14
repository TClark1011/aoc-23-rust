use std::collections::HashMap;

// use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash)]
enum NavigationDirection {
    Left,
    Right,
}

fn parse_navigation_direction_character(character: char) -> NavigationDirection {
    return match character {
        'L' => NavigationDirection::Left,
        'R' => NavigationDirection::Right,
        _ => panic!("invalid navigation direction character"),
    };
}

#[derive(Clone)]
struct NavigationNode {
    code: String,
    left_node_code: String,
    right_node_code: String,
}

type NavigationNodeBranches = HashMap<NavigationDirection, String>;

enum ParseNavigationNodeLineError {
    LineDoesNotMatchPattern,
}

fn parse_navigation_node_line(line: &str) -> Result<NavigationNode, ParseNavigationNodeLineError> {
    // Regex is much slower than this split approach
    match line.split_once(" = (") {
        Some((code, rest)) => match rest.split_once(", ") {
            Some((left_code, second_rest)) => match second_rest.split_once(")") {
                Some((right_code, _)) => {
                    return Ok(NavigationNode {
                        code: code.to_string(),
                        left_node_code: left_code.to_string(),
                        right_node_code: right_code.to_string(),
                    });
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    };

    return Err(ParseNavigationNodeLineError::LineDoesNotMatchPattern);
}

struct NavigationNodeTraverser {
    active_node_code: String,
    nodes_map: HashMap<String, NavigationNodeBranches>,
}

impl NavigationNodeTraverser {
    fn parse_from_input(input: &str) -> Self {
        let mut nodes_map: HashMap<String, NavigationNodeBranches> = HashMap::new();

        input
            .lines()
            .filter_map(|line| parse_navigation_node_line(line).ok())
            .for_each(|node| {
                nodes_map.insert(
                    node.code.clone(),
                    HashMap::from([
                        (NavigationDirection::Left, node.left_node_code),
                        (NavigationDirection::Right, node.right_node_code),
                    ]),
                );
            });

        NavigationNodeTraverser {
            active_node_code: "AAA".to_string(),
            nodes_map,
        }
    }

    fn get_active_node_branches(&self) -> &NavigationNodeBranches {
        self.nodes_map.get(&self.active_node_code).unwrap()
    }

    fn go_direction(&mut self, direction: &NavigationDirection) {
        self.active_node_code = self
            .get_active_node_branches()
            .get(direction)
            .unwrap()
            .clone();
    }

    fn follow_directions_to_node_code(
        &mut self,
        directions: Vec<NavigationDirection>,
        destination_code: &str,
    ) -> u32 {
        let mut previous_steps: u32 = 0;

        while &self.active_node_code != destination_code {
            // println!(
            //     "Step Number: {}, current code: {} ",
            //     previous_steps, self.active_node_code
            // );
            let destination_index = usize::try_from(previous_steps).unwrap() % directions.len();
            let next_direction = directions.get(destination_index).unwrap();

            let branches = self.get_active_node_branches();
            if branches.values().any(|v| v == destination_code) {
                println!(
                    "One step away from solution (previous-steps: {}, current-code: {})",
                    previous_steps, self.active_node_code
                );
            }

            self.go_direction(next_direction);
            previous_steps += 1;
        }

        return previous_steps;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let directions: Vec<NavigationDirection> = lines[0]
        .chars()
        .map(parse_navigation_direction_character)
        .collect();
    // println!("Directions parsed");

    let mut traverser = NavigationNodeTraverser::parse_from_input(input);
    println!("traverser created");

    let steps = traverser.follow_directions_to_node_code(directions, &"ZZZ");

    Some(steps)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
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
