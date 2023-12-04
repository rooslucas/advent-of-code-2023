use std::fs::read_to_string;

fn main() {
    let file_path = "./input/day_04.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    // Part 1
    let mut total_points: u32 = 0;

    for card in liness {
        total_points += card_check(card);
    }

    println!("total points is {total_points}");
}

fn card_check(input: &str) -> u32 {
    let mut points: u32 = 0;

    let split_card: Vec<_> = input.split(":").collect();
    let split_numbers: Vec<_> = split_card[1].split("|").collect();

    let mut check_numbers: Vec<_> = split_numbers[0].split(" ").collect();
    check_numbers.retain(|value| *value != "");
    let mut own_numbers: Vec<_> = split_numbers[1].split(" ").collect();
    own_numbers.retain(|value| *value != "");

    for numb in check_numbers {
        if own_numbers.contains(&numb) {
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}
