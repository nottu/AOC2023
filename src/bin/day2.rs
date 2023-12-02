fn main() {
    let input = include_str!("day2_input.txt");
    let output = part1(input);
    dbg!(output);

    let input = include_str!("day2_input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Default, PartialEq, Eq)]
struct SetBalls {
    red: u32,
    green: u32,
    blue: u32,
}
impl SetBalls {
    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    id: usize,
    sets: Vec<SetBalls>,
}
#[derive(Debug, Clone, Copy)]
struct GameParseError;

#[test]
fn day2_parse_game_test() {
    let game: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let parsed_game = Game::try_from(game);
    assert!(parsed_game.is_ok());
    let parsed_game = parsed_game.unwrap();
    let expected_game = Game {
        id: 1,
        sets: vec![
            SetBalls {
                blue: 3,
                red: 4,
                ..Default::default()
            },
            SetBalls {
                red: 1,
                green: 2,
                blue: 6,
            },
            SetBalls {
                green: 2,
                ..Default::default()
            },
        ],
    };
    assert_eq!(expected_game, parsed_game);
}

impl TryFrom<&str> for Game {
    type Error = GameParseError;
    fn try_from(game_str: &str) -> Result<Self, Self::Error> {
        let mut game_str_parts = game_str.split(':');
        let id = game_str_parts
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let Some(game_str) = game_str_parts.next() else {
            eprintln!("Failed to Split game from sets");
            return Err(GameParseError);
        };
        let sets: Vec<_> = game_str
            .split(';')
            .map(SetBalls::try_from)
            // can we do with out unwrap?
            .map(|s| s.unwrap())
            .collect();
        Ok(Self { id, sets })
    }
}

impl TryFrom<&str> for SetBalls {
    type Error = GameParseError;
    fn try_from(sets_str: &str) -> Result<Self, Self::Error> {
        let count_and_color: Vec<_> = sets_str
            .trim()
            .split(',')
            .map(|count_and_color| {
                let count_and_color: Vec<_> = count_and_color.trim().split(' ').collect();
                // todo: expect panics! replace with returning an Error
                let count: u32 = count_and_color[0].parse().expect("Expected Number");
                (count, count_and_color[1])
            })
            .collect();
        let mut balls = Self::default();
        for (count, color) in count_and_color {
            match color {
                "red" => balls.red = count,
                "green" => balls.green = count,
                "blue" => balls.blue = count,
                _ => return Err(GameParseError),
            }
        }
        Ok(balls)
    }
}

#[test]
fn day2_classify_valid_tests() {
    let input = include_str!("day2_test_input.txt");
    let expected_validities: [bool; 5] = [true, true, false, false, true];
    input
        .lines()
        .map(Game::try_from)
        .map(|game| game.unwrap().is_valid())
        .zip(expected_validities)
        .enumerate()
        .for_each(|(idx, (validity, expected_validity))| {
            // dbg!((validity, expected_validity));
            assert_eq!(validity, expected_validity, "Game {idx}");
        });
}

impl Game {
    fn is_valid(&self) -> bool {
        // go with hard coded rules for now...
        const MAX_RED: u32 = 12;
        const MAX_GREEN: u32 = 13;
        const MAX_BLUE: u32 = 14;
        self.sets.iter().all(|set_balls| {
            set_balls.red <= MAX_RED && set_balls.green <= MAX_GREEN && set_balls.blue <= MAX_BLUE
        })
    }
    fn compute_min_available_balls(&self) -> SetBalls {
        self.sets
            .iter()
            .fold(SetBalls::default(), |min_balls, curr_set| SetBalls {
                blue: min_balls.blue.max(curr_set.blue),
                red: min_balls.red.max(curr_set.red),
                green: min_balls.green.max(curr_set.green),
            })
    }
}

#[test]
fn day2_test_part1() {
    let input = include_str!("day2_test_input.txt");
    assert_eq!(part1(input), "8")
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(Game::try_from)
        .filter_map(|game| game.ok())
        .filter(Game::is_valid)
        .map(|game| game.id)
        .sum::<usize>()
        .to_string()
}

#[test]
fn day2_test_part2() {
    let input = include_str!("day2_test_input.txt");
    assert_eq!(part2(input), "2286")
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(Game::try_from)
        .filter_map(|game| game.ok())
        .map(|g| g.compute_min_available_balls())
        .map(|s| s.power() as usize)
        .sum::<usize>()
        .to_string()
}
