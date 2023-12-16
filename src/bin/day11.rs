use std::{collections::HashSet, usize};

fn main() {
    let input = include_str!("day11/input.txt");
    dbg!(part1(input));
    // should produce the same results
    // assert_eq!(part1(input), part2(input, 2));

    let input = include_str!("day11/input.txt");
    dbg!(part2(input, 1000000));
}

#[test]
fn day11_test_part1() {
    let input = include_str!("day11/test_input.txt");
    assert_eq!(part1(input), "374");
}

fn part1(input: &str) -> String {
    let universe = parse_galaxy(input);
    let universe = expand_universe(&universe);
    let positions = get_galaxy_positions(&universe);

    let mut sum_distances = 0;
    for (idx, p1) in positions.iter().enumerate() {
        for p2 in positions.iter().skip(idx) {
            let dist = get_galaxy_distance(*p1, *p2);

            sum_distances += dist;
        }
    }
    sum_distances.to_string()
}

#[test]
fn day11_test_part2() {
    let input = include_str!("day11/test_input.txt");
    assert_eq!(part2(input, 10), "1030");

    let input = include_str!("day11/test_input.txt");
    assert_eq!(part2(input, 100), "8410");
}

fn part2(input: &str, expansion_rate: usize) -> String {
    let universe = parse_galaxy(input);
    let empty_rows = get_empty_rows(&universe);
    let empty_cols = get_empty_cols(&universe);
    let positions = get_galaxy_positions(&universe);
    let mut sum_distances = 0;
    for (y, g1) in positions.iter().enumerate() {
        for g2 in positions.iter().skip(y) {
            let dist = {
                let vertical_distance_range = if g1.y > g2.y { g2.y..g1.y } else { g1.y..g2.y };
                let horizontal_distance_range = if g1.x > g2.x { g2.x..g1.x } else { g1.x..g2.x };
                let vertical_distance = vertical_distance_range
                    .map(|y| {
                        if empty_rows.contains(&y) {
                            expansion_rate
                        } else {
                            1
                        }
                    })
                    .sum::<usize>();
                let horizontal_distance = horizontal_distance_range
                    .map(|x| {
                        if empty_cols.contains(&x) {
                            expansion_rate
                        } else {
                            1
                        }
                    })
                    .sum::<usize>();
                vertical_distance + horizontal_distance
            };
            sum_distances += dist;
        }
    }
    sum_distances.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Space,
}

fn get_empty_rows(universe: &[Vec<Tile>]) -> HashSet<usize> {
    universe
        .iter()
        .enumerate()
        .filter_map(|(idx, tiles)| tiles.iter().all(|&t| t == Tile::Space).then_some(idx))
        .collect()
}
fn get_empty_cols(universe: &[Vec<Tile>]) -> HashSet<usize> {
    let mut empty_cols = HashSet::new();
    for idx in 0..universe[0].len() {
        if universe.iter().map(|r| r[idx]).all(|t| t == Tile::Space) {
            empty_cols.insert(idx);
        }
    }
    empty_cols
}

fn expand_universe(universe: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    // find row with no galaxies
    let empty_rows = get_empty_rows(universe);
    let empty_cols = get_empty_cols(universe);

    let mut expanded = vec![];
    for (y, row) in universe.iter().enumerate() {
        let mut expaned_row = vec![];
        for (x, tile) in row.iter().enumerate() {
            expaned_row.push(*tile);
            if empty_cols.contains(&x) {
                expaned_row.push(*tile);
            }
        }
        expanded.push(expaned_row.clone());
        if empty_rows.contains(&y) {
            expanded.push(expaned_row);
        }
    }
    expanded
}

fn parse_galaxy(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|row| {
            row.trim()
                .chars()
                .map(|c| match c {
                    '#' => Tile::Galaxy,
                    '.' => Tile::Space,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    y: usize,
    x: usize,
}

fn get_galaxy_positions(universe: &[Vec<Tile>]) -> Vec<Position> {
    universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, tile)| (*tile == Tile::Galaxy).then_some(Position { x, y }))
                .collect::<Vec<Position>>()
        })
        .collect()
}

fn get_galaxy_distance(g1: Position, g2: Position) -> usize {
    let vertical_distance = if g1.y < g2.y {
        g2.y - g1.y
    } else {
        g1.y - g2.y
    };

    let horizontal_diatance = if g1.x < g2.x {
        g2.x - g1.x
    } else {
        g1.x - g2.x
    };

    vertical_distance + horizontal_diatance
}
