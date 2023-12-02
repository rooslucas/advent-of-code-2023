// use std::env;
//use std::fs;
use std::fs::read_to_string;

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

    println!("{:?}", numbers);

    let sum: u32 = numbers.iter().sum();
    println!("the total sum is: {}", sum);

}

fn disect(instruction: &str) -> u32 {
    let mut splits: Vec<&str> = instruction.split(|c: char| c.is_alphabetic()).collect();
    splits.retain(|value| *value != "");
    println!("{:?}", splits);
    // println!("number one {}, and number two {}", splits[0], splits[splits.len() - 1]);

    let number_1 = splits[0].to_string() + splits[splits.len() - 1];
    //println!("{}", number_1);
    let number_2 = [number_1.chars().next(), number_1.chars().last()].iter().flatten().collect::<String>();
    println!("{}", number_2);
    let number_final: u32 = number_2.trim().parse().expect("Split errorr");

    number_final

}

