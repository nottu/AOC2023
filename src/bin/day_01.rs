fn main() {
    let input = include_str!("day_01/input.txt");
    let output = part1(input);
    dbg!(output);

    let input = include_str!("day_01/input.txt");
    let output = part2(input);
    dbg!(output);
}

#[test]
fn test_part1() {
    let input = include_str!("day_01/test_input.txt");
    let output = part1(input);
    assert_eq!(output, "142")
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(get_first_and_last_digit_as_num)
        .sum::<usize>()
        .to_string()
}

fn get_first_and_last_digit_as_num(input: &str) -> usize {
    let first_digit = input.chars().find(|n| n.is_ascii_digit()).unwrap() as usize - '0' as usize;
    let last_digit = input.chars().rfind(|n| n.is_ascii_digit()).unwrap() as usize - '0' as usize;
    first_digit * 10 + last_digit
}

#[test]
fn test_part_2() {
    let input = include_str!("day_01/test_input2.txt");
    let output = part2(input);
    assert_eq!(output, "281")
}

fn part2(input: &str) -> String {
    const NUMBERS: [&str; 20] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|num| get_first_and_last_digit_or_string_as_num(num, &NUMBERS))
        .sum::<usize>()
        .to_string()
}

fn get_first_and_last_digit_or_string_as_num(input: &str, nums: &[&str]) -> usize {
    let first_and_last =
        nums.iter()
            .enumerate()
            .fold((None, None), |(first_num, last_num), (idx, &num)| {
                let mut idx_pad = 0;
                let mut first_num: Option<(usize, usize)> = first_num;
                let mut last_num: Option<(usize, usize)> = last_num;
                while let Some(match_idx) = input[idx_pad..].find(num) {
                    let find_idx = match_idx + idx_pad;
                    let num_val = idx % 10;
                    if let Some((idx, _num)) = first_num {
                        if idx > find_idx {
                            first_num = Some((find_idx, num_val))
                        }
                    } else {
                        first_num = Some((find_idx, num_val))
                    }
                    if let Some((idx, _num)) = last_num {
                        if idx < find_idx {
                            last_num = Some((find_idx, num_val))
                        }
                    } else {
                        last_num = Some((find_idx, num_val))
                    }
                    idx_pad = find_idx + 1;
                }
                (first_num, last_num)
            });
    first_and_last.0.unwrap().1 * 10 + first_and_last.1.unwrap().1
}
