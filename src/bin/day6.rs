fn main() {
    let input = include_str!("day6/input.txt");
    dbg!(part1(input));

    let input = include_str!("day6/input.txt");
    dbg!(part2(input));
}

#[test]
fn day6_test_part1() {
    let input = include_str!("day6/test_input.txt");
    assert_eq!(part1(input), "288")
}

struct RaceData {
    time: i64,
    distance: i64,
}

fn parse_input(input: &str) -> Vec<RaceData> {
    let input: Vec<&str> = input.lines().collect();
    let times = input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|times| times.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());
    let distance = input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|times| times.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());
    times
        .zip(distance)
        .map(|(time, distance)| RaceData { time, distance })
        .collect()
}

fn part1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|race_data| win_ways(race_data.time, race_data.distance))
        .fold(0, |acc, x| {
            dbg!(x);
            if acc == 0 {
                x
            } else if x == 0 {
                acc
            } else {
                acc * x
            }
        })
        .to_string()
}

#[test]
fn day6_test_win_ways() {
    let win_ways = win_ways(7, 9);
    assert_eq!(win_ways, 4);
}

fn win_ways(time: i64, distance: i64) -> i64 {
    // (time - x) * x - distance = -x^2 + x*time - ditance
    // same as solving := x^2 - x * time + distance
    // a = 1
    // b = -time
    // c = distance
    let time = time as f64;
    // add a small epsilon to ensure we get a larger distance
    let distance = distance as f64 + 0.000000000001;
    let sq_pt = ((time * time) - (4.0 * distance)).sqrt();
    let left_part = ((time - sq_pt) / 2.0).ceil() as i64;
    let right_part = ((time + sq_pt) / 2.0).floor() as i64;
    dbg!(left_part, right_part);
    (left_part..=right_part).count() as i64
}

#[test]
fn day6_test_part2() {
    let input = include_str!("day6/test_input.txt");
    assert_eq!(part2(input), "71503")
}

fn part2(input: &str) -> String {
    let race_data = parse_input2(input);
    win_ways(race_data.time, race_data.distance).to_string()
}

fn parse_input2(input: &str) -> RaceData {
    let input: Vec<&str> = input.lines().collect();
    let time = input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|times| times.trim())
        .filter(|s| !s.is_empty())
        .collect::<String>();
    let distance = input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|times| times.trim())
        .filter(|s| !s.is_empty())
        .collect::<String>();
    dbg!(&time, &distance);
    RaceData {
        time: time.parse::<i64>().unwrap(),
        distance: distance.parse::<i64>().unwrap(),
    }
}
