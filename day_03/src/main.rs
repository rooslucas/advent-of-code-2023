use std::fs::read_to_string;

fn main() {
    let file_path = "./input/test.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();
    let kutparsednumbers = parse(&matrix);

    // let set = part_1(matrix.clone());

    // let sum: u32 = set.iter().sum();

    // println!("total is {sum}");

    // let set2 = part_2(matrix.clone());

    // let sum2: u32 = set2.iter().sum();

    // println!("total is {sum2}");
}
#[derive(Debug)]
struct PartNumber {
    number: u32,
    y: usize,
    x1: usize,
    x2: usize,
}

//

fn parse(matrix: &Vec<Vec<char>>) -> Vec<PartNumber> {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut numb: String = "".to_string();
    let mut final_set = Vec::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if numbers.contains(&matrix[y][x]) {
                numb += &matrix[y][x].to_string();
            } else {
                if numb != "" {
                    if x > 0 {
                        //final_set.push(numb.trim().parse::<u32>().expect("Split errorr"));
                        let parse_number = PartNumber {
                            number: numb.parse::<u32>().expect("Split errorr"),
                            y,
                            x1: x - numb.len(),
                            x2: x - 1,
                        };
                        final_set.push(parse_number);
                    } else {
                        let parse_number = PartNumber {
                            number: numb.parse::<u32>().expect("Split errorr"),
                            y,
                            x1: matrix[y].len() - numb.len(),
                            x2: matrix[y].len(),
                        };
                        final_set.push(parse_number);
                    }

                    numb = "".to_string();
                }
            }
        }
    }
    println!("{:#?}", final_set);

    final_set
}

fn part_1(matrix: Vec<Vec<char>>) -> Vec<u32> {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut numb: String = "".to_string();
    let mut final_set = Vec::new();
    let mut numb: String = "".to_string();
    let mut flag = false;

    for elem in 0..matrix.len() {
        for char in 0..matrix[elem].len() {
            if numbers.contains(&matrix[elem][char]) {
                let mut positions_1 = Vec::new();
                let mut positions_2 = Vec::new();
                // println!("{}", matrix[elem][char]);
                numb += &matrix[elem][char].to_string();
                let pos_2 = char;
                let pos_1: usize = elem;

                if pos_1 != 0 && pos_1 != (matrix.len() - 1) {
                    positions_1 = vec![pos_1, (pos_1 + 1), (pos_1 - 1)];
                } else if pos_1 == 0 {
                    positions_1 = vec![pos_1, (pos_1 + 1)];
                } else {
                    positions_1 = vec![pos_1, (pos_1 - 1)];
                }

                if pos_2 != 0 && pos_2 != (matrix[elem].len() - 1) {
                    positions_2 = vec![pos_2, (pos_2 + 1), (pos_2 - 1)];
                } else if pos_2 == 0 {
                    positions_2 = vec![pos_2, (pos_2 + 1)];
                } else {
                    positions_2 = vec![pos_2, (pos_2 - 1)];
                }

                for post_1 in positions_1 {
                    for post_2 in &positions_2 {
                        if matrix[post_1][*post_2] != '.'
                            && !numbers.contains(&matrix[post_1][*post_2])
                        {
                            flag = true;
                            //println!("sign{}", matrix[post_1][*post_2]);
                        }
                    }
                }
            } else {
                if flag {
                    final_set.push(numb.trim().parse::<u32>().expect("Split errorr"));
                    flag = false;
                    // println!("sign {}, number{}", matrix[elem][char], numb);
                }
                numb = "".to_string();
            }
        }
    }

    final_set
}

fn part_2(matrix: Vec<Vec<char>>) -> Vec<u32> {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut final_set = Vec::new();
    let mut positions_1 = Vec::new();
    let mut positions_2 = Vec::new();
    let mut numb: String = "".to_string();
    let mut flag = false;
    let mut flag2 = false;

    for elem in 0..matrix.len() {
        for char in 0..matrix[elem].len() {
            if &matrix[elem][char] == &'*' {
                // println!("{}", matrix[elem][char]);
                //numb += &matrix[elem][char].to_string();
                let pos_2 = char;
                let pos_1: usize = elem;

                if pos_1 != 0 && pos_1 != (matrix.len() - 1) {
                    positions_1 = vec![pos_1, (pos_1 + 1), (pos_1 - 1)];
                } else if pos_1 == 0 {
                    positions_1 = vec![pos_1, (pos_1 + 1)];
                } else {
                    positions_1 = vec![pos_1, (pos_1 - 1)];
                }

                if pos_2 != 0 && pos_2 != (matrix[elem].len() - 1) {
                    positions_2 = vec![pos_2, (pos_2 + 1), (pos_2 - 1)];
                } else if pos_2 == 0 {
                    positions_2 = vec![pos_2, (pos_2 + 1)];
                } else {
                    positions_2 = vec![pos_2, (pos_2 - 1)];
                }

                for post_1 in positions_1 {
                    for post_2 in &positions_2 {
                        if numbers.contains(&matrix[post_1][*post_2]) && !flag {
                            flag = true;
                            println!("sign{}", matrix[post_1][*post_2]);
                        } else if numbers.contains(&matrix[post_1][*post_2]) && flag {
                            flag2 = true;
                            println!("sign{}", matrix[post_1][*post_2]);
                        }
                    }
                }
            } else {
                if flag && flag2 {
                    //final_set.push(numb.trim().parse::<u32>().expect("Split errorr"));
                    flag = false;
                    flag2 = false;
                    println!("sign {}, number{}", matrix[elem][char], numb);
                }
                numb = "".to_string();
            }
        }
    }

    final_set
}
