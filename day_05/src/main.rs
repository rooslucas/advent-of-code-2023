use std::fs::read_to_string;
use std::slice::Split;

#[derive(Debug)]
struct Range {
    start_destination: u64,
    start_source: u64,
    length: u64,
}
#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

fn main() {
    let file_path = "./input/test.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let parsed = parse(liness);
    println!("{:#?}", parsed);
}

fn parse(input: Vec<&str>) -> (Vec<&str>, Vec<Map>) {
    let mut parsed_input = Vec::new();
    let mut seeds: Vec<_> = input[0].split(" ").collect();
    seeds.remove(0);
    println!("{:?}", seeds);

    let mut all: Vec<_> = input
        .split(|&e| e == "")
        .filter(|v| !v.is_empty())
        .collect();
    all.remove(0);
    println!("{:?}", all);

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

        let mut new_map = Map {
            name: map[0].to_string(),
            ranges: ranges,
        };

        println!("{:#?}", new_map);
        parsed_input.push(new_map);
    }

    (seeds, parsed_input)
}
