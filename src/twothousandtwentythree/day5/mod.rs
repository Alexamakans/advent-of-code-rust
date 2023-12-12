use std::ops::Range;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<u64> for Solver {
    fn part_one_driver(&self, input: &str) -> u64 {
        solve_part_one(input)
    }

    fn part_two_driver(&self, input: &str) -> u64 {
        solve_part_two(input)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 5)
    }
}

fn solve_part_one(s: &str) -> u64 {
    let processed_input = s.replace("\r", "");
    let mut sections = processed_input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|e| {
            println!("parsing '{}'", e);
            e.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    let seeds_to_soil = parse_map_section(sections.next().unwrap());
    let soil_to_fertilizer = parse_map_section(sections.next().unwrap());
    let fertilizer_to_water = parse_map_section(sections.next().unwrap());
    let water_to_light = parse_map_section(sections.next().unwrap());
    let light_to_temperature = parse_map_section(sections.next().unwrap());
    let temperature_to_humidity = parse_map_section(sections.next().unwrap());
    let humidity_to_location = parse_map_section(sections.next().unwrap());

    let mut best_location = u64::MAX;
    for seed in seeds {
        let soil = step(seed, &seeds_to_soil);
        let fert = step(soil, &soil_to_fertilizer);
        let water = step(fert, &fertilizer_to_water);
        let light = step(water, &water_to_light);
        let temp = step(light, &light_to_temperature);
        let hum = step(temp, &temperature_to_humidity);
        let loc = step(hum, &humidity_to_location);
        println!(
            "{} -> {} -> {} -> {} -> {} -> {} -> {} -> {}",
            seed, soil, fert, water, light, temp, hum, loc
        );

        if loc < best_location {
            best_location = loc;
        }
    }

    best_location
}
/**
 Amazingly, bruteforce worked for my input.
 Was trying to think of a better solution and it suddenly finished /shrug

 Took about 100 seconds on my machine.
*/
fn solve_part_two(s: &str) -> u64 {
    let processed_input = s.replace("\r", "");
    let mut sections = processed_input.split("\n\n");
    let seed_ranges = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|e| {
            println!("parsing '{}'", e);
            e.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    let mut seeds = Vec::new();
    for e in seed_ranges.chunks_exact(2) {
        for seed in e[0]..(e[0] + e[1]) {
            seeds.push(seed);
        }
    }

    let seeds_to_soil = parse_map_section(sections.next().unwrap());
    let soil_to_fertilizer = parse_map_section(sections.next().unwrap());
    let fertilizer_to_water = parse_map_section(sections.next().unwrap());
    let water_to_light = parse_map_section(sections.next().unwrap());
    let light_to_temperature = parse_map_section(sections.next().unwrap());
    let temperature_to_humidity = parse_map_section(sections.next().unwrap());
    let humidity_to_location = parse_map_section(sections.next().unwrap());

    let mut best_location = u64::MAX;
    for seed in seeds {
        let soil = step(seed, &seeds_to_soil);
        let fert = step(soil, &soil_to_fertilizer);
        let water = step(fert, &fertilizer_to_water);
        let light = step(water, &water_to_light);
        let temp = step(light, &light_to_temperature);
        let hum = step(temp, &temperature_to_humidity);
        let loc = step(hum, &humidity_to_location);

        if loc < best_location {
            best_location = loc;
        }
    }

    best_location
}
fn parse_map_section(section: &str) -> Vec<(Range<u64>, Range<u64>)> {
    let mut mapping = Vec::new();
    for line in section.lines().skip(1) {
        let mut parts = line.split_whitespace();
        let dst_start = parts.next().unwrap().parse::<u64>().unwrap();
        let src_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();
        mapping.push((
            dst_start..(dst_start + range_length),
            src_start..(src_start + range_length),
        ));
    }

    mapping
}

fn get(src_num: u64, src: &Range<u64>, dst: &Range<u64>) -> Option<u64> {
    if src.contains(&src_num) {
        let index = src_num - src.start;
        let res = dst.start + index;
        return Some(res);
    }

    None
}

fn step(src_num: u64, mapper: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    let mut found = false;
    let mut num = 0;
    for m in mapper.iter() {
        let soil = get(src_num, &m.1, &m.0);
        match soil {
            Some(v) => {
                num = v;
                found = true;
                break;
            }
            None => continue,
        }
    }

    if !found {
        num = src_num;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            35,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 486613012);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            46,
        )];
        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 56931769);
    }
}
