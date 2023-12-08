use std::collections::HashMap;

fn main() {
    let input = include_str!("day8/input.txt");
    dbg!(part1(input));

    let input = include_str!("day8/input.txt");
    dbg!(part2(input));
}

#[test]
fn day8_test_part1() {
    let input = include_str!("day8/test_input.txt");
    assert_eq!(part1(input), "2");

    let input = include_str!("day8/test_input2.txt");
    assert_eq!(part1(input), "6");
}

fn part1(input: &str) -> String {
    let (directions, map) = parse_input(input);
    let curr_node = "AAA".to_string();
    get_steps(curr_node, &map, &directions, |s: &str| s == "ZZZ").to_string()
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("???"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut input_lines = input.lines();
    let directions: Vec<Direction> = input_lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from)
        .collect();
    // one empty line
    input_lines.next();
    // parse nodes
    let node_data = input_lines.map(|l| {
        let mut node = l.split('=').map(|i| i.trim());
        let node_name = node.next().unwrap();
        let next_nodes = node.next().unwrap().replace(['(', ')'], "");
        let next_nodes: Vec<_> = next_nodes.split(',').map(|n| n.trim()).collect();
        (
            node_name.to_owned(),
            next_nodes[0].to_owned(),
            next_nodes[1].to_owned(),
        )
    });
    let mut node_map = HashMap::new();
    for data in node_data {
        node_map.insert(
            data.0.to_owned(),
            Node {
                left: data.1,
                right: data.2,
            },
        );
    }
    (directions, node_map)
}

#[test]
fn day8_test_part2() {
    let input = include_str!("day8/test_input3.txt");
    assert_eq!(part2(input), "6")
}

fn part2(input: &str) -> String {
    let (directions, map) = parse_input(input);
    let curr_nodes: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| n.to_owned())
        .collect();

    let check_is_end = |s: &str| s.ends_with('Z');
    curr_nodes
        .into_iter()
        .map(|n| get_steps(n, &map, &directions, check_is_end))
        .fold(1, num::integer::lcm)
        .to_string()
}

fn get_steps(
    mut curr_node: String,
    map: &HashMap<String, Node>,
    directions: &Vec<Direction>,
    check_is_end: fn(&str) -> bool,
) -> usize {
    let mut step_count = 0;
    let num_directions = directions.len();

    while !check_is_end(&curr_node) {
        let next_dirs = &map[&curr_node];
        curr_node = match directions[step_count % num_directions] {
            Direction::Left => &next_dirs.left,
            Direction::Right => &next_dirs.right,
        }
        .to_string();
        step_count += 1;
    }
    step_count
}
