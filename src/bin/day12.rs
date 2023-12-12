use std::collections::HashMap;

fn main() {
    let input = include_str!("day12/input.txt");
    dbg!(part1(input));
    let input = include_str!("day12/input.txt");
    dbg!(part2(input));
}

#[test]
fn day12_test_part1() {
    let input = include_str!("day12/test_input.txt");
    assert_eq!(part1(input), "21");
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(find_num_arrangements)
        .sum::<usize>()
        .to_string()
}

#[test]
fn day12_test_part2() {
    let input = include_str!("day12/test_input.txt");
    assert_eq!(part2(input), "525152");
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|row| find_num_arrangements_repeated(row, 5))
        .sum::<usize>()
        .to_string()
}

fn find_num_arrangements(row: &str) -> usize {
    let mut data = row.split(' ');

    let springs: Vec<SpringState> = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("undefined spring state"),
        })
        .collect();
    let damaged_list: Vec<usize> = data
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().expect("expected number"))
        .collect();
    let mut memo = HashMap::new();
    find_num_arrangements_recursive(&springs, &damaged_list, &mut memo)
}

fn find_num_arrangements_repeated(row: &str, repetitions: usize) -> usize {
    // dbg!(row);
    let mut data = row.split(' ');

    let mut springs: Vec<SpringState> = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("undefined spring state"),
        })
        .collect();

    springs.push(SpringState::Unknown);
    springs = springs.repeat(repetitions);
    springs.pop();

    let damaged_list: Vec<usize> = data
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().expect("expected number"))
        .collect();

    let mut memo = HashMap::new();
    find_num_arrangements_recursive(&springs, &damaged_list.repeat(repetitions), &mut memo)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn find_num_arrangements_recursive(
    mut springs: &[SpringState],
    damaged: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(val) = memo.get(&(springs.len(), damaged.len())) {
        return *val;
    }
    if damaged.is_empty() {
        return if springs.iter().any(|spring| *spring == SpringState::Damaged) {
            0
        } else {
            1
        };
    }
    // find either first broken spring or unkown
    while let Some(spring_state) = springs.first() {
        if *spring_state == SpringState::Operational {
            springs = &springs[1..];
        } else {
            break;
        }
    }
    if springs.is_empty() {
        return 0;
    }
    let curr_damaged_count = damaged[0];
    // we either take it as damaged or skip

    // skip first
    let num_arrangements_no_take = if springs[0] == SpringState::Unknown {
        find_num_arrangements_recursive(&springs[1..], damaged, memo)
    } else {
        // cant skip, so 0 ways
        0
    };
    let num_arrangements_take = {
        let has_operational = springs
            .iter()
            .take(curr_damaged_count)
            .any(|spring_state| *spring_state == SpringState::Operational);
        if has_operational
            || springs.len() < curr_damaged_count
            || springs.get(curr_damaged_count) == Some(&SpringState::Damaged)
        {
            0
        } else {
            let damaged = &damaged[1..];
            let springs = if 1 + curr_damaged_count > springs.len() {
                &[]
            } else {
                &springs[(1 + curr_damaged_count)..]
            };
            find_num_arrangements_recursive(springs, damaged, memo)
        }
    };
    let total = num_arrangements_take + num_arrangements_no_take;
    memo.insert((springs.len(), damaged.len()), total);
    total
}
