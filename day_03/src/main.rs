use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_path = "./input/day_03.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();
    let parsed_numbers = parse(&matrix);
    let pars = part_2(&matrix, &parsed_numbers);

    // let set = part_1(matrix.clone());

    // let sum: u32 = set.iter().sum();

    // println!("total is {sum}");

    // let set2 = part_2(matrix.clone());

    let sum2: u32 = pars.iter().sum();

    println!("total is {sum2}");
}
// #[derive(Debug)]
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
                            y: y - 1,
                            x1: matrix[y].len() - numb.len(),
                            x2: matrix[y].len() - 1,
                        };
                        final_set.push(parse_number);
                    }

                    numb = "".to_string();
                }
            }
        }
    }
    // println!("{:#?}", final_set);

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

fn part_2(matrix: &Vec<Vec<char>>, parsed_numbers: &Vec<PartNumber>) -> Vec<u32> {
    let mut gear_map = HashMap::new();
    let mut final_set = Vec::new();
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let char = matrix[y][x];
            if numbers.contains(&char) {
                let mut positions_1 = Vec::new();
                let mut positions_2 = Vec::new();
                // println!("{}", matrix[elem][char]);
                let pos_2 = x.clone();
                let pos_1: usize = y.clone();

                if pos_1 != 0 && pos_1 != (matrix.len() - 1) {
                    positions_1 = vec![pos_1, (pos_1 + 1), (pos_1 - 1)];
                } else if pos_1 == 0 {
                    positions_1 = vec![pos_1, (pos_1 + 1)];
                } else {
                    positions_1 = vec![pos_1, (pos_1 - 1)];
                }

                if pos_2 != 0 && pos_2 != (matrix[y].len() - 1) {
                    positions_2 = vec![pos_2, (pos_2 + 1), (pos_2 - 1)];
                } else if pos_2 == 0 {
                    positions_2 = vec![pos_2, (pos_2 + 1)];
                } else {
                    positions_2 = vec![pos_2, (pos_2 - 1)];
                }

                for post_1 in &positions_1 {
                    for post_2 in &positions_2 {
                        let char_2 = matrix[*post_1][*post_2];

                        if char_2 == '*' {
                            let number = parsed_numbers
                                .iter()
                                .filter(|i| (i.y == y) && (i.x1..(i.x2 + 1)).contains(&x))
                                .collect::<Vec<_>>();

                            let key = (post_1.clone(), post_2.clone());

                            if !gear_map.contains_key(&key) {
                                gear_map.insert(key, number);
                            } else if number.len() > 0 {
                                gear_map.get_mut(&key).unwrap().push(number[0]);
                            }
                        }
                    }
                }
            }
        }
    }

    for value in gear_map.values() {
        let numbs = value.iter().unique_by(|p| &p.number).collect::<Vec<_>>();
        if numbs.len() >= 2 {
            // println!(
            //     "numb 1 {:?}, numb2 {}, product {}",
            //     numbs[0].number,
            //     numbs[1].number,
            //     (numbs[0].number * numbs[1].number)
            // );
            final_set.push(numbs[0].number * numbs[1].number);
        }
    }

    final_set
}

// 1. maak HashMap<(x,y), Vec<PartNumber>> waarbij (x,y) het coordinaat is van een *
// 2. Ga door alle key, value pairs van de hashmap heen, en filter het zodat je alleen
//    de entries hebt met 2 part numbers
// 3. voila
