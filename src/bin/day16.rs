use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("day16/input.txt");
    dbg!(part1(input));
    let input = include_str!("day16/input.txt");
    dbg!(part2(input));
}

#[test]
fn day16_test_part1() {
    let input = include_str!("day16/test_input.txt");
    assert_eq!(part1(input), "46");
}

fn part1(input: &str) -> String {
    let mut map = parse_map(input);
    let mut visited: HashSet<Beam> = HashSet::new();
    simulate_beams(
        &mut map,
        Beam {
            position: Position { y: 0, x: 0 },
            direction: Direction::Right,
        },
        &mut visited,
    );
    engergized(&map).to_string()
}

#[test]
fn day16_test_part2() {
    let input = include_str!("day16/test_input.txt");
    assert_eq!(part2(input), "145");
}

fn part2(input: &str) -> String {
    let mut map = parse_map(input);
    let mut max_activated = 0;
    let map_height = map.len();
    let map_widht = map[0].len();
    let mut visited: HashSet<Beam> = HashSet::new();
    for y in 0..map_height {
        clear_map(&mut map);
        visited.clear();
        simulate_beams(
            &mut map,
            Beam {
                position: Position { y, x: 0 },
                direction: Direction::Right,
            },
            &mut visited,
        );
        max_activated = std::cmp::max(max_activated, engergized(&map));

        clear_map(&mut map);
        visited.clear();
        simulate_beams(
            &mut map,
            Beam {
                position: Position {
                    y,
                    x: map_widht - 1,
                },
                direction: Direction::Left,
            },
            &mut visited,
        );
        max_activated = std::cmp::max(max_activated, engergized(&map));
    }
    for x in 0..map_widht {
        clear_map(&mut map);
        simulate_beams(
            &mut map,
            Beam {
                position: Position { y: 0, x },
                direction: Direction::Down,
            },
            &mut visited,
        );
        max_activated = std::cmp::max(max_activated, engergized(&map));

        clear_map(&mut map);
        simulate_beams(
            &mut map,
            Beam {
                position: Position {
                    y: map_height - 1,
                    x,
                },
                direction: Direction::Left,
            },
            &mut visited,
        );
        max_activated = std::cmp::max(max_activated, engergized(&map));
    }

    max_activated.to_string()
}

#[inline]
fn engergized(map: &[Vec<(Tiles, bool)>]) -> usize {
    map.iter()
        .flatten()
        .filter(|(_t, activated)| *activated)
        .count()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tiles {
    Empty,
    MirrorL,
    MirrorR,
    SplitterV,
    SplitterH,
}

impl Tiles {
    fn interact(&self, light_direction: Direction) -> [Option<Direction>; 2] {
        match (self, light_direction) {
            (Self::Empty, _) => [Some(light_direction), None],
            // Splitters
            (Self::SplitterV, Direction::Down | Direction::Up) => [Some(light_direction), None],
            (Self::SplitterH, Direction::Right | Direction::Left) => [Some(light_direction), None],
            (Self::SplitterH, Direction::Up | Direction::Down) => {
                [Some(Direction::Left), Some(Direction::Right)]
            }
            (Self::SplitterV, Direction::Left | Direction::Right) => {
                [Some(Direction::Up), Some(Direction::Down)]
            }
            // Mirror Left
            (Self::MirrorL, Direction::Up) => [Some(Direction::Right), None],
            (Self::MirrorL, Direction::Down) => [Some(Direction::Left), None],
            (Self::MirrorL, Direction::Left) => [Some(Direction::Down), None],
            (Self::MirrorL, Direction::Right) => [Some(Direction::Up), None],
            // Mirror Right
            (Self::MirrorR, Direction::Up) => [Some(Direction::Left), None],
            (Self::MirrorR, Direction::Down) => [Some(Direction::Right), None],
            (Self::MirrorR, Direction::Left) => [Some(Direction::Up), None],
            (Self::MirrorR, Direction::Right) => [Some(Direction::Down), None],
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<(Tiles, bool)>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Tiles::Empty,
                    '/' => Tiles::MirrorL,
                    '\\' => Tiles::MirrorR,
                    '|' => Tiles::SplitterV,
                    '-' => Tiles::SplitterH,
                    _ => panic!("no"),
                })
                .map(|t| (t, false))
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
}

fn simulate_beams(map: &mut [Vec<(Tiles, bool)>], start_beam: Beam, visited: &mut HashSet<Beam>) {
    let mut beams = VecDeque::new();
    let map_width = map[0].len();
    let map_height = map.len();
    beams.push_back(start_beam);
    let mut remaining_steps = 10000000;
    while !beams.is_empty() && remaining_steps != 0 {
        remaining_steps -= 1;
        let beam = beams.pop_front().expect("expected beam");
        visited.insert(beam);
        let (curr_tile, _) = map[beam.position.y][beam.position.x];
        map[beam.position.y][beam.position.x].1 = true;
        let beam_interactions = curr_tile.interact(beam.direction);
        let new_beams = beam_interactions
            .into_iter()
            .flatten()
            .filter_map(|direction| match direction {
                Direction::Right => {
                    if beam.position.x + 1 < map_width {
                        let position = Position {
                            y: beam.position.y,
                            x: beam.position.x + 1,
                        };
                        Some(Beam {
                            direction,
                            position,
                        })
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if beam.position.x > 0 {
                        let position = Position {
                            y: beam.position.y,
                            x: beam.position.x - 1,
                        };
                        Some(Beam {
                            direction,
                            position,
                        })
                    } else {
                        None
                    }
                }
                Direction::Up => {
                    if beam.position.y > 0 {
                        let position = Position {
                            y: beam.position.y - 1,
                            x: beam.position.x,
                        };
                        Some(Beam {
                            direction,
                            position,
                        })
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if beam.position.y + 1 < map_height {
                        let position = Position {
                            y: beam.position.y + 1,
                            x: beam.position.x,
                        };
                        Some(Beam {
                            direction,
                            position,
                        })
                    } else {
                        None
                    }
                }
            });
        for beam in new_beams.filter(|beam| !visited.contains(beam)) {
            beams.push_back(beam);
        }
    }
}

#[inline]
fn clear_map(map: &mut [Vec<(Tiles, bool)>]) {
    for row in map {
        for tile in row {
            tile.1 = false
        }
    }
}
