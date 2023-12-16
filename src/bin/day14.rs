use std::collections::HashMap;

fn main() {
    let input = include_str!("day14/input.txt");
    dbg!(part1(input));
    let input = include_str!("day14/input.txt");
    dbg!(part2(input));
}

#[test]
fn day14_test_part1() {
    let input = include_str!("day14/test_input.txt");
    assert_eq!(part1(input), "136");
}

fn part1(input: &str) -> String {
    let map = parse_map(input);
    let mut score = 0;
    for x in 0..map[0].len() {
        let mut load = map.len();
        for y in 0..map.len() {
            match map[y][x] {
                Tiles::Empty => (),
                Tiles::Square => load = map.len() - y - 1,
                Tiles::Round => {
                    score += load;
                    load -= 1
                }
            }
        }
    }
    score.to_string()
}

#[test]
fn day14_test_part2() {
    let input = include_str!("day14/test_input.txt");
    assert_eq!(part2(input), "64");
}

fn part2(input: &str) -> String {
    let mut map = parse_map(input);
    const NUM_ITERATIONS: usize = 1_000_000_000;
    // scores will start cycling eventually
    // we need to detect the cycle and then with that
    // extrapolate the socre at the 1M iteration
    let mut score_map: HashMap<Vec<Vec<Tiles>>, (usize, usize)> = HashMap::new();
    for idx in 1..=NUM_ITERATIONS {
        tilt_full_round(&mut map);
        if score_map.contains_key(&map) {
            let (prev_idx, prev_score) = score_map[&map];
            let cycle_len = idx - prev_idx;
            if (NUM_ITERATIONS - idx) % cycle_len == 0 {
                return prev_score.to_string();
            }
            continue;
        }
        let score = calculate_score(&map);
        // cloning the whole map... not very efficient but works...
        score_map.insert(map.clone(), (idx, score));
    }
    unreachable!("How long did it take?")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tiles {
    Round,
    Square,
    Empty,
}

fn parse_map(input: &str) -> Vec<Vec<Tiles>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Tiles::Empty,
                    'O' => Tiles::Round,
                    '#' => Tiles::Square,
                    _ => panic!("no"),
                })
                .collect()
        })
        .collect()
}

fn tilt_map_north(map: &mut [Vec<Tiles>]) {
    for x in 0..map[0].len() {
        let mut last_empty = 0;
        let mut rock_to_move = 0;
        for y in 0..map.len() {
            match map[y][x] {
                Tiles::Empty => (),
                Tiles::Square => {
                    // move the rocks
                    for new_y in 0..rock_to_move {
                        map[new_y + last_empty][x] = Tiles::Round
                    }
                    rock_to_move = 0;
                    last_empty = y + 1
                }
                Tiles::Round => {
                    map[y][x] = Tiles::Empty;
                    rock_to_move += 1;
                }
            }
        }
        for new_y in 0..rock_to_move {
            map[new_y + last_empty][x] = Tiles::Round
        }
    }
}

fn tilt_map_south(map: &mut [Vec<Tiles>]) {
    for x in 0..map[0].len() {
        let mut last_empty = map.len() - 1;
        let mut rock_to_move = 0;
        for y in (0..map.len()).rev() {
            match map[y][x] {
                Tiles::Empty => (),
                Tiles::Square => {
                    // move the rocks
                    for new_y in 0..rock_to_move {
                        map[last_empty - new_y][x] = Tiles::Round
                    }
                    rock_to_move = 0;
                    last_empty = y.saturating_sub(1)
                }
                Tiles::Round => {
                    map[y][x] = Tiles::Empty;
                    rock_to_move += 1;
                }
            }
        }
        for new_y in 0..rock_to_move {
            map[last_empty - new_y][x] = Tiles::Round
        }
    }
}

fn tilt_map_west(map: &mut [Vec<Tiles>]) {
    for y in 0..map.len() {
        let mut last_empty = 0;
        let mut rock_to_move = 0;
        for x in 0..map[0].len() {
            match map[y][x] {
                Tiles::Empty => (),
                Tiles::Square => {
                    // move the rocks
                    for new_x in 0..rock_to_move {
                        map[y][new_x + last_empty] = Tiles::Round
                    }
                    rock_to_move = 0;
                    last_empty = x + 1
                }
                Tiles::Round => {
                    map[y][x] = Tiles::Empty;
                    rock_to_move += 1;
                }
            }
        }
        for new_x in 0..rock_to_move {
            map[y][new_x + last_empty] = Tiles::Round
        }
    }
}

fn tilt_map_east(map: &mut [Vec<Tiles>]) {
    for y in 0..map.len() {
        let mut last_empty = map.len() - 1;
        let mut rock_to_move = 0;
        for x in (0..map[0].len()).rev() {
            match map[y][x] {
                Tiles::Empty => (),
                Tiles::Square => {
                    // move the rocks
                    for new_x in 0..rock_to_move {
                        map[y][last_empty - new_x] = Tiles::Round
                    }
                    rock_to_move = 0;
                    last_empty = x.saturating_sub(1)
                }
                Tiles::Round => {
                    map[y][x] = Tiles::Empty;
                    rock_to_move += 1;
                }
            }
        }
        // println!("{y}, {rock_to_move} {last_empty}");
        for new_x in 0..rock_to_move {
            map[y][last_empty - new_x] = Tiles::Round
        }
    }
}

fn tilt_full_round(map: &mut [Vec<Tiles>]) {
    tilt_map_north(map);
    tilt_map_west(map);
    tilt_map_south(map);
    tilt_map_east(map);
}

fn calculate_score(map: &[Vec<Tiles>]) -> usize {
    let num_rows = map.len();
    map.iter().enumerate().fold(0, |acc, (row_idx, row)| {
        acc + row.iter().filter(|tile| **tile == Tiles::Round).count() * (num_rows - row_idx)
    })
}
