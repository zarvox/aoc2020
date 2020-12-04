use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
struct State {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl State {
    fn reset(&mut self) {
        self.byr = None;
        self.iyr = None;
        self.eyr = None;
        self.hgt = None;
        self.hcl = None;
        self.ecl = None;
        self.pid = None;
        self.cid = None;
    }

    fn byr_valid(&self) -> bool {
        match &self.byr {
            None => false,
            Some(byr) => {
                byr.len() == 4 && if let Ok(y) = byr.parse::<u64>() {
                    y >= 1920 && y <= 2002
                } else {
                    false
                }
            },
        }
    }

    fn iyr_valid(&self) -> bool {
        match &self.iyr {
            None => false,
            Some(iyr) => {
                iyr.len() == 4 && if let Ok(y) = iyr.parse::<u64>() {
                    y >= 2010 && y <= 2020
                } else {
                    false
                }
            },
        }
    }

    fn eyr_valid(&self) -> bool {
        match &self.eyr {
            None => false,
            Some(eyr) => {
                eyr.len() == 4 && if let Ok(y) = eyr.parse::<u64>() {
                    y >= 2020 && y <= 2030
                } else {
                    false
                }
            },
        }
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(hgt) => {
                if hgt.len() < 3 {
                    false
                } else {
                    if let Ok(num) = hgt[0..hgt.len()-2].parse::<u64>() {
                        match &hgt[hgt.len()-2..] {
                            "cm" => {
                                num >= 150 && num <= 193
                            },
                            "in" => {
                                num >= 59 && num <= 76
                            },
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
            },
        }
    }

    fn hcl_valid(&self) -> bool {
        match &self.hcl {
            None => false,
            Some(hcl) => {
                hcl.len() == 7 && {
                    let (first, rest) = hcl.split_at(1);
                    first == "#" && rest.chars().all(|x| x.is_ascii_hexdigit())
                }
            }
        }
    }

    fn ecl_valid(&self) -> bool {
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.ecl {
            None => false,
            Some(ecl) => valid_eye_colors.contains(&ecl.as_str()),
        }
    }

    fn pid_valid(&self) -> bool {
        match &self.pid {
            None => false,
            Some(pid) => {
                pid.len() == 9 && pid.chars().all(|x| x.is_ascii_digit())
            },
        }
    }

    fn cid_valid(&self) -> bool {
        true
    }

    fn valid(&self) -> bool {
      self.byr_valid() && self.iyr_valid() && self.eyr_valid() && self.hgt_valid()
          && self.hcl_valid() && self.ecl_valid() && self.pid_valid() && self.cid_valid()
    }
}

fn part_one(state: &Vec<State>) {
    let valid_count = state.iter().filter(|s| {
        s.byr.is_some() &&
        s.iyr.is_some() &&
        s.eyr.is_some() &&
        s.hgt.is_some() &&
        s.hcl.is_some() &&
        s.ecl.is_some() &&
        s.pid.is_some()
    }).count();

    println!("{:?}", valid_count);
}

fn part_two(state: &Vec<State>) {
    let valid_count = state.iter().filter(|s| {
        s.valid()
    }).count();

    println!("{:?}", valid_count);
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);

    let mut build = State {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };
    let mut passports = Vec::new();

    reader
        .lines()
        .for_each(|maybe_l| {
            let l = maybe_l.unwrap();
            if l.len() == 0 {
                passports.push(build.clone());
                build.reset();
            } else {
                l.split(" ").for_each(|field| {
                    if field.starts_with("byr:") {
                        build.byr = Some(field[4..].into());
                    }
                    if field.starts_with("iyr:") {
                        build.iyr = Some(field[4..].into());
                    }
                    if field.starts_with("eyr:") {
                        build.eyr = Some(field[4..].into());
                    }
                    if field.starts_with("hgt:") {
                        build.hgt = Some(field[4..].into());
                    }
                    if field.starts_with("hcl:") {
                        build.hcl = Some(field[4..].into());
                    }
                    if field.starts_with("ecl:") {
                        build.ecl = Some(field[4..].into());
                    }
                    if field.starts_with("pid:") {
                        build.pid = Some(field[4..].into());
                    }
                    if field.starts_with("cid:") {
                        build.cid = Some(field[4..].into());
                    }
                });
            }
        });


    //println!("{:?}", passports[0]);
    part_one(&passports);
    part_two(&passports);
}
