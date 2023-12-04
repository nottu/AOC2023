use std::collections::HashMap;

fn main() {
    let input = include_str!("day4/input.txt");
    dbg!(part1(input));

    let input = include_str!("day4/input.txt");
    dbg!(part2(input));
}

#[test]
fn day4_test_part1() {
    let input = include_str!("day4/test_input.txt");
    assert_eq!(part1(input), "13")
}

fn part1(input: &str) -> String {
    input.lines().map(parse_card).sum::<usize>().to_string()
}

#[test]
fn day4_parse_card() {
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
    let cards = cards.split(':').nth(1)
        .expect("card should be formatted like 'Card <id> : <data>',  no ':' character found");
        .split('|').collect::<Vec<&str>>();

    let right_side: HashSet<&str> = cards.get(1)
        .expect("card should have two sides separated by '|', no right side found")
        .trim()
        .split(' ').map(|num| num.trim()).collect;

    cards.get(0)
        .expect("card should have two sides separated by '|', no left side found")
        .trim()
        .split(' ')
        .filter(|number| right_side.contins(number.trim()))
        .count()
}

#[test]
fn day4_test_part2() {
    let input = include_str!("day4/test_input.txt");
    assert_eq!(part2(input), "30")
}

fn part2(input: &str) -> String {
    let parse_cards: Vec<_> = input.lines().map(get_card_matches).collect();
    let mut parsed_cards_counts = vec![1; parse_cards.len()];
    for (idx, score) in parse_cards.into_iter().enumerate() {
        for count_idx in 1..=score {
            parsed_cards_counts[idx + count_idx] += parsed_cards_counts[idx];
        }
    }
    parsed_cards_counts.iter().sum::<usize>().to_string()
}
