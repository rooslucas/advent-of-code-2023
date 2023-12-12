use array2d::{Array2D, Error};
use std::{char, fs::read_to_string};

fn main() {
    let file_path = "./input/test.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();

    parse(matrix).iter().for_each(|x| println!("{:?}", x));
}

fn parse(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // double empty lines
    // rows

    let mut index = Vec::new();
    for x in 0..input.len() {
        if !input[x].contains(&'#') {
            index.push(x);
        }
    }

    for i in 0..index.len() {
        let ind: usize = index[i] + i;
        input.insert(ind, input[ind].clone());
    }
    let array = Array2D::from_rows(&input).unwrap();
    let mut columns = array.as_columns();
    let mut indexcol = Vec::new();
    for x in 0..columns.len() {
        if !columns[x].contains(&'#') {
            indexcol.push(x);
        }
    }

    for i in 0..indexcol.len() {
        let ind: usize = indexcol[i] + i;
        columns.insert(ind, columns[ind].clone());
    }
    let s = Array2D::from_columns(&columns).unwrap();
    let mut matrix = Vec::new();
    for row in s.as_rows() {
        matrix.push(row);
    }

    // matrix.iter().for_each(|x| println!("{:?}", x));
    // println!("\n",);

    // change # to numbers
    let mut num = 1;

    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if matrix[x][y] == '#' {
                let vec: Vec<char> = num.to_string().chars().collect();
                num += 1;
                matrix[x][y] = vec[0];
            }
        }
    }

    // matrix.iter().for_each(|x| println!("{:?}", x));

    // return matrix
    matrix
}

fn find_path() {}
