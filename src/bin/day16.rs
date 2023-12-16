use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("day16/input.txt");
    dbg!(part1(input));
    let input = include_str!("day16/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day16/test_input.txt");
    assert_eq!(part1(input), "46");
}

fn part1(input: &str) -> String {
    let mut map = parse_map(input);
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut beams = VecDeque::new();
    simulate_beams(
        &mut map,
        Beam {
            position: Position { y: 0, x: 0 },
            direction: Direction::Right,
        },
        &mut visited,
        &mut beams,
    );
    engergized(&map).to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day16/test_input.txt");
    assert_eq!(part2(input), "51");
}

fn part2(input: &str) -> String {
    let mut map = parse_map(input);
    let map_height = map.len();
    let map_widht = map[0].len();
    let y_positions = (0..map_height).flat_map(|y| {
        [
            Beam {
                position: Position { y, x: 0 },
                direction: Direction::Right,
            },
            Beam {
                position: Position {
                    y,
                    x: map_widht - 1,
                },
                direction: Direction::Left,
            },
        ]
    });
    let x_positions = (0..map_widht).flat_map(|x| {
        [
            Beam {
                position: Position { y: 0, x },
                direction: Direction::Down,
            },
            Beam {
                position: Position {
                    y: map_height - 1,
                    x,
                },
                direction: Direction::Up,
            },
        ]
    });

    let mut visited: HashSet<Beam> = HashSet::new();
    let mut beams = VecDeque::new();
    let max_activated = y_positions
        .chain(x_positions)
        .map(|beam| {
            // reuse-repair-recycle => less calls to allocate memory from heap
            clear_map(&mut map);
            visited.clear();
            beams.clear();
            simulate_beams(&mut map, beam, &mut visited, &mut beams);
            (beam, engergized(&map))
        })
        .max_by_key(|(_p, energy)| *energy)
        .expect("expected max");
    // dbg!(max_activated);
    max_activated.1.to_string()
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

fn simulate_beams(
    map: &mut [Vec<(Tiles, bool)>],
    start_beam: Beam,
    visited: &mut HashSet<Beam>,
    beams: &mut VecDeque<Beam>,
) {
    let map_height = map.len();
    let map_width = map[0].len();
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
            .filter_map(|direction| next_beam(direction, beam.position, map_width, map_height));
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

fn next_beam(
    direction: Direction,
    prev_position: Position,
    map_width: usize,
    map_height: usize,
) -> Option<Beam> {
    match direction {
        Direction::Right => (prev_position.x + 1 < map_width).then_some(Beam {
            direction,
            position: Position {
                y: prev_position.y,
                x: prev_position.x + 1,
            },
        }),
        Direction::Left => (prev_position.x > 0).then_some(Beam {
            direction,
            position: Position {
                y: prev_position.y,
                x: prev_position.x.saturating_sub(1),
            },
        }),
        Direction::Up => (prev_position.y > 0).then_some(Beam {
            direction,
            position: Position {
                y: prev_position.y.saturating_sub(1),
                x: prev_position.x,
            },
        }),
        Direction::Down => (prev_position.y + 1 < map_height).then_some(Beam {
            direction,
            position: Position {
                y: prev_position.y + 1,
                x: prev_position.x,
            },
        }),
    }
}
