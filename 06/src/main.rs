use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct Reply {
    answer_mask: u32, // if question's answer was true, (1 << question - 'a') is set
}

#[derive(Debug, Clone)]
struct Group {
    members: Vec<Reply>,
}

impl Reply {
    fn new() -> Reply {
        Reply {
            answer_mask: 0,
        }
    }

    fn set(&mut self, c: &char) {
        assert!(char::is_ascii_lowercase(c));
        let offset = (*c as u32) - ('a' as u32);
        self.answer_mask = self.answer_mask | (1 << offset);
    }
}

impl Group {
    fn new() -> Group {
        Group {
            members: Vec::new(),
        }
    }

    fn num_any_yesses(&self) -> u32 {
        let mut yesses = 0;
        for i in 0..26 {
            yesses += if self.members.iter().any(|reply| reply.answer_mask & (1 << i) != 0 ) { 1 } else { 0 };
        }
        yesses
    }

    fn num_all_yesses(&self) -> u32 {
        let mut yesses = 0;
        for i in 0..26 {
            yesses += if self.members.iter().all(|reply| reply.answer_mask & (1 << i) != 0 ) { 1 } else { 0 };
        }
        yesses
    }
}

fn part_one(groups: &Vec<Group>) {
    let total: u32 = groups
        .iter()
        .map(|group| group.num_any_yesses())
        .sum();
    println!("{:?}", total);
}

fn part_two(groups: &Vec<Group>) {
    let total: u32 = groups
        .iter()
        .map(|group| group.num_any_yesses())
        .sum();
    println!("{:?}", total);
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);

    let mut groups = Vec::new();
    let mut group = Group::new();
    reader
        .lines()
        .for_each(|maybe_l| {
            let l = maybe_l.unwrap();
            if l.len() == 0 {
                groups.push(group.clone());
                group = Group::new();
            } else {
                // Build a Reply for this line.
                let mut reply = Reply::new();
                l.chars().for_each(|c| {
                    reply.set(&c);
                });
                group.members.push(reply);
            }
        });
    // Pick up last group if we missed it
    if (group.members.len() > 0) {
        groups.push(group.clone());
    }

    //println!("{:?}", groups[0]);
    part_one(&groups);
    part_two(&groups);
}
