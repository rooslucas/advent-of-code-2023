use std::fs::read_to_string;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    let file_path = "./input/test.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let parsed = parse1(&liness);
    let race = parse2(liness);

    part_1(parsed);
}

fn parse1(input: &Vec<&str>) -> Vec<Race> {
    let mut times: Vec<_> = input[0].split(" ").collect();
    times.retain(|value| *value != "");

    let mut distances: Vec<_> = input[1].split(" ").collect();
    distances.retain(|value| *value != "");

    let parsed = (1..times.len())
        .map(|i| Race {
            time: times[i].parse::<u32>().expect("Error error"),
            distance: distances[i].parse::<u32>().expect("Error error"),
        })
        .collect();

    parsed
}

fn parse2(input: Vec<&str>) -> Race {
    let mut times: Vec<_> = input[0].split(" ").collect();
    times.retain(|value| *value != "");
    let time = (times[1].to_string() + times[2] + times[3])
        .trim()
        .parse::<u32>()
        .expect("Error error");

    let mut distances: Vec<_> = input[1].split(" ").collect();
    distances.retain(|value| *value != "");
    let distance = (distances[1].to_string() + distances[2] + distances[3])
        .trim()
        .parse::<u32>()
        .expect("Error error");

    println!("{time}, {distance}");

    Race { time, distance }
}

fn part_1(input: Vec<Race>) {
    let mut ways = Vec::new();
    for race in input {
        let mut way = 0;
        for sec in 0..(race.time + 1) {
            let dist = sec * (race.time - sec);
            if dist > race.distance {
                way += 1;
            }
        }
        ways.push(way);
    }

    println!("{}", ways.iter().product::<u32>());
}

// fn part_2() {}
