use std::fs::read_to_string;

fn main() {
    let file_path = "./input/day_04.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    // Part 1
    let mut total_points: u32 = 0;

    for card in &liness {
        total_points += card_check1(card);
    }

    println!("total points is {total_points}");

    // Part 2
    let mut total_scratchcards: u32 = liness.len().try_into().unwrap();

    for card in 0..liness.len() {
        total_scratchcards += card_check2(card, liness.clone());
    }

    println!("total points is {total_scratchcards}");
}

fn card_check1(input: &str) -> u32 {
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

fn card_check2(index: usize, file: Vec<&str>) -> u32 {
    let mut extra_cards: u32 = 0;
    let input = file[index];

    let split_card: Vec<_> = input.split(":").collect();
    let split_numbers: Vec<_> = split_card[1].split("|").collect();

    let mut check_numbers: Vec<_> = split_numbers[0].split(" ").collect();
    check_numbers.retain(|value| *value != "");
    let mut own_numbers: Vec<_> = split_numbers[1].split(" ").collect();
    own_numbers.retain(|value| *value != "");
    let mut i = 1;

    for numb in check_numbers {
        if own_numbers.contains(&numb) {
            extra_cards += 1;
            extra_cards += card_check2(index + i, file.clone());
            i += 1;
        }
    }

    extra_cards
}
