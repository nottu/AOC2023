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
    let cards = cards.split(':').nth(1).unwrap();
    let cards = cards.split('|').collect::<Vec<&str>>();

    // we assume card numbers can repeat themselves...
    let mut right_side: HashMap<&str, usize> = HashMap::new();
    for num in cards[1].trim().split(' ') {
        let num = num.trim();
        *right_side.entry(num).or_default() += 1;
    }

    cards[0]
        .trim()
        .split(' ')
        .filter(|number| {
            let curr_count = right_side.entry(number.trim()).or_default();
            if number.is_empty() || *curr_count == 0 {
                false
            } else {
                *curr_count -= 1;
                true
            }
        })
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
