use array2d::Array2D;
use std::{char, fs::read_to_string};

fn main() {
    let file_path = "./input/day_14.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    println!("{}", part_1(liness));
}

fn part_1(input: Vec<&str>) -> usize {
    let mat: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let array = Array2D::from_rows(&mat).unwrap();
    let mut columns = array.as_columns();

    for y in 0..columns.len() {
        let adapt: Vec<_> = (0..columns[y].len())
            .map(|x| {
                if columns[y][x] == 'O' {
                    let mut i = x.clone();
                    while (i != 0) && (columns[y][i - 1] == '.') {
                        i -= 1;
                    }
                    columns[y][x] = '.';
                    columns[y][i] = 'O';
                }
            })
            .collect();
    }

    let s = Array2D::from_columns(&columns).unwrap();
    let mut normal = Vec::new();
    for row in s.as_rows() {
        let n_row: Vec<_> = row.iter().map(|x| *x).collect();
        normal.push(n_row);
    }

    normal.reverse();

    let sum: usize = (0..normal.len())
        .map(|x| {
            let v: Vec<_> = normal[x].iter().filter(|y| y == &&'O').collect();
            v.len() * (x + 1)
        })
        .sum();

    sum
}

// Transform into columns.
// Each rock up until meeting a #

// Transform back to rows
// reverse index
// rocks * (index + 1)
