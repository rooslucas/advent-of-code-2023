use std::fs::read_to_string;

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

fn main() {
    let file_path = "./input/day_08.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let parsed = parse(liness);
    println!("{:?}", parsed);

    println!("{}", part_1(parsed.1, parsed.0));
}

fn parse(mut input: Vec<&str>) -> (Vec<char>, Vec<Node>) {
    let instructions: Vec<char> = input[0].chars().collect();
    input.remove(0);
    input.remove(0);

    let parsed: Vec<_> = input
        .iter()
        .map(|x| {
            let label: Vec<_> = x.split(" =").collect();
            let other: Vec<_> = label[1].split(", ").collect();

            let left = other[0].replace(" (", "");
            let right = other[1].replace(")", "");
            Node {
                label: label[0].to_owned(),
                left: left,
                right: right,
            }
        })
        .collect();
    // vec![label, left, right]  ['aaa', ['vvv', 'ccc']]

    (instructions, parsed)
}

fn part_1(input: Vec<Node>, instructions: Vec<char>) -> i32 {
    let mut node = &input[0];
    let mut label = &input[0].label;
    let mut steps = 0;
    let mut index = 0;

    while label != "ZZZ" {
        let char = instructions[index];

        if char == 'R' {
            label = &node.right;
            let find_note: Vec<_> = input.iter().filter(|x| x.label == *label).collect();
            node = find_note[0];
        } else if char == 'L' {
            label = &node.left;
            let find_note: Vec<_> = input.iter().filter(|x| x.label == *label).collect();
            node = find_note[0];
        }
        steps += 1;

        if index < (instructions.len() - 1) {
            index += 1;
        } else {
            index = 0;
        }

        println!("{:?}", node);
    }

    steps
}
