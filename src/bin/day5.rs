fn main() {
    let input = include_str!("day5/input.txt");
    dbg!(part1(input));

    let input = include_str!("day5/input.txt");
    dbg!(part2(input));
}

#[test]
fn day5_test_part1() {
    let input = include_str!("day5/test_input.txt");
    assert_eq!(part1(input), "35")
}

fn part1(input: &str) -> String {
    let mut input_lines = input.lines();

    let seeds: Vec<u32> = input_lines
        .next()
        .expect("at least one line")
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|num| num.parse().unwrap())
        .collect();
    let maps = {
        input_lines.next();
        let lines: Vec<&str> = input_lines.collect();
        let mut idx = 1;
        let seed_soil_map = read_map(&lines[idx..]);
        idx += 2 + seed_soil_map.len();
        let soil_fert_map = read_map(&lines[idx..]);
        idx += 2 + soil_fert_map.len();
        let fert_water_map = read_map(&lines[idx..]);
        idx += 2 + fert_water_map.len();
        let water_light_map = read_map(&lines[idx..]);
        idx += 2 + water_light_map.len();
        let light_temp_map = read_map(&lines[idx..]);
        idx += 2 + light_temp_map.len();
        let temp_humidity_map = read_map(&lines[idx..]);
        idx += 2 + temp_humidity_map.len();
        let humidity_location_map = read_map(&lines[idx..]);

        [
            seed_soil_map,
            soil_fert_map,
            fert_water_map,
            water_light_map,
            light_temp_map,
            temp_humidity_map,
            humidity_location_map,
        ]
    };

    seeds
        .into_iter()
        .map(|seed| {
            // println!("_______________");
            maps.iter().fold(seed, |item, map_rels| {
                // dbg!(map_rels, item);
                let pp = map_rels.partition_point(|m| m.from <= item);
                if pp == 0 {
                    item
                } else {
                    let pp = pp - 1;
                    // dbg!(map_rels, pp);
                    let diff = item - map_rels[pp].from;
                    
                    // dbg!(item, mapped);
                    if diff + 1 > map_rels[pp].range {
                        item
                    } else {
                        diff + map_rels[pp].to
                    }
                }
            })
        })
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct MapRel {
    to: u32,
    from: u32,
    range: u32,
}

fn read_map(lines: &[&str]) -> Vec<MapRel> {
    let mut compressed_map = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }
        let nums: Vec<_> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        compressed_map.push(MapRel {
            to: nums[0],
            from: nums[1],
            range: nums[2],
        });
    }
    compressed_map.sort_by_key(|m| m.from);
    compressed_map
}

#[test]
fn day5_test_part2() {
    let input = include_str!("day5/test_input.txt");
    assert_eq!(part2(input), "46")
}

fn part2(input: &str) -> String {
    let mut input_lines = input.lines();

    let seeds: Vec<u32> = input_lines
        .next()
        .expect("at least one line")
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|num| num.parse().unwrap())
        .collect();
    let maps = {
        input_lines.next();
        let lines: Vec<&str> = input_lines.collect();
        let mut idx = 1;
        let seed_soil_map = read_map(&lines[idx..]);
        idx += 2 + seed_soil_map.len();
        let soil_fert_map = read_map(&lines[idx..]);
        idx += 2 + soil_fert_map.len();
        let fert_water_map = read_map(&lines[idx..]);
        idx += 2 + fert_water_map.len();
        let water_light_map = read_map(&lines[idx..]);
        idx += 2 + water_light_map.len();
        let light_temp_map = read_map(&lines[idx..]);
        idx += 2 + light_temp_map.len();
        let temp_humidity_map = read_map(&lines[idx..]);
        idx += 2 + temp_humidity_map.len();
        let humidity_location_map = read_map(&lines[idx..]);

        [
            seed_soil_map,
            soil_fert_map,
            fert_water_map,
            water_light_map,
            light_temp_map,
            temp_humidity_map,
            humidity_location_map,
        ]
    };

    let mut curr_min = u32::MAX;
    for idx in 0..seeds.len() / 2 {
        let idx = 2 * idx;
        let seed = seeds[idx];
        let range = seeds[idx + 1];
        curr_min = curr_min.min(
            (0..range)
                .map(|i| seed + i)
                .map(|seed| {
                    // println!("_______________");
                    maps.iter().fold(seed, |item, map_rels| {
                        // dbg!(map_rels, item);
                        let pp = map_rels.partition_point(|m| m.from <= item);
                        if pp == 0 {
                            item
                        } else {
                            let pp = pp - 1;
                            // dbg!(map_rels, pp);
                            let diff = item - map_rels[pp].from;
                            
                            // dbg!(item, mapped);
                            if diff + 1 > map_rels[pp].range {
                                item
                            } else {
                                diff + map_rels[pp].to
                            }
                        }
                    })
                })
                .min()
                .unwrap(),
        );
    }
    curr_min.to_string()
}
