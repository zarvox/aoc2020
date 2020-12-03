use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Trees {
  rows: usize,
  cols: usize,
  grid: Vec<Vec<u8>>,
}

fn trees_on_path(trees: &Trees, row_step: usize, col_step: usize) -> u64 {
    let mut row = 0;
    let mut col = 0;
    let mut trees_encountered = 0;
    while row < trees.rows {
        if trees.grid[row][col] != 46 {
            // println!("tree at {} {}", row, col);
            trees_encountered += 1;
        }
        col = (col + col_step) % trees.cols;
        row = (row + row_step);
    }
    trees_encountered
}

fn part_one(trees: &Trees) {
    let trees_encountered = trees_on_path(trees, 1, 3);
    println!("{:?}", trees_encountered);
}

fn part_two(trees: &Trees) {
    let a = trees_on_path(trees, 1, 1);
    let b = trees_on_path(trees, 1, 3);
    let c = trees_on_path(trees, 1, 5);
    let d = trees_on_path(trees, 1, 7);
    let e = trees_on_path(trees, 2, 1);

    println!("a: {}\nb: {}\nc: {}\nd: {}\ne: {}\n", a, b, c, d, e);
    println!("{}\n", a * b * c * d * e);
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut values: Vec<Vec<u8>> = reader
        .lines()
        .map(|maybe_l| {
            let l = maybe_l.unwrap();
            println!("{}", l.len());
            l.into_bytes()
        })
        .collect();

    let trees = Trees {
        rows: values.len(),
        cols: values[0].len(),
        grid: values,
    };

    part_one(&trees);
    part_two(&trees);
}
