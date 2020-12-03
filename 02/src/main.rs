use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Policy {
  min: i64,
  max: i64,
  letter: char,
}

#[derive(Debug)]
struct TestCase {
  p: Policy,
  pw: String,
}

fn eval_testcase_one(t: &TestCase) -> bool {
  // Count chars that match policy
  let l_count = t.pw.chars().filter(|&c| c == t.p.letter).count() as i64;
  // In acceptable range?
  t.p.min <= l_count && l_count <= t.p.max
}

fn part_one(values: &Vec<TestCase>) {
    let res = values
        .iter()
        .filter(|t| eval_testcase_one(&t))
        .count();
    println!("{:?}", res);
}

fn eval_testcase_two(t: &TestCase) -> bool {
  // Get the min'th and max'th chars from pw
  let a = t.pw.chars().nth((t.p.min - 1) as usize).unwrap();
  let b = t.pw.chars().nth((t.p.max - 1) as usize).unwrap();

  // see if each matches, and return the xor
  (t.p.letter == a) ^ (t.p.letter == b)
}

fn part_two(values: &Vec<TestCase>) {
    let res = values
        .iter()
        .filter(|t| eval_testcase_two(&t))
        .count();
    println!("{:?}", res);
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut values: Vec<TestCase> = reader
        .lines()
        .map(|maybe_l| {
            let l = maybe_l.unwrap();
            let v: Vec<_> = l.split(" ").collect();
            let lens: Vec<_> = v[0].split("-").collect();
            let c = v[1].chars().next().unwrap();

            let pw = v[2];
            let policy = Policy {
                min: lens[0].parse::<i64>().unwrap(),
                max: lens[1].parse::<i64>().unwrap(),
                letter: c,
            };

            TestCase {
                p: policy,
                pw: pw.into(),
            }

        })
        .collect();

    part_one(&values);
    part_two(&values);

}
