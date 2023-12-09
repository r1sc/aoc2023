use std::collections::HashMap;

use num::integer::lcm;

use crate::aoc;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct DirectionMap {
    directions: Vec<char>,
    nodes: HashMap<String, Node>,
}

impl DirectionMap {
    pub fn new(lines: &[String]) -> Self {
        let cleaned_lines: Vec<_> = lines.iter().filter(|f| !f.is_empty()).collect();
        let directions: Vec<_> = cleaned_lines[0].chars().collect();

        let mut nodes = HashMap::new();
        for line in &cleaned_lines[1..] {
            let lr: Vec<_> = line.split(" = ").collect();

            let without_parens = lr[1].replace(['(', ')'], "");
            let left_right: Vec<_> = without_parens.split(", ").collect();

            nodes.insert(
                lr[0].to_string(),
                Node {
                    left: left_right[0].to_string(),
                    right: left_right[1].to_string(),
                },
            );
        }

        Self { directions, nodes }
    }
}

fn get_path_length(map: &DirectionMap, starting_node_name: &str, use_ends_with_z: bool) -> u64 {
    let mut directions = map.directions.iter().cycle();
    let mut current_node_name = starting_node_name;
    let mut num_steps = 0;

    while if use_ends_with_z {
        !current_node_name.ends_with('Z')
    } else {
        current_node_name != "ZZZ"
    } {
        let current_node = map.nodes.get(current_node_name).unwrap();
        let direction = directions.next().unwrap();
        if *direction == 'L' {
            current_node_name = &current_node.left;
        } else {
            current_node_name = &current_node.right;
        }
        num_steps += 1;
    }

    num_steps
}

fn part1(map: &DirectionMap) -> u64 {
    get_path_length(map, "AAA", false)
}

fn part2(map: &DirectionMap) -> u64 {
    let nodes_names_starting_with_a: Vec<_> = map
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();

    let path_lengths: Vec<_> = nodes_names_starting_with_a
        .iter()
        .map(|n| get_path_length(map, n, true))
        .collect();

    let mut result = path_lengths[0];
    for path_length in &path_lengths[1..] {
        result = lcm(result, *path_length);
    }

    result
}

pub fn run() -> (String, String) {
    let lines = aoc::lines_from_file("day8.txt");
    let map = DirectionMap::new(&lines);

    let part1_result = part1(&map);
    let part2_result = part2(&map);

    (part1_result.to_string(), part2_result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        aoc,
        days::day8::{part1, part2, DirectionMap},
    };

    #[test]
    fn part_1_test() {
        let lines = aoc::lines_from_test(
            r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        let map = DirectionMap::new(&lines);
        assert_eq!(part1(&map), 6);
    }

    #[test]
    fn part_2_test() {
        let lines = aoc::lines_from_test(
            r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );

        let map = DirectionMap::new(&lines);
        assert_eq!(part2(&map), 6);
    }
}
