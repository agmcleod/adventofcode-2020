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

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
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
        "{} {}",
        passports.len(),
        passports.iter().fold(0, |acc, passport| {
            if passport.is_valid() {
                acc + 1
            } else {
                acc + 0
            }
        })
    );
}
