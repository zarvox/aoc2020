use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn part_one(values: &Vec<i64>) {
    let pair = values.iter()
        .tuple_combinations()
        .filter(|(&x, &y)| x + y == 2020)
        .next()
        .expect("one pair should sum to 2020");
    println!("{:?}", pair);
    println!("{:?}", pair.0 * pair.1);
}

fn part_two(values: &Vec<i64>) {
    let vec = values.iter()
        .combinations(3)
        .filter(|v| v[0] + v[1] + v[2] == 2020)
        .next()
        .expect("one triple should sum to 2020");
    println!("{:?}", vec);
    println!("{:?}", vec[0] * vec[1] * vec[2]);
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut values: Vec<i64> = reader.lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    part_one(&values);
    part_two(&values);

}
