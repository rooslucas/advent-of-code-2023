// use std::env;
//use std::fs;
use regex::Regex;
use regex::RegexSet;
use std::fs::read_to_string;
use std::string;

fn main() {
    let file_path = "./input/part_01.txt";
    // --snip--
    println!("In file {}", file_path);

    //let contents = fs::read_to_string(file_path)
    //    .expect("Should have been able to read the file");

    //let content_list = contents.split('\n');

    let mut liness = Vec::new();
    let mut numbers = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        liness.push(line.to_string());
    }

    for line in &liness {
        numbers.push(disect(&line));
    }

    let sum: u32 = numbers.iter().sum();
    println!("the total sum is: {}", sum);

    // Part 2

    let mut all_numbers = Vec::new();

    for line in &liness {
        all_numbers.push(part_2(&line));
    }

    let sum2: u32 = all_numbers.iter().sum();
    println!("the total sum is: {}", sum2,);
}

fn disect(instruction: &str) -> u32 {
    let mut splits: Vec<&str> = instruction.split(|c: char| c.is_alphabetic()).collect();
    splits.retain(|value| *value != "");
    //println!("{:?}", splits);
    // println!("number one {}, and number two {}", splits[0], splits[splits.len() - 1]);

    let number_1 = splits[0].to_string() + splits[splits.len() - 1];
    //println!("{}", number_1);
    let number_2 = [number_1.chars().next(), number_1.chars().last()]
        .iter()
        .flatten()
        .collect::<String>();
    // println!("{}", number_2);
    let number_final: u32 = number_2.trim().parse().expect("Split errorr");

    number_final
}

fn part_2(input: &str) -> u32 {
    let re_numbers = RegexSet::new(&[
        r"[1-9]", r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine",
    ])
    .unwrap();
    let matches: Vec<_> = re_numbers.matches(input).iter().collect();

    let re_numbers = Regex::new(r"[1-9]").unwrap();
    let re_one = Regex::new(r"one").unwrap();
    let re_two = Regex::new(r"two").unwrap();
    let re_three = Regex::new(r"three").unwrap();
    let re_four = Regex::new(r"four").unwrap();
    let re_five = Regex::new(r"five").unwrap();
    let re_six = Regex::new(r"six").unwrap();
    let re_seven = Regex::new(r"seven").unwrap();
    let re_eight = Regex::new(r"eight").unwrap();
    let re_nine = Regex::new(r"nine").unwrap();

    let all_regexes = vec![
        &re_numbers,
        &re_one,
        &re_two,
        &re_three,
        &re_four,
        &re_five,
        &re_six,
        &re_seven,
        &re_eight,
        &re_nine,
    ];

    let match_num = re_numbers.find(input);
    let match_one = re_one.find(input);
    let match_two = re_two.find(input);
    let match_three = re_three.find(input);
    let match_four = re_four.find(input);
    let match_five = re_five.find(input);
    let match_six = re_six.find(input);
    let match_seven = re_seven.find(input);
    let match_eight = re_eight.find(input);
    let match_nine = re_nine.find(input);

    let all_matches = vec![
        match_num,
        match_one,
        match_two,
        match_three,
        match_four,
        match_five,
        match_six,
        match_seven,
        match_eight,
        match_nine,
    ];

    let mut high: usize = usize::MIN;
    let mut high_value: String = "hello".to_string();
    let mut low: usize = usize::MAX;
    let mut low_value: String = "hello".to_string();

    for matchy in matches {
        let all_match: Vec<_> = all_regexes[matchy].find_iter(input).collect();
        for num in all_match {
            if num.start() >= high {
                if matchy == 0 {
                    high = num.start();
                    high_value = num.as_str().to_string();
                } else {
                    high = num.start();
                    high_value = matchy.to_string();
                }
            }

            if num.start() <= low {
                if matchy == 0 {
                    low = num.start();
                    low_value = num.as_str().to_string();
                } else {
                    low = all_matches[matchy].unwrap().start();
                    low_value = matchy.to_string();
                }
            }
        }
    }

    let number_1 = low_value.to_string() + &high_value;
    number_1.trim().parse().expect("Split errorr")
}
