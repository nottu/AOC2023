fn main() {
    let input = include_str!("day_13/input.txt");
    dbg!(part1(input));
    let input = include_str!("day_13/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day_13/test_input.txt");
    assert_eq!(part1(input), "405");
}

fn part1(input: &str) -> String {
    let maps = parse_maps(input);
    maps.iter()
        .map(|map| map_summary(map, 0))
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day_13/test_input.txt");
    assert_eq!(part2(input), "400");
}

fn part2(input: &str) -> String {
    let maps = parse_maps(input);
    maps.iter()
        .map(|map| map_summary(map, 1))
        .sum::<usize>()
        .to_string()
}

fn map_summary(map: &[Vec<Tile>], max_smudges: usize) -> usize {
    let vert = find_vertical_reflection(map, max_smudges);
    let hori = find_horizontal_reflection(map, max_smudges);
    if hori.is_some() && vert.is_some() {
        panic!("two relfections?")
    }
    if hori.is_none() && vert.is_none() {
        panic!("no reflections?")
    }
    vert.unwrap_or(0) + hori.unwrap_or(0) * 100
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Ash,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}

fn parse_maps(input: &str) -> Vec<Vec<Vec<Tile>>> {
    let input = input.lines();

    let mut maps = Vec::new();
    // Vec<Vec<Tile>>
    let mut curr_map = Vec::new();

    for line in input {
        if line.is_empty() {
            maps.push(curr_map);
            curr_map = Vec::new();
        } else {
            curr_map.push(line.chars().map(Tile::from).collect::<Vec<Tile>>());
        }
    }
    if !curr_map.is_empty() {
        maps.push(curr_map);
    }
    maps
}

fn find_vertical_reflection(map: &[Vec<Tile>], max_smudges: usize) -> Option<usize> {
    // dbg!(map.len());
    // let map_rows = map[0].len();
    // dbg!(map_rows);
    (1..(map[0].len())).find(|&idx| has_vertical_reflection(map, idx, max_smudges))
}

fn find_horizontal_reflection(map: &[Vec<Tile>], max_smudges: usize) -> Option<usize> {
    (1..(map.len())).find(|&idx| has_horizontal_reflection(map, idx, max_smudges))
}

fn has_vertical_reflection(map: &[Vec<Tile>], idx: usize, max_smudges: usize) -> bool {
    let mut left_idx = idx;
    let mut right_idx = idx;
    let mut smudge_count = 0;

    while left_idx > 0 && right_idx < map[0].len() {
        left_idx -= 1;
        for (_row_idx, row) in map.iter().enumerate() {
            if row[left_idx] != row[right_idx] {
                smudge_count += 1;
                if smudge_count > max_smudges {
                    return false;
                }
            }
        }
        right_idx += 1;
    }
    smudge_count == max_smudges
}

fn has_horizontal_reflection(map: &[Vec<Tile>], idx: usize, max_smudges: usize) -> bool {
    let mut left_idx = idx;
    let mut right_idx = idx;
    let mut smudge_count = 0;
    while left_idx > 0 && right_idx < map.len() {
        left_idx -= 1;
        for x in 0..map[0].len() {
            if map[left_idx][x] != map[right_idx][x] {
                smudge_count += 1;
                if smudge_count > max_smudges {
                    return false;
                }
            }
        }
        right_idx += 1;
    }
    smudge_count == max_smudges
}
