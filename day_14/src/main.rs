use array2d::Array2D;
use rayon::prelude::*;
use std::{char, fs::read_to_string};

fn main() {
    let file_path = "./input/test.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();
    println!("{}", part_1(liness.clone()));

    let mut mat: Vec<Vec<char>> = liness.iter().map(|x| x.chars().collect()).collect();
    let mut i = 0;
    while i < 1000000000 {
        mat = part_2(mat);
        i += 1;
    }

    println!("{}", calc_sum(mat));
}

fn part_1(input: Vec<&str>) -> usize {
    let mat: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let array = Array2D::from_rows(&mat).unwrap();
    let mut columns = array.as_columns();

    columns = roll(columns);

    let s = Array2D::from_columns(&columns).unwrap();
    let mut normal = Vec::new();
    for row in s.as_rows() {
        let n_row: Vec<_> = row.iter().map(|x| *x).collect();
        normal.push(n_row);
    }

    normal.reverse();

    calc_sum(normal)
}

fn part_2(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Make it North
    let array = Array2D::from_rows(&input).unwrap();
    let mut north = array.as_columns();

    north = roll(north);

    // Now west
    let s = Array2D::from_columns(&north).unwrap();
    let mut normal = Vec::new();
    for row in s.as_rows() {
        let n_row: Vec<_> = row.iter().map(|x| *x).collect();
        normal.push(n_row);
    }

    let mut west = roll(normal);

    west.reverse();

    // Make it South
    let array = Array2D::from_rows(&west).unwrap();
    let mut south = array.as_columns();

    south = roll(south);

    // Now East
    let s2 = Array2D::from_columns(&south).unwrap();
    let mut normal2 = Vec::new();
    for row2 in s2.as_rows() {
        let n_row: Vec<_> = row2.iter().map(|x| *x).collect();
        normal2.push(n_row);
    }
    normal2.reverse();

    let mut east: Vec<Vec<char>> = normal2
        .iter()
        .map(|x: &Vec<char>| {
            let mut p = x.clone();
            p.reverse();
            p
        })
        .collect();
    east = roll(east);

    let east2: Vec<Vec<char>> = east
        .iter()
        .map(|x: &Vec<char>| {
            let mut p = x.clone();
            p.reverse();
            p
        })
        .collect();

    east2
}

fn calc_sum(normal: Vec<Vec<char>>) -> usize {
    let sum: usize = (0..normal.len())
        .into_par_iter()
        .map(|x| {
            let v: Vec<_> = normal[x].iter().filter(|y| y == &&'O').collect();
            v.len() * (x + 1)
        })
        .sum();

    sum
}

fn roll(mut inp: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for y in 0..inp.len() {
        let _adapt: Vec<_> = (0..inp[y].len())
            .map(|x| {
                if inp[y][x] == 'O' {
                    let mut i = x.clone();
                    while (i != 0) && (inp[y][i - 1] == '.') {
                        i -= 1;
                    }
                    inp[y][x] = '.';
                    inp[y][i] = 'O';
                }
            })
            .collect();
    }

    inp
}
