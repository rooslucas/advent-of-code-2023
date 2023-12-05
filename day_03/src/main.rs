use std::fs::read_to_string;

fn main() {
    let file_path = "./input/day_03.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();

    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut final_set = Vec::new();
    let mut positions_1 = Vec::new();
    let mut positions_2 = Vec::new();
    let mut numb: String = "".to_string();
    let mut flag = false;

    for elem in 0..matrix.len() {
        for char in 0..matrix[elem].len() {
            if numbers.contains(&matrix[elem][char]) {
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
                            // println!("sign{}", matrix[post_1][*post_2]);
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

    let sum: u32 = final_set.iter().sum();

    // println!("{:?}", final_set);
    println!("total is {sum}");
}
