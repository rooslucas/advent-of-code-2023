use array2d::{Array2D, Error};
use itertools::Itertools;
use std::{char, fs::read_to_string};

struct Galaxy {
    number: i32,
    x: i32,
    y: i32,
}

fn main() {
    let file_path = "./input/day_11.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();
    let parsed = parse(matrix);
    println!("{}", parsed.0.len());

    // parse(matrix).iter().for_each(|x: &Vec<char>| println!("{:?}", x));
    println!("{:?}", (part_1(parsed.0, parsed.1)));
}

fn parse(mut input: Vec<Vec<char>>) -> (Vec<Galaxy>, i32) {
    // double empty lines
    // rows

    let mut index = Vec::new();
    for x in 0..input.len() {
        if !input[x].contains(&'#') {
            index.push(x);
        }
    }
    println!("{}", index.len());

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
        let n_row: Vec<_> = row.iter().map(|x| x.to_string()).collect();
        matrix.push(n_row);
    }

    // matrix.iter().for_each(|x| println!("{:?}", x));
    // println!("\n",);

    // change # to numbers
    let mut num = 1;
    let mut galaxies = Vec::new();

    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if matrix[x][y] == "#" {
                matrix[x][y] = num.to_string();
                galaxies.push(Galaxy {
                    number: num,
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                num += 1;
            }
        }
    }

    matrix.iter().for_each(|x| println!("{:?}", x));

    // return matrix
    (galaxies, num)
}

fn part_1(galaxies: Vec<Galaxy>, num: i32) -> i32 {
    // find all unique combinations of galaxies
    let all_numbs: Vec<_> = (1..num).collect();
    // println!("{:?}", all_numbs);

    let all_combinations: Vec<_> = all_numbs
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b))
        .unique()
        .collect();

    // println!("{:?}", all_combinations);
    let mut total = 0;

    for combi in all_combinations {
        let g1: Vec<&Galaxy> = galaxies.iter().filter(|g| g.number == *combi.0).collect();
        let g2: Vec<&Galaxy> = galaxies.iter().filter(|g| g.number == *combi.1).collect();
        total += find_path(g1[0], g2[0]);
    }

    // find index of those galaxies (x, y) (x, y)
    // find path
    // return sum of numbers.
    total.try_into().unwrap()
}

fn find_path(xy_1: &Galaxy, xy_2: &Galaxy) -> i32 {
    let x = (xy_2.x - xy_1.x).abs();
    let y = (xy_2.y - xy_1.y).abs();
    // println!("{}", x + y);

    x + y
}
