fn main() {
    let input = include_str!("day_09/input.txt");
    dbg!(part1(input));

    let input = include_str!("day_09/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day_09/test_input.txt");
    assert_eq!(part1(input), "114");
}

fn part1(input: &str) -> String {
    let res = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i64>().expect("expected num"))
                .collect::<Vec<i64>>()
        })
        .map(|seq| extrapolate_sequence(&seq))
        .sum::<i64>();
    res.to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day_09/test_input.txt");
    assert_eq!(part2(input), "2")
}

fn part2(input: &str) -> String {
    let res = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i64>().expect("expected num"))
                .collect::<Vec<i64>>()
        })
        .map(|seq| extrapolate_prev_sequece(&seq))
        .sum::<i64>();
    res.to_string()
}

fn extrapolate_sequence(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|elem| *elem == 0) {
        return 0;
    }
    let mut next_steps = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        next_steps.push(sequence[i] - sequence[i - 1])
    }
    let extra = extrapolate_sequence(&next_steps);
    sequence.last().expect("expected a value") + extra
}

fn extrapolate_prev_sequece(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|elem| *elem == 0) {
        return 0;
    }
    let mut next_steps = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        next_steps.push(sequence[i] - sequence[i - 1])
    }
    let extra = extrapolate_prev_sequece(&next_steps);
    sequence.first().expect("expected a value") - extra
}

#[test]
fn test_extrapolate_sequence() {
    let values: Vec<i64> = (0..=5).map(|x| x * 3).collect();
    let next_val = extrapolate_sequence(&values);
    assert_eq!(next_val, 18);

    let prev_val = extrapolate_prev_sequece(&values);
    assert_eq!(prev_val, -3);
}

#[test]
fn test_extrapolate_seq_2() {
    let values = vec![10, 13, 16, 21, 30, 45];
    let pre_val = extrapolate_prev_sequece(&values);
    assert_eq!(pre_val, 5)
}
