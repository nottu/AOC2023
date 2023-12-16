fn main() {
    let input = include_str!("day10/input.txt");
    dbg!(part1(input));

    let input = include_str!("day10/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day10/test_input.txt");
    assert_eq!(part1(input), "4");
}

fn part1(input: &str) -> String {
    let (start_pos, map) = parse_map(input);
    (loop_size(start_pos, &map) / 2).to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day10/test_input2.txt");
    assert_eq!(part2(input), "4");

    let input = include_str!("day10/test_input3.txt");
    assert_eq!(part2(input), "8");

    let input = include_str!("day10/test_input4.txt");
    assert_eq!(part2(input), "10");
}

fn part2(input: &str) -> String {
    let (start_position, mut map) = parse_map(input);
    let loop_tiles = find_loop(start_position, &map);

    map[start_position.y][start_position.x] = find_starting_tile(start_position, &map);
    find_inner_tiles(&loop_tiles, &map).to_string()
}

fn find_starting_tile(start_position: Position, map: &[Vec<Tile>]) -> Tile {
    let start_neighbours: Vec<_> = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .filter(|&dir| get_next_position(start_position, dir, map).is_some())
    .collect();
    dbg!(&start_neighbours);
    if start_neighbours.len() != 2 {
        panic!("too many conections");
    }
    match (start_neighbours[0], start_neighbours[1]) {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => Tile::VerticalPipe,

        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
            Tile::HorizontalPipe
        }
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => Tile::BendNE,
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => Tile::BendNW,
        (Direction::Right, Direction::Down) | (Direction::Down, Direction::Right) => Tile::BendSE,
        (Direction::Left, Direction::Down) | (Direction::Down, Direction::Left) => Tile::BendSW,
        _ => panic!("weird combo!"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    StartPosition,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

fn parse_map(input: &str) -> (Position, Vec<Vec<Tile>>) {
    let mut start_position = Position::default();
    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Tile::VerticalPipe,
                    '-' => Tile::HorizontalPipe,
                    'L' => Tile::BendNE,
                    'J' => Tile::BendNW,
                    '7' => Tile::BendSW,
                    'F' => Tile::BendSE,
                    '.' => Tile::Ground,
                    'S' => {
                        start_position = Position { x, y };
                        Tile::StartPosition
                    }
                    _ => Tile::Ground,
                })
                .collect::<Vec<Tile>>()
        })
        .collect();
    (start_position, tiles)
}

fn loop_size(start_position: Position, map: &[Vec<Tile>]) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    find_loop_size_recursive(start_position, map, &mut visited)
}

fn find_loop(start_position: Position, map: &[Vec<Tile>]) -> Vec<Vec<bool>> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    find_loop_size_recursive(start_position, map, &mut visited);
    visited[start_position.y][start_position.x] = true;
    visited
}

fn find_loop_size_recursive(
    curr_position: Position,
    map: &[Vec<Tile>],
    visited: &mut [Vec<bool>],
) -> usize {
    let next_to_visit = get_next_neighbours(curr_position, map);
    let mut max_steps = 0;
    for next in next_to_visit {
        if tile_at_position(next, map) == Tile::StartPosition {
            max_steps = std::cmp::max(max_steps, 1);
        } else {
            if visited[next.y][next.x] {
                continue;
            }
            visited[next.y][next.x] = true;
            let steps = find_loop_size_recursive(next, map, visited);
            if steps > 0 {
                max_steps = std::cmp::max(max_steps, steps + 1);
            }
        }
    }
    max_steps
}

