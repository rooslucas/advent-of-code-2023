use std::fs::read_to_string;
use std::u64;

#[derive(Debug)]
struct Range {
    start_destination: u64,
    start_source: u64,
    length: u64,
}
impl Range {
    fn range_check(&self, i: u64) -> u64 {
        let end_source = self.start_source + self.length;

        if (self.start_source..end_source).contains(&i) {
            let diff = i - self.start_source;
            return self.start_destination + diff;
        } else {
            return 0;
        }
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

impl Map {
    fn check_ranges(&self, value: u64) -> u64 {
        let mut check_list = Vec::new();
        for range in &self.ranges {
            check_list.push(range.range_check(value));
        }

        let sum: u64 = check_list.iter().sum();
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
    // println!("{:#?}", parsed);

    part_1(parsed.0, parsed.1)
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

        // println!("{:#?}", new_map);
        maps.push(new_map);
    }

    (seeds, maps)
}

fn part_1(seeds: Vec<&str>, maps: Vec<Map>) {
    let mut locations = Vec::new();

    for seed in seeds {
        let mut current: u64;
        let seed2 = seed.parse::<u64>().expect("Split errorr");
        current = maps[0].check_ranges(seed2);
        current = maps[1].check_ranges(current);
        current = maps[2].check_ranges(current);
        current = maps[3].check_ranges(current);
        current = maps[4].check_ranges(current);
        current = maps[5].check_ranges(current);
        current = maps[6].check_ranges(current);

        println!("{current}");
        locations.push(current);
    }
    locations.sort();
    println!("{:?}", locations);
}
