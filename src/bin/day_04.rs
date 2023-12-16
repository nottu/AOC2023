use std::collections::HashSet;

fn main() {
    let input = include_str!("day_04/input.txt");
    dbg!(part1(input));

    let input = include_str!("day_04/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day_04/test_input.txt");
    assert_eq!(part1(input), "13")
}

fn part1(input: &str) -> String {
    input.lines().map(parse_card).sum::<usize>().to_string()
}

#[test]
fn test_parse_card() {
    let value = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(value, 8)
}

fn parse_card(card: &str) -> usize {
    let num_matches = get_card_matches(card);
    match num_matches {
        0 => 0,
        1 => 1,
        n => 1 << (n - 1),
    }
}

fn get_card_matches(cards: &str) -> usize {
    let cards = cards
        .split(':')
        .nth(1)
        .expect("card should be formatted like 'Card <id> : <data>',  no ':' character found")
        .split('|')
        .collect::<Vec<&str>>();

    let right_side: HashSet<_> = cards[1]
        .trim()
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .collect();

    let left_side: HashSet<_> = cards[0]
        .trim()
        .split(' ')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .collect();

    left_side.intersection(&right_side).count()
}

#[test]
fn test_part2() {
    let input = include_str!("day_04/test_input.txt");
    assert_eq!(part2(input), "30")
}

fn part2(input: &str) -> String {
    let parse_cards: Vec<_> = input.lines().map(get_card_matches).collect();
    let mut card_counter = vec![0; parse_cards.len() + 1];
    card_counter[0] = 1;
    let mut count = 0;
    let mut total_card_count = 0;
    for (idx, score) in parse_cards.into_iter().enumerate() {
        count += card_counter[idx];
        total_card_count += count;
        card_counter[idx + 1] += count;
        card_counter[idx + score + 1] -= count;
    }
    total_card_count.to_string()
}
