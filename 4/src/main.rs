use regex::Regex;

use read_input::read_text;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn field_in_range(valid: &mut bool, value: &String, min: usize, max: usize) {
        if let Ok(value) = value.parse::<usize>() {
            if value < min || value > max {
                *valid = false;
            }
        } else {
            *valid = false;
        }
    }

    fn has_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        let mut valid = true;
        let byr = self.byr.as_ref().unwrap();
        Passport::field_in_range(&mut valid, byr, 1920, 2002);

        let iyr = self.iyr.as_ref().unwrap();
        Passport::field_in_range(&mut valid, iyr, 2010, 2020);

        let eyr = self.eyr.as_ref().unwrap();
        Passport::field_in_range(&mut valid, eyr, 2020, 2030);

        let hgt = self.hgt.as_ref().unwrap();
        if hgt.contains("cm") {
            Passport::field_in_range(&mut valid, &hgt.replace("cm", ""), 150, 193);
        } else if hgt.contains("in") {
            Passport::field_in_range(&mut valid, &hgt.replace("in", ""), 59, 76);
        } else {
            valid = false;
        }

        let re = Regex::new("^#[a-f0-9]{6}$").unwrap();

        if !re.is_match(self.hcl.as_ref().unwrap()) {
            valid = false;
        }

        let ecl = self.ecl.as_ref().unwrap();
        if ecl != "amb"
            && ecl != "blu"
            && ecl != "brn"
            && ecl != "gry"
            && ecl != "grn"
            && ecl != "hzl"
            && ecl != "oth"
        {
            valid = false;
        }

        let pid = self.pid.as_ref().unwrap();
        let re = Regex::new("^\\d{9}$").unwrap();
        if !re.is_match(pid) {
            valid = false;
        }

        valid
    }
}

fn main() {
    let text = read_text("4/input.txt").unwrap();

    let mut passports = Vec::new();

    let mut passport = Passport::new();
    for line in text.lines() {
        if line == "" {
            passports.push(passport);
            passport = Passport::new();
        } else {
            for field in line.split(" ") {
                let details: Vec<&str> = field.split(":").collect();
                match details[0] {
                    "byr" => {
                        passport.byr = Some(details[1].to_string());
                    }
                    "iyr" => {
                        passport.iyr = Some(details[1].to_string());
                    }
                    "eyr" => {
                        passport.eyr = Some(details[1].to_string());
                    }
                    "hgt" => {
                        passport.hgt = Some(details[1].to_string());
                    }
                    "hcl" => {
                        passport.hcl = Some(details[1].to_string());
                    }
                    "ecl" => {
                        passport.ecl = Some(details[1].to_string());
                    }
                    "pid" => {
                        passport.pid = Some(details[1].to_string());
                    }
                    "cid" => {
                        passport.cid = Some(details[1].to_string());
                    }
                    _ => {
                        panic!("Field not understood: {}", details[0]);
                    }
                }
            }
        }
    }

    passports.push(passport);

    println!(
        "{} {} {}",
        passports.len(),
        passports.iter().fold(0, |acc, passport| {
            if passport.has_required_fields() {
                acc + 1
            } else {
                acc + 0
            }
        }),
        passports.iter().fold(0, |acc, passport| {
            if passport.is_valid() {
                acc + 1
            } else {
                acc + 0
            }
        })
    );
}
