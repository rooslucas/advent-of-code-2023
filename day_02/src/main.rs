use std::fs::read_to_string;

fn main() {
    let file_path = "./input/day_02.txt";
    println!("In file {}", file_path);

    let mut liness = Vec::new();
    let mut ids = Vec::new();
    let mut powers = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        liness.push(line.to_string());
    }

    for game in &liness {
        ids.push(readgame_part1(game));
    }

    let sum1: u32 = ids.iter().sum();
    println!("the total games is: {}", sum1);

    for game2 in &liness {
        powers.push(readgame_part2(game2));
    }

    let sum2: u32 = powers.iter().sum();
    println!("the total power is: {}", sum2);
}

fn readgame_part1(input: &str) -> u32 {
    // split on :
    let double: Vec<&str> = input.split(":").collect();
    let get_id: Vec<&str> = double[0].split(" ").collect();
    let mut id: u32 = get_id[1].trim().parse().expect("Split errorr");
    // split on ;
    let point: Vec<&str> = double[1].split(";").collect();

    // Check for each grab if valid
    for game in point {
        let comma: Vec<&str> = game.split(",").collect();
        let mut red_num: u32 = 0;
        let mut blue_num: u32 = 0;
        let mut green_num: u32 = 0;

        for block in comma {
            if block.contains("red") {
                let space: Vec<&str> = block.split(" ").collect();
                red_num += space[1].trim().parse::<u32>().expect("Split errorr");
            } else if block.contains("blue") {
                let space: Vec<&str> = block.split(" ").collect();
                blue_num += space[1].trim().parse::<u32>().expect("Split errorr");
            } else if block.contains("green") {
                let space: Vec<&str> = block.split(" ").collect();
                green_num += space[1].trim().parse::<u32>().expect("Split errorr");
            }
        }

        // check_true(red, blue, green)
        if !check_true(red_num, blue_num, green_num) {
            id = 0;
            break; // if false
        }
    }
    id
}

fn check_true(red: u32, blue: u32, green: u32) -> bool {
    (red <= 12) && (blue <= 14) && (green <= 13)
}

fn readgame_part2(input: &str) -> u32 {
    // split on :
    let double: Vec<&str> = input.split(":").collect();

    // split on ;
    let point: Vec<&str> = double[1].split(";").collect();

    let mut red_num: u32 = 0;
    let mut blue_num: u32 = 0;
    let mut green_num: u32 = 0;
    // Check for each grab if valid
    for game in point {
        let comma: Vec<&str> = game.split(",").collect();

        for block in comma {
            if block.contains("red") {
                let space: Vec<&str> = block.split(" ").collect();
                let red: u32 = space[1].trim().parse::<u32>().expect("Split errorr");
                if red > red_num {
                    red_num = red;
                }
            } else if block.contains("blue") {
                let space: Vec<&str> = block.split(" ").collect();
                let blue: u32 = space[1].trim().parse::<u32>().expect("Split errorr");
                if blue > blue_num {
                    blue_num = blue;
                }
            } else if block.contains("green") {
                let space: Vec<&str> = block.split(" ").collect();
                let green: u32 = space[1].trim().parse::<u32>().expect("Split errorr");
                if green > green_num {
                    green_num = green;
                }
            }
        }
    }
    red_num * green_num * blue_num
}
