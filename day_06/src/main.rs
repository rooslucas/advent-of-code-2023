use rayon::prelude::*;
use std::fs::read_to_string;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let file_path = "./input/day_06.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let parsed = parse1(&liness);
    let race = parse2(liness);

    println!("{}", find_winnings(parsed));
    println!("{}", find_winnings(race));
}

fn parse1(input: &Vec<&str>) -> Vec<Race> {
    let mut times: Vec<_> = input[0].split(" ").collect();
    times.retain(|value| *value != "");

    let mut distances: Vec<_> = input[1].split(" ").collect();
    distances.retain(|value| *value != "");

    let parsed = (1..times.len())
        .into_par_iter()
        .map(|i| Race {
            time: times[i].parse::<u64>().expect("Error error"),
            distance: distances[i].parse::<u64>().expect("Error error"),
        })
        .collect();

    parsed
}

fn parse2(input: Vec<&str>) -> Vec<Race> {
    let mut times: Vec<_> = input[0].split(" ").collect();
    times.retain(|value| *value != "");
    let time = (times[1].to_string() + times[2] + times[3] + times[4])
        .trim()
        .parse::<u64>()
        .expect("Error error");

    let mut distances: Vec<_> = input[1].split(" ").collect();
    distances.retain(|value| *value != "");
    let distance = (distances[1].to_string() + distances[2] + distances[3] + distances[4])
        .trim()
        .parse::<u64>()
        .expect("Error error");

    println!("{time}, {distance}");

    vec![Race { time, distance }]
}

fn find_winnings(input: Vec<Race>) -> u64 {
    input
        .par_iter()
        .map(|race| {
            (0..(race.time + 1))
                .into_par_iter()
                .filter(|sec| {
                    let dist = sec * (race.time - sec);
                    dist > race.distance
                })
                .count() as u64
        })
        .product()
}
