fn main() {
    let input = include_str!("day15/input.txt");
    dbg!(part1(input));
    let input = include_str!("day15/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day15/test_input.txt");
    assert_eq!(part1(input), "1320");
}

fn part1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(hash_str)
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day15/test_input.txt");
    assert_eq!(part2(input), "145");
}

fn part2(input: &str) -> String {
    let steps = parse_steps(input);
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for step in steps {
        let box_idx = hash_str(&step.label);
        let curr_box = &mut boxes[box_idx];
        match step.operation {
            Operation::Remove => {
                let mut item_idx = None;
                for (idx, curr_box_item) in curr_box.iter().enumerate() {
                    if curr_box_item.0 == step.label {
                        item_idx = Some(idx);
                        break;
                    }
                }
                if let Some(item_idx) = item_idx {
                    curr_box.remove(item_idx);
                }
            }
            Operation::Equal(focal_len) => {
                let mut item_idx = None;
                for (idx, curr_box_item) in curr_box.iter().enumerate() {
                    if curr_box_item.0 == step.label {
                        item_idx = Some(idx);
                        break;
                    }
                }
                if let Some(item_idx) = item_idx {
                    curr_box[item_idx].1 = focal_len;
                } else {
                    curr_box.push((step.label, focal_len))
                }
            }
        }
    }

    let mut power = 0;
    for (idx, box_item) in boxes.into_iter().enumerate() {
        if box_item.is_empty() {
            continue;
        }
        println!("{idx} | {box_item:?}");
        for (item_idx, item) in box_item.into_iter().enumerate() {
            power += (idx + 1) * item.1 * (item_idx + 1);
        }
    }
    power.to_string()
}

fn hash_str(input: &str) -> usize {
    input.chars().fold(0, |acc, c| {
        let hash = acc + (c as usize);
        let hash = hash * 17;
        hash % 256
    })
}

#[test]
fn test_hash() {
    assert_eq!(hash_str("HASH"), 52)
}

#[derive(Debug, PartialEq, Eq)]
struct Step {
    label: String,
    operation: Operation,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Equal(usize),
    Remove,
}

fn parse_steps(input: &str) -> Vec<Step> {
    input.trim().split(',').map(parse_step).collect()
}

fn parse_step(step: &str) -> Step {
    if step.contains('=') {
        let mut data = step.split('=');
        let label = data.next().expect("expected label").to_string();
        let focal = data
            .next()
            .expect("expected focal")
            .parse::<usize>()
            .expect("expected usize");
        Step {
            label,
            operation: Operation::Equal(focal),
        }
    } else {
        let mut data = step.split('-');
        let label = data.next().expect("s").to_string();
        Step {
            label,
            operation: Operation::Remove,
        }
    }
}