#[inline]
fn tile_at_position(position: Position, map: &[Vec<Tile>]) -> Tile {
    map[position.y][position.x]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next_position(
    curr_position: Position,
    direction: Direction,
    map: &[Vec<Tile>],
) -> Option<Position> {
    let Position { x, y } = curr_position;
    match direction {
        Direction::Up => {
            if y < 1 {
                return None;
            }
            let next_pos = Position { x, y: y - 1 };
            let can_connect = tile_at_position(next_pos, map) == Tile::VerticalPipe
                || tile_at_position(next_pos, map) == Tile::BendSE
                || tile_at_position(next_pos, map) == Tile::BendSW
                || tile_at_position(next_pos, map) == Tile::StartPosition;
            can_connect.then_some(next_pos)
        }
        Direction::Down => {
            if y > map.len() - 1 {
                return None;
            }
            let next_pos = Position { x, y: y + 1 };
            let can_connect = tile_at_position(next_pos, map) == Tile::VerticalPipe
                || tile_at_position(next_pos, map) == Tile::BendNE
                || tile_at_position(next_pos, map) == Tile::BendNW
                || tile_at_position(next_pos, map) == Tile::StartPosition;
            can_connect.then_some(next_pos)
        }
        Direction::Left => {
            if x < 1 {
                return None;
            }
            let next_pos = Position { x: x - 1, y };
            let can_connect = tile_at_position(next_pos, map) == Tile::HorizontalPipe
                || tile_at_position(next_pos, map) == Tile::BendNE
                || tile_at_position(next_pos, map) == Tile::BendSE
                || tile_at_position(next_pos, map) == Tile::StartPosition;
            can_connect.then_some(next_pos)
        }
        Direction::Right => {
            if x > map[y].len() - 1 {
                return None;
            }
            let next_pos = Position { x: x + 1, y };
            let can_connect = tile_at_position(next_pos, map) == Tile::HorizontalPipe
                || tile_at_position(next_pos, map) == Tile::BendNW
                || tile_at_position(next_pos, map) == Tile::BendSW
                || tile_at_position(next_pos, map) == Tile::StartPosition;
            can_connect.then_some(next_pos)
        }
    }
}

fn get_next_neighbours(curr_position: Position, map: &[Vec<Tile>]) -> Vec<Position> {
    match tile_at_position(curr_position, map) {
        Tile::VerticalPipe => [
            get_next_position(curr_position, Direction::Up, map),
            get_next_position(curr_position, Direction::Down, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::HorizontalPipe => [
            get_next_position(curr_position, Direction::Left, map),
            get_next_position(curr_position, Direction::Right, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::BendNE => [
            get_next_position(curr_position, Direction::Up, map),
            get_next_position(curr_position, Direction::Right, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::BendNW => [
            get_next_position(curr_position, Direction::Up, map),
            get_next_position(curr_position, Direction::Left, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::BendSW => [
            get_next_position(curr_position, Direction::Left, map),
            get_next_position(curr_position, Direction::Down, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::BendSE => [
            get_next_position(curr_position, Direction::Right, map),
            get_next_position(curr_position, Direction::Down, map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>(),
        Tile::StartPosition => {
            //we don't know where we can go...
            [
                get_next_position(curr_position, Direction::Up, map),
                get_next_position(curr_position, Direction::Left, map),
                get_next_position(curr_position, Direction::Right, map),
                get_next_position(curr_position, Direction::Down, map),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<Position>>()
        }
        Tile::Ground => panic!("in gournd!"),
    }
}

/// Does a Left to Right sweep, counting the number of tiles in the way.
/// If the number of tiles is even, we are inside the loop, otherwise we are not
fn find_inner_tiles(is_loop: &[Vec<bool>], map: &[Vec<Tile>]) -> usize {
    let mut num_inner = 0;
    for y in 0..is_loop.len() {
        for x in 0..is_loop[y].len() {
            if is_loop[y][x] {
                continue;
            }
            if is_in_loop(Position { x, y }, is_loop, map) {
                num_inner += 1;
            }
        }
    }
    num_inner
}

fn is_in_loop(position: Position, is_loop: &[Vec<bool>], map: &[Vec<Tile>]) -> bool {
    let Position { x, y } = position;
    let mut num_crossings = 0;
    let mut prev_crossing_tile = Tile::Ground;
    for (is_loop, tile) in is_loop[y].iter().zip(map[y].iter()).skip(x) {
        if *tile == Tile::Ground {
            prev_crossing_tile = *tile;
            continue;
        }
        if !is_loop || *tile == Tile::HorizontalPipe {
            continue;
        }
        if (prev_crossing_tile == Tile::BendSE && *tile == Tile::BendNW)
            || (prev_crossing_tile == Tile::BendNE && *tile == Tile::BendSW)
        {
        } else {
            num_crossings += 1
        }
        prev_crossing_tile = *tile;
    }
    num_crossings % 2 == 1
}
