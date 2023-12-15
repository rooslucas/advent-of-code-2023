use std::{char, fs::read_to_string};

fn main() {
    let file_path = "./input/day_15.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let parsed = parse(liness);
    part_1(parsed);
}

fn parse(input: Vec<&str>) -> Vec<Vec<char>> {
    let split: Vec<_> = input[0].split(",").collect();
    let char_split: Vec<Vec<char>> = split.iter().map(|x| x.chars().collect()).collect();

    char_split
}

fn part_1(input: Vec<Vec<char>>) -> u32 {
    let sum: u32 = input
        .iter()
        .map(|x| {
            let mut current = 0;
            x.iter().for_each(|y| {
                let i = *y as u32;
                current += i;
                current *= 17;
                current = current % 256;
            });

            current
        })
        .sum();

    println!("{sum}");
    sum
}
