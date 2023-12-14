use array2d::Array2D;
// use itertools::Itertools;
use std::{char, fs::read_to_string};

#[derive(Debug, Clone)]
struct Pattern {
    matrix: Vec<String>,
}

impl Pattern {
    fn find_mirror(self) -> usize {
        for line in 0..self.matrix.len() {
            if line < (self.matrix.len() - 1) {
                if self.matrix[line] == self.matrix[line + 1] {
                    let mut left: Vec<_> = self.matrix[0..line + 1].to_vec();
                    let mut right: Vec<_> = self.matrix[line + 1..].to_vec();

                    if left.len() > right.len() {
                        for _i in 0..(left.len() - right.len()) {
                            left.remove(0);
                        }
                    } else if right.len() > left.len() {
                        for _i in 0..(right.len() - left.len()) {
                            right.remove(right.len() - 1);
                        }
                    }

                    right.reverse();

                    if left == right {
                        println!("{}", (line + 1) * 100);
                        return (line + 1) * 100;
                    }
                }
            }
        }

        let mat: Vec<Vec<char>> = self.matrix.iter().map(|x| x.chars().collect()).collect();

        let array = Array2D::from_rows(&mat).unwrap();
        let columns = array.as_columns();

        for line in 0..columns.len() {
            if line < (columns.len() - 1) {
                if columns[line] == columns[line + 1] {
                    let mut top: Vec<_> = columns[0..line + 1].to_vec();
                    let mut down: Vec<_> = columns[line + 1..].to_vec();

                    if top.len() > down.len() {
                        for _i in 0..(top.len() - down.len()) {
                            top.remove(0);
                        }
                    } else if down.len() > top.len() {
                        for _i in 0..(down.len() - top.len()) {
                            down.remove(down.len() - 1);
                        }
                    }

                    down.reverse();

                    if top == down {
                        println!("{}", line + 1);
                        return line + 1;
                    }
                }
            }
        }

        0
        // self.matrix
    }
}

fn main() {
    let file_path = "./input/day_13.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let patterns = parse(liness);
    println!(
        "{}",
        patterns
            .iter()
            .map(|p| p.clone().find_mirror())
            .sum::<usize>()
    );
}

fn parse(input: Vec<&str>) -> Vec<Pattern> {
    let all: Vec<_> = input
        .split(|&e| e == "")
        .filter(|v| !v.is_empty())
        .collect();

    let patterns: Vec<Pattern> = all
        .iter()
        .map(|y| {
            let p = Pattern {
                matrix: y.iter().map(|x| x.to_owned().to_owned()).collect(),
            };
            p
        })
        .collect();
    patterns
}
