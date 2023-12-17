use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("day_17/input.txt");
    dbg!(part1(input));
    let input = include_str!("day_17/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day_17/test_input.txt");
    assert_eq!(part1(input), "102");
}

fn part1(input: &str) -> String {
    let map = parse_map(input);
    minimize_heat_bfs(
        &map,
        Position { y: 0, x: 0 },
        Position {
            y: map.len() - 1,
            x: map[0].len() - 1,
        },
        0,
        3,
    )
    .to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day_17/test_input.txt");
    assert_eq!(part2(input), "94");
}

fn part2(input: &str) -> String {
    let map = parse_map(input);
    minimize_heat_bfs(
        &map,
        Position { y: 0, x: 0 },
        Position {
            y: map.len() - 1,
            x: map[0].len() - 1,
        },
        4,
        10,
    )
    .to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    heat_loss: usize,
}

fn parse_map(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c as usize - '0' as usize)
                .map(|heat_loss| Tile { heat_loss })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn get_next_directions(&self) -> [Self; 3] {
        match self {
            Direction::Down => [Self::Right, Self::Left, Self::Down],
            Direction::Up => [Self::Right, Self::Left, Self::Up],
            Direction::Left => [Self::Left, Self::Down, Self::Up],
            Direction::Right => [Self::Right, Self::Down, Self::Up],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct MoveInfo {
    curr_position: Position,
    prev_direction: Direction,
    min_consecutive_count: usize,
    max_consecutive_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct PathData {
    move_info: MoveInfo,
    curr_loss: usize,
}

impl Ord for PathData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.curr_loss.cmp(&self.curr_loss)
    }
}
impl PartialOrd for PathData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn minimize_heat_bfs(
    map: &[Vec<Tile>],
    start_position: Position,
    end_position: Position,
    min_consecutive_count: usize,
    max_consecutive_count: usize,
) -> usize {
    let map_height = map.len();
    let map_width = map[0].len();
    let mut visited: HashMap<MoveInfo, usize> = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    let move_info = MoveInfo {
        curr_position: start_position,
        prev_direction: Direction::Right,
        min_consecutive_count,
        max_consecutive_count,
    };
    to_visit.push(PathData {
        move_info,
        curr_loss: 0,
    });

    let mut min_heat_loss: usize = usize::MAX;
    while let Some(visiting) = to_visit.pop() {
        if visiting.curr_loss >= min_heat_loss {
            continue;
        }
        // println!("{max_visited_cnt} : {visiting:?}");
        if visiting.move_info.curr_position == end_position {
            min_heat_loss = min_heat_loss.min(visiting.curr_loss);
            // println!("HERE");
            continue;
        }
        let move_info = visiting.move_info;
        let heat_loss = visiting.curr_loss;
        if let Some(prev_loss) = visited.get(&move_info) {
            if *prev_loss <= heat_loss {
                continue;
            }
            visited.insert(move_info, heat_loss);
        }
        visited.insert(move_info, heat_loss);
        let next = move_info
            .prev_direction
            .get_next_directions()
            .map(|direction| {
                if (move_info.prev_direction != direction && move_info.min_consecutive_count > 0)
                    || (move_info.prev_direction == direction
                        && move_info.max_consecutive_count == 0)
                {
                    None
                } else {
                    let max_consecutive_count = if move_info.prev_direction == direction {
                        move_info.max_consecutive_count
                    } else {
                        max_consecutive_count
                    } - 1;

                    let min_consecutive_count = if move_info.prev_direction == direction {
                        move_info.min_consecutive_count
                    } else {
                        min_consecutive_count
                    }
                    .saturating_sub(1);

                    let new_position =
                        get_new_position(map_width, map_height, direction, move_info.curr_position);
                    new_position.map(|position| {
                        (
                            MoveInfo {
                                max_consecutive_count,
                                min_consecutive_count,
                                curr_position: position,
                                prev_direction: direction,
                            },
                            heat_loss + map[position.y][position.x].heat_loss,
                        )
                    })
                }
            })
            .into_iter()
            .flatten();
        for (move_info, curr_loss) in next {
            to_visit.push(PathData {
                move_info,
                curr_loss,
            })
        }
    }
    min_heat_loss
}

fn get_new_position(
    map_width: usize,
    map_height: usize,
    direction: Direction,
    curr_positoin: Position,
) -> Option<Position> {
    match direction {
        Direction::Right => {
            if curr_positoin.x + 1 == map_width {
                None
            } else {
                Some(Position {
                    y: curr_positoin.y,
                    x: curr_positoin.x + 1,
                })
            }
        }
        Direction::Left => {
            if curr_positoin.x == 0 {
                None
            } else {
                Some(Position {
                    y: curr_positoin.y,
                    x: curr_positoin.x - 1,
                })
            }
        }
        Direction::Up => {
            if curr_positoin.y == 0 {
                None
            } else {
                Some(Position {
                    y: curr_positoin.y - 1,
                    x: curr_positoin.x,
                })
            }
        }
        Direction::Down => {
            if curr_positoin.y + 1 == map_height {
                None
            } else {
                Some(Position {
                    y: curr_positoin.y + 1,
                    x: curr_positoin.x,
                })
            }
        }
    }
}
