use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("day_03/input.txt");
    let output = part1(input);
    dbg!(output);

    let input = include_str!("day_03/input.txt");
    let output = part2(input);
    dbg!(output);
}

#[test]
fn test_part1() {
    let input = include_str!("day_03/test_input.txt");
    assert_eq!(part1(input), "4361")
}

fn part1(input: &str) -> String {
    let engine_scheme: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut total_sum: usize = 0;
    for (y, row) in engine_scheme.iter().enumerate() {
        const ZERO_VAL: usize = '0' as usize;
        let mut was_adjacent = false;
        let mut num: usize = 0;
        for (x, val) in row.iter().enumerate() {
            if val.is_ascii_digit() {
                num *= 10;
                num += *val as usize - ZERO_VAL;
                was_adjacent = was_adjacent
                    || get_adjecent_positions(x, y)
                        .into_iter()
                        .filter(|&(x, y)| y < engine_scheme.len() && x < engine_scheme[0].len())
                        .any(|(x, y)| {
                            let is_dot_or_digit =
                                engine_scheme[y][x] == '.' || engine_scheme[y][x].is_numeric();
                            !is_dot_or_digit
                        });
            } else {
                if was_adjacent {
                    total_sum = total_sum.saturating_add(num);
                    was_adjacent = false;
                }
                num = 0;
            }
        }
        if was_adjacent {
            total_sum = total_sum.saturating_add(num);
        }
    }
    total_sum.to_string()
}

fn get_adjecent_positions(x: usize, y: usize) -> [(usize, usize); 8] {
    [
        (x + 1, y),
        (x + 1, y + 1),
        (x + 1, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x.saturating_sub(1), y + 1),
        (x.saturating_sub(1), y.saturating_sub(1)),
        (x, y + 1),
        (x, y.saturating_sub(1)),
    ]
}

#[test]
fn test_part2() {
    let input = include_str!("day_03/test_input.txt");
    assert_eq!(part2(input), "467835")
}

fn part2(input: &str) -> String {
    let engine_scheme: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut gear_data: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (y, row) in engine_scheme.iter().enumerate() {
        const ZERO_VAL: usize = '0' as usize;
        let mut gears: HashSet<(usize, usize)> = HashSet::new();
        let mut num: usize = 0;
        for (x, val) in row.iter().enumerate() {
            if val.is_ascii_digit() {
                num *= 10;
                num += *val as usize - ZERO_VAL;
                for gear_pos in get_adjecent_positions(x, y)
                    .into_iter()
                    .filter(|&(x, y)| y < engine_scheme.len() && x < engine_scheme[0].len())
                    .filter(|&(x, y)| engine_scheme[y][x] == '*')
                {
                    gears.insert(gear_pos);
                }
            } else {
                for gear in gears.drain() {
                    gear_data.entry(gear).or_default().push(num)
                }
                num = 0;
            }
        }
        for gear in gears.drain() {
            gear_data.entry(gear).or_default().push(num)
        }
    }
    gear_data
        .into_iter()
        .filter(|(_key, ratios)| ratios.len() == 2)
        .map(|(_k, ratios)| ratios.into_iter().product::<usize>())
        .sum::<usize>()
        .to_string()
}
