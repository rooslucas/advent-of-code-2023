use rayon::prelude::*;
use std::fs::read_to_string;
use std::u64;

struct Range {
    start_destination: u64,
    start_source: u64,
    length: u64,
}
impl Range {
    fn range_check(&self, i: u64) -> u64 {
        let end_source = self.start_source + self.length;

        if (i >= self.start_source) && (i < end_source) {
            let diff = i - self.start_source;
            return self.start_destination + diff;
        } else {
            return 0;
        }
    }
}

struct Map {
    name: String,
    ranges: Vec<Range>,
}

impl Map {
    fn check_ranges(&self, value: u64) -> u64 {
        let sum = (&self.ranges)
            .iter()
            .map(|range| range.range_check(value))
            .sum();

        if sum > 0 {
            return sum;
        } else {
            return value;
        }
    }
}

fn main() {
    let file_path = "./input/day_05.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let parsed = parse(liness);

    part_1(&parsed.0, &parsed.1);

    part_2(&parsed.0, &parsed.1);
}

fn parse(input: Vec<&str>) -> (Vec<&str>, Vec<Map>) {
    let mut maps = Vec::new();
    let mut seeds: Vec<_> = input[0].split(" ").collect();
    seeds.remove(0);

    let mut all: Vec<_> = input
        .split(|&e| e == "")
        .filter(|v| !v.is_empty())
        .collect();
    all.remove(0);

    for map in all {
        let mut ranges = Vec::new();

        for range in &map[1..map.len()] {
            let split_range: Vec<_> = range.split(" ").collect();
            ranges.push(Range {
                start_destination: split_range[0].parse::<u64>().expect("Split errorr"),
                start_source: split_range[1].parse::<u64>().expect("Split errorr"),
                length: split_range[2].parse::<u64>().expect("Split errorr"),
            });
        }

        let new_map = Map {
            name: map[0].to_string(),
            ranges: ranges,
        };

        maps.push(new_map);
    }

    (seeds, maps)
}

fn part_1(seeds: &Vec<&str>, maps: &Vec<Map>) {
    let min = seeds
        .iter()
        .map(|seed| {
            let mut current: u64;
            let seed2 = seed.parse::<u64>().expect("Split errorr");
            current = maps[0].check_ranges(seed2);
            current = maps[1].check_ranges(current);
            current = maps[2].check_ranges(current);
            current = maps[3].check_ranges(current);
            current = maps[4].check_ranges(current);
            current = maps[5].check_ranges(current);
            current = maps[6].check_ranges(current);

            current
        })
        .min()
        .expect("lol where is the min");

    println!("{min}");
}

fn part_2(seeds: &Vec<&str>, maps: &Vec<Map>) {
    let min = (0..seeds.len())
        .step_by(2)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|index| {
            let seed = seeds[*index].parse::<u64>().expect("Split errorr");
            let length = seeds[index + 1].parse::<u64>().expect("Split errorr");

            (seed..(seed + length))
                .into_par_iter()
                .map(|s| {
                    let mut current: u64;
                    let seed2 = s;
                    current = maps[0].check_ranges(seed2);
                    current = maps[1].check_ranges(current);
                    current = maps[2].check_ranges(current);
                    current = maps[3].check_ranges(current);
                    current = maps[4].check_ranges(current);
                    current = maps[5].check_ranges(current);
                    current = maps[6].check_ranges(current);

                    current
                })
                .min()
                .expect("There should be a min")
        })
        .min()
        .expect("There really should be a min dude");

    println!("{min}");
}
