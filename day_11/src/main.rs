use array2d::Array2D;
use itertools::Itertools;
use std::{char, fs::read_to_string};

#[derive(Debug, Clone)]
struct Galaxy {
    number: i32,
    x: i64,
    y: i64,
}

fn main() {
    let file_path = "./input/day_11.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    let matrix: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();

    let parsed1 = parse(matrix.clone());
    let parsed2 = parse2(matrix);

    println!("{:?}", (part_1(parsed1.0, parsed1.1)));
    println!("{:?}", (part_1(parsed2.0, parsed2.1)));
}

fn parse(mut input: Vec<Vec<char>>) -> (Vec<Galaxy>, i32) {
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
        let n_row: Vec<_> = row.iter().map(|x| x.to_string()).collect();
        matrix.push(n_row);
    }

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

    // return matrix
    (galaxies, num)
}

fn parse2(input: Vec<Vec<char>>) -> (Vec<Galaxy>, i32) {
    let mut index = Vec::new();
    for x in 0..input.len() {
        if !input[x].contains(&'#') {
            index.push(x);
        }
    }

    let array = Array2D::from_rows(&input).unwrap();
    let columns = array.as_columns();

    let mut indexcol = Vec::new();
    for x in 0..columns.len() {
        if !columns[x].contains(&'#') {
            indexcol.push(x);
        }
    }

    let s = Array2D::from_columns(&columns).unwrap();
    let mut matrix = Vec::new();
    for row in s.as_rows() {
        let n_row: Vec<_> = row.iter().map(|x| x.to_string()).collect();
        matrix.push(n_row);
    }

    // change # to numbers
    let mut num = 1;
    let mut galaxies = Vec::new();
    let size = 1000000 - 1;

    for x in 0..matrix.len() {
        let rows = index.iter().filter(|i| i < &&x).collect::<Vec<_>>();
        for y in 0..matrix[x].len() {
            let columns = indexcol.iter().filter(|i| i < &&y).collect::<Vec<_>>();

            if matrix[x][y] == "#" {
                matrix[x][y] = num.to_string();

                let new_x = x + (rows.len() * size);
                let new_y = y + (columns.len() * size);

                galaxies.push(Galaxy {
                    number: num,
                    x: new_x.try_into().unwrap(),
                    y: new_y.try_into().unwrap(),
                });
                num += 1;
            }
        }
    }

    // return matrix
    (galaxies, num)
}

fn part_1(galaxies: Vec<Galaxy>, num: i32) -> i64 {
    // find all unique combinations of galaxies
    let all_numbs: Vec<_> = (1..num).collect();

    let all_combinations: Vec<_> = all_numbs
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b))
        .unique()
        .collect();

    let mut total = 0;

    for combi in all_combinations {
        let g1: Vec<&Galaxy> = galaxies.iter().filter(|g| g.number == *combi.0).collect();
        let g2: Vec<&Galaxy> = galaxies.iter().filter(|g| g.number == *combi.1).collect();
        total += find_path(g1[0], g2[0]);
    }

    total.try_into().unwrap()
}

fn find_path(xy_1: &Galaxy, xy_2: &Galaxy) -> i64 {
    let x = (xy_2.x - xy_1.x).abs();
    let y = (xy_2.y - xy_1.y).abs();

    x + y
}
