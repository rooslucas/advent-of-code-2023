use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Hash)]
struct Node {
    left: String,
    right: String,
}

fn main() {
    let file_path = "./input/day_08.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let parsed = parse(liness);

    println!("{}", part_1(parsed.1, parsed.0));
}

fn parse(mut input: Vec<&str>) -> (Vec<char>, HashMap<String, Node>) {
    let instructions: Vec<char> = input[0].chars().collect();
    input.remove(0);
    input.remove(0);
    let mut parsed = HashMap::new();

    input.iter().for_each(|x| {
        let label: Vec<_> = x.split(" =").collect();
        let other: Vec<_> = label[1].split(", ").collect();

        let left = other[0].replace(" (", "");
        let right = other[1].replace(")", "");
        parsed.insert(label[0].to_string(), Node { left, right: right });
    });
    // vec![label, left, right]  ['aaa', ['vvv', 'ccc']]

    (instructions, parsed)
}

fn part_1(input: HashMap<String, Node>, instructions: Vec<char>) -> i32 {
    let mut node = input.get("AAA").unwrap();
    let mut label = "AAA".to_string();
    let mut steps: i32 = 0;
    let mut char: char;
    let length: i32 = instructions.len().try_into().unwrap();

    // println!("{:?}, {:?}", label, node);

    while label != "ZZZ" {
        let index: usize = (steps % length).try_into().unwrap();
        char = instructions[index];
        // println!("{char}");
        // println!("{:?}, {:?}", label, node);

        if char == 'L' {
            label = node.left.to_string();
            node = &input.get(&label).unwrap();
        } else if char == 'R' {
            label = node.right.to_string();
            node = &input.get(&label).unwrap();
        }
        steps += 1;

        // println!("{},{:?}", label, node);
    }

    steps
}
