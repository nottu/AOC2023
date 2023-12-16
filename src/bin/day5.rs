use std::str::Lines;

fn main() {
    let input = include_str!("day5/input.txt");
    dbg!(part1(input));

    let input = include_str!("day5/input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part1() {
    let input = include_str!("day5/test_input.txt");
    assert_eq!(part1(input), "35")
}

fn part1(input: &str) -> String {
    let mut input_lines = input.lines();

    let seeds = parse_seeds(input_lines.next().expect("expected seed data"));
    let almanac = parse_almanac(&mut input_lines);

    let res = seeds
        .into_iter()
        .map(|seed| almanac.seed_soil_map.map(seed))
        .map(|soil| almanac.soil_fert_map.map(soil))
        .map(|fert| almanac.fert_water_map.map(fert))
        .map(|water| almanac.water_light_map.map(water))
        .map(|light| almanac.light_temp_map.map(light))
        .map(|temp| almanac.temp_humidity_map.map(temp))
        .map(|humidity| almanac.humidity_location_map.map(humidity))
        .min()
        .expect("Expected a result!");
    res.to_string()
}

#[test]
fn test_part2() {
    let input = include_str!("day5/test_input.txt");
    assert_eq!(part2(input), "46")
}
fn part2(input: &str) -> String {
    let mut input_lines = input.lines();

    let seeds = parse_seed_ranges(input_lines.next().expect("expected seed data"));
    let almanac = parse_almanac(&mut input_lines);
    let soil: Vec<_> = seeds
        .into_iter()
        .flat_map(|seed_range| almanac.seed_soil_map.map_range(seed_range))
        .flat_map(|soil| almanac.soil_fert_map.map_range(soil))
        .flat_map(|fert| almanac.fert_water_map.map_range(fert))
        .flat_map(|water| almanac.water_light_map.map_range(water))
        .flat_map(|light| almanac.light_temp_map.map_range(light))
        .flat_map(|temp| almanac.temp_humidity_map.map_range(temp))
        .flat_map(|humidity| almanac.humidity_location_map.map_range(humidity))
        .collect();
    soil.into_iter()
        .map(|range| range.start)
        .min()
        .expect("expected result")
        .to_string()
}

#[derive(Debug)]
struct MapRel {
    to: u64,
    from: u64,
    range: u64,
}

#[derive(Debug, Default)]
struct RangeMap {
    inner_map: Vec<MapRel>,
}
#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}
impl Range {
    fn len(&self) -> u64 {
        self.end - self.start
    }
}

/// Given an input like `"79 14 55 13"` produces a
/// `Vector<Range>` with `[Range{start: 79, end: 79 + 14}, Range{ start: 55, end: 55+13}]`
fn parse_seed_ranges(input: &str) -> Vec<Range> {
    input
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("expected num"))
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|ch| Range {
            start: ch[0],
            end: ch[0] + ch[1],
        })
        .collect()
}

impl RangeMap {
    pub fn new(mut relations: Vec<MapRel>) -> Self {
        relations.sort_by_key(|rel| rel.from);
        Self {
            inner_map: relations,
        }
    }
    pub fn map(&self, item: u64) -> u64 {
        let partition_idx = self.inner_map.partition_point(|m| m.from <= item);
        if partition_idx == 0 {
            item
        } else {
            let pp = partition_idx - 1;
            // item is guaranteed to be greater than `from`
            let needed_range = item - self.inner_map[pp].from;
            if needed_range + 1 > self.inner_map[pp].range {
                item
            } else {
                needed_range + self.inner_map[pp].to
            }
        }
    }
    pub fn map_range(&self, input_range: Range) -> Vec<Range> {
        let partition_idx = self
            .inner_map
            .partition_point(|m| m.from <= input_range.start);
        if partition_idx == 0 {
            return vec![input_range];
        }
        let start_index = partition_idx - 1;
        let map_rel = &self.inner_map[start_index];
        let needed_range = input_range.len();
        let range_start_diff = input_range.start - map_rel.from;
        let available_range = map_rel.range.saturating_sub(range_start_diff);
        if available_range == 0 {
            return vec![input_range];
        }
        let mapped_start = map_rel.to + (input_range.start - map_rel.from);
        let mapped_range = Range {
            start: mapped_start,
            end: mapped_start + available_range.min(needed_range),
        };
        let mut out_map = vec![mapped_range];
        if available_range < needed_range {
            let unmapped_range = needed_range - available_range;
            let start = input_range.start + available_range;
            let end = start + unmapped_range;
            let new_range = Range { start, end };
            let mut sub_map = self.map_range(new_range);
            out_map.append(&mut sub_map);
        }
        // we could compact the ranges
        // (0..4),(4..7) => (0..7)
        // but this is fast enough for the given input
        out_map
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .split(':')
        .nth(1)
        .expect("expected seeds")
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|num| {
            num.parse()
                .unwrap_or_else(|_| panic!("expected a number found: {num}"))
        })
        .collect()
}

struct Almanac {
    pub seed_soil_map: RangeMap,
    pub soil_fert_map: RangeMap,
    pub fert_water_map: RangeMap,
    pub water_light_map: RangeMap,
    pub light_temp_map: RangeMap,
    pub temp_humidity_map: RangeMap,
    pub humidity_location_map: RangeMap,
}

fn parse_almanac(input_lines: &mut Lines) -> Almanac {
    // empty line
    input_lines.next();
    // seed-to-soil map:
    input_lines.next();
    let seed_soil_map = read_map(input_lines);
    input_lines.next();
    let soil_fert_map = read_map(input_lines);
    input_lines.next();
    let fert_water_map = read_map(input_lines);
    input_lines.next();
    let water_light_map = read_map(input_lines);
    input_lines.next();
    let light_temp_map = read_map(input_lines);
    input_lines.next();
    let temp_humidity_map = read_map(input_lines);
    input_lines.next();
    let humidity_location_map = read_map(input_lines);
    Almanac {
        seed_soil_map,
        soil_fert_map,
        fert_water_map,
        water_light_map,
        light_temp_map,
        temp_humidity_map,
        humidity_location_map,
    }
}

fn read_map(input_lines: &mut Lines) -> RangeMap {
    let mut compressed_map = vec![];
    for line in input_lines.by_ref() {
        if line.is_empty() {
            break;
        }
        // println!("{}", line);
        let nums: Vec<_> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        compressed_map.push(MapRel {
            to: nums[0],
            from: nums[1],
            range: nums[2],
        });
    }
    RangeMap::new(compressed_map)
}
