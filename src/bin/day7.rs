fn main() {
    let input = include_str!("day7/input.txt");
    dbg!(part1(input));

    let input = include_str!("day7/input.txt");
    dbg!(part2(input));
}

type Hand = [u8; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    Single,
    Pair,
    Three,
    Four,
    Five,
}

impl HandType {
    fn new(count: usize) -> Self {
        match count {
            1 => Self::Single,
            2 => Self::Pair,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            n => {
                dbg!(n);
                panic!("you're not supposed to be here!")
            }
        }
    }
    fn next(self) -> Self {
        match self {
            HandType::Single => HandType::Pair,
            HandType::Pair => HandType::Three,
            HandType::Three => HandType::Four,
            HandType::Four => HandType::Five,
            HandType::Five => unreachable!(),
        }
    }
}

impl From<HandType> for usize {
    fn from(value: HandType) -> Self {
        match value {
            HandType::Single => 1,
            HandType::Pair => 2,
            HandType::Three => 3,
            HandType::Four => 4,
            HandType::Five => 5,
        }
    }
}

fn get_hand_types(hand: &Hand, use_jokers: bool) -> Vec<HandType> {
    println!("__________________");
    let mut counts = [0; 15];
    for card in hand {
        counts[*card as usize] += 1;
    }
    let mut hands: Vec<_> = counts
        .into_iter()
        .enumerate()
        .filter(|&(_idx, c)| c > 0)
        .map(|(idx, c)| (idx, HandType::new(c)))
        .collect();
    hands.sort_by_key(|(_idx, h)| h.to_owned());
    hands.reverse();

    let Some(jokers) = hands.iter().find(|(idx, _)| *idx == 0) else {
        return hands.into_iter().map(|(_idx, h)| h).collect();
    };
    if !use_jokers || hands.len() == 1 {
        return hands.into_iter().map(|(_idx, h)| h).collect();
    }

    let mut final_hands: Vec<_> = hands
        .iter()
        .filter(|(idx, _h)| *idx != 0)
        .map(|(_idx, h)| h.to_owned())
        .collect();
    let num_jokers: usize = jokers.1.into();
    dbg!(&hands, (jokers, num_jokers));
    for _ in 0..num_jokers {
        final_hands[0] = final_hands[0].next();
    }
    dbg!(final_hands)
    // final_hands
}

#[test]
fn day7_test_part1() {
    let input = include_str!("day7/test_input.txt");
    assert_eq!(part1(input), "6440")
}

fn part1(input: &str) -> String {
    let hands = parse_input(input);
    rank_hands(&hands, false)
        .into_iter()
        .sum::<u128>()
        .to_string()
}

fn rank_hands(hands: &[(Hand, u128)], use_jokers: bool) -> Vec<u128> {
    let mut hands: Vec<_> = hands
        .iter()
        .map(|(hand, bid)| (hand, get_hand_types(hand, use_jokers), bid))
        .collect();
    hands.sort_by(|h1, h2| {
        let (cards1, h1, _) = h1;
        let mut h1 = h1.iter();
        let (cards2, h2, _) = h2;
        let mut h2 = h2.iter();
        // println!("Comparing:\n\t{i} | Here h1: {h1:?}\t{j} | h2: {h2:?}");
        while let (Some(hand_type1), Some(hand_type2)) = (h1.next(), h2.next()) {
            if hand_type1 == hand_type2 {
                continue;
            }
            return hand_type1.cmp(hand_type2);
        }
        for (c1, c2) in cards1.iter().zip(cards2.iter()) {
            if c1 == c2 {
                continue;
            }
            return c1.cmp(c2);
        }
        println!("here h1: {h1:?}\th2: {h2:?}");
        std::cmp::Ordering::Equal
    });
    hands
        .iter()
        .enumerate()
        .map(|hand| dbg!(hand))
        .map(|(idx, hand_bid)| (idx as u128 + 1) * hand_bid.2)
        .collect()
}

fn parse_input(input: &str) -> Vec<(Hand, u128)> {
    input
        .lines()
        .map(|line| {
            let mut items = line.split(' ');
            let hand = parse_hand(items.next().unwrap());
            let bid = items.next().unwrap().parse::<u128>().unwrap();
            (hand, bid)
        })
        .collect()
}
fn parse_hand(input: &str) -> Hand {
    let mut hand = Hand::default();
    input
        .chars()
        .map(|c| match c {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            n => n as u8 - b'0',
        })
        .enumerate()
        .for_each(|(idx, n)| {
            hand[idx] = n;
        });
    hand
}

#[test]
fn day7_test_part2() {
    let input = include_str!("day7/test_input.txt");
    assert_eq!(part2(input), "5905")
}

fn part2(input: &str) -> String {
    let hands = parse_input(input);
    rank_hands(&hands, true)
        .into_iter()
        .sum::<u128>()
        .to_string()
}
