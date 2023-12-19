use std::{collections::HashMap, str::Lines};

fn main() {
    let input = include_str!("day_19/input.txt");
    dbg!(part1(input));
    let input = include_str!("day_19/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day_19/test_input.txt");
    assert_eq!(part1(input), "19114");
}

fn part1(input: &str) -> String {
    let (workflow_map, parts) = parse_input(input);
    parts
        .into_iter()
        .filter(|part| process_part(part, &workflow_map))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<i64>()
        .to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day_19/test_input.txt");
    assert_eq!(part2(input), "167409079868000");
}

fn part2(input: &str) -> String {
    let (workflow_map, _parts) = parse_input(input);
    let part_range = PartRange {
        x: Range {
            start: 1,
            end: 4001,
        },
        m: Range {
            start: 1,
            end: 4001,
        },
        a: Range {
            start: 1,
            end: 4001,
        },
        s: Range {
            start: 1,
            end: 4001,
        },
    };
    analyze_workflows(part_range, &workflow_map).to_string()
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}
impl Range {
    fn len(&self) -> i64 {
        if self.start > self.end {
            println!("HERE!");
            0
        } else {
            self.end - self.start
        }
    }
    const EMPTY: Self = Self { start: 0, end: 0 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}
impl PartRange {
    fn len(&self) -> i64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
    const EMPTY: Self = {
        Self {
            x: Range::EMPTY,
            m: Range::EMPTY,
            a: Range::EMPTY,
            s: Range::EMPTY,
        }
    };
}
struct Workflow {
    rules: Vec<String>,
}

impl Workflow {
    fn process_part(&self, part: &Part) -> String {
        for rule in &self.rules {
            if let Some(label) = Self::process_rule(rule, part) {
                return label;
            }
        }
        panic!("ran out of rules!")
    }
    fn process_rule(rule: &str, part: &Part) -> Option<String> {
        // hacky way to check if last part
        if rule.ends_with('}') {
            let mut chunk_chars = rule.chars();
            chunk_chars.next_back();
            let to_label = chunk_chars.as_str().to_string();
            return Some(to_label);
        }
        let mut rule_chunks = rule.split(':');
        let condition = rule_chunks.next().expect("expected condition");
        let to_label = rule_chunks.next().expect("expected label").to_string();

        let mut condition_chars = condition.chars();
        let part_target = condition_chars.next().expect("exp");
        let condition_condition = condition_chars.next().expect("< || >");
        let condition_value: i64 = condition_chars.as_str().parse().expect("target");

        let part_value = match part_target {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };
        match condition_condition {
            '>' => {
                if part_value > condition_value {
                    return Some(to_label);
                }
            }
            '<' => {
                if part_value < condition_value {
                    return Some(to_label);
                }
            }
            _ => unreachable!(),
        }
        None
    }
    fn analyze_part_range(&self, part_range: PartRange) -> Vec<(String, PartRange)> {
        let mut out = vec![];
        let mut curr_range = part_range;
        for rule in &self.rules {
            if curr_range.len() == 0 {
                break;
            }
            let rule_analysis = Self::analyze_rule(rule, curr_range);
            let pass_analysis = rule_analysis.0;
            out.push((pass_analysis.0.unwrap(), pass_analysis.1));
            curr_range = rule_analysis.1 .1;
        }
        out
    }
    fn analyze_rule(
        rule: &str,
        part_range: PartRange,
    ) -> ((Option<String>, PartRange), (Option<String>, PartRange)) {
        // hacky way to check if last part
        if rule.ends_with('}') {
            let mut chunk_chars = rule.chars();
            chunk_chars.next_back();
            let to_label = chunk_chars.as_str().to_string();
            return ((Some(to_label), part_range), (None, PartRange::EMPTY));
        }
        let mut rule_chunks = rule.split(':');
        let condition = rule_chunks.next().expect("expected condition");
        let to_label = rule_chunks.next().expect("expected label").to_string();

        let mut condition_chars = condition.chars();
        let part_target = condition_chars.next().expect("exp");
        let condition_condition = condition_chars.next().expect("< || >");
        let condition_value: i64 = condition_chars.as_str().parse().expect("target");

        match part_target {
            'x' => {
                let x_ranges =
                    Self::divide_ranges(part_range.x, condition_condition, condition_value);
                let mut pass = part_range.to_owned();
                pass.x = x_ranges[0];
                let mut no_pass = part_range.to_owned();
                no_pass.x = x_ranges[1];
                ((Some(to_label), pass), (None, no_pass))
            }
            'm' => {
                let m_ranges =
                    Self::divide_ranges(part_range.m, condition_condition, condition_value);
                let mut pass = part_range.to_owned();
                pass.m = m_ranges[0];
                let mut no_pass = part_range.to_owned();
                no_pass.m = m_ranges[1];
                ((Some(to_label), pass), (None, no_pass))
            }
            'a' => {
                let a_ranges =
                    Self::divide_ranges(part_range.a, condition_condition, condition_value);
                let mut pass = part_range.to_owned();
                pass.a = a_ranges[0];
                let mut no_pass = part_range.to_owned();
                no_pass.a = a_ranges[1];
                ((Some(to_label), pass), (None, no_pass))
            }
            's' => {
                let s_ranges =
                    Self::divide_ranges(part_range.s, condition_condition, condition_value);
                let mut pass = part_range.to_owned();
                pass.s = s_ranges[0];
                let mut no_pass = part_range.to_owned();
                no_pass.s = s_ranges[1];
                ((Some(to_label), pass), (None, no_pass))
            }
            _ => unreachable!(),
        }
    }
    fn divide_ranges(range: Range, condition: char, value: i64) -> [Range; 2] {
        // println!("dividing range := {range:?} {condition} {value} ");
        match condition {
            // (1000..1801) > 838 => (0,0), (1000..1801)
            '>' => [
                Range {
                    start: range.start.max(value + 1),
                    end: range.end,
                },
                Range {
                    start: range.start,
                    end: range.end.min(value + 1),
                },
                //167393896388196
            ],
            // (0..1801) < 838 => (0..838), (838..1801)
            '<' => [
                Range {
                    start: range.start,
                    end: value.min(range.end),
                },
                Range {
                    start: value.max(range.start),
                    end: range.end,
                },
            ],
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut lines = input.lines();
    let workflow_map = parse_workflows(&mut lines);
    let parts = parse_parts(lines);
    (workflow_map, parts)
}

fn parse_workflows(input: &mut Lines) -> HashMap<String, Workflow> {
    let mut map = HashMap::new();
    for workflow in input.by_ref() {
        if workflow.is_empty() {
            break;
        }
        let mut workflow_chunks = workflow.split('{');
        let label = workflow_chunks.next().expect("expected workflow label");
        let rules: Vec<_> = workflow_chunks
            .next()
            .expect("expected rule")
            .split(',')
            .map(|r| r.to_string())
            .collect();
        map.insert(label.to_string(), Workflow { rules });
    }
    map
}

fn parse_parts(input: Lines) -> Vec<Part> {
    input
        .map(|part_data| {
            let mut part_data = part_data.chars();
            part_data.next();
            part_data.next_back();
            let part_data = part_data.as_str();
            // dbg!(part_data);
            let mut part_chunks = part_data.split(',');
            let x = part_chunks
                .next()
                .expect("expected x chunk")
                .split('=')
                .nth(1)
                .expect("expected x value")
                .parse::<i64>()
                .expect("expected x number");
            let m = part_chunks
                .next()
                .expect("expected m chunk")
                .split('=')
                .nth(1)
                .expect("expected m value")
                .parse::<i64>()
                .expect("expected m number");
            let a = part_chunks
                .next()
                .expect("expected a chunk")
                .split('=')
                .nth(1)
                .expect("expected a value")
                .parse::<i64>()
                .expect("expected a number");
            let s = part_chunks
                .next()
                .expect("expected s chunk")
                .split('=')
                .nth(1)
                .expect("expected s value")
                .parse::<i64>()
                .expect("expected s number");
            Part { x, m, a, s }
        })
        .collect()
}

fn process_part(part: &Part, workflow_map: &HashMap<String, Workflow>) -> bool {
    let mut to_label = "in".to_string();

    loop {
        if to_label == "R" {
            return false;
        }
        if to_label == "A" {
            return true;
        }
        let curr_workflow = workflow_map
            .get(&to_label)
            .expect("expected to find rule for label");
        to_label = curr_workflow.process_part(part);
    }
}

fn analyze_workflows(part_range: PartRange, workflow_map: &HashMap<String, Workflow>) -> i64 {
    let mut to_visit = vec![("in".to_string(), part_range)];
    let mut accepted_sum = 0;
    while let Some((to_label, part_range)) = to_visit.pop() {
        // println!("-----------------------------");
        // println!("{to_label} {part_range:?}");
        if to_label == "R" {
            continue;
        }
        if to_label == "A" {
            accepted_sum += part_range.len();
            // println!("{part_range:?}");
            continue;
        }
        let curr_flow = workflow_map.get(&to_label).expect("expected rule");
        let mut next = curr_flow.analyze_part_range(part_range);
        // println!("{next:?}");
        to_visit.append(&mut next);
        // println!("{to_visit:?}");
        // break;
    }
    accepted_sum
}
