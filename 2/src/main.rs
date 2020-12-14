use read_input::read_text;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
    pw: String,
}

fn get_policies_from_input(text: &String) -> Vec<Policy> {
    let mut policies = Vec::new();

    for line in text.lines() {
        let mut itr = line.split(": ");

        let mut policy = Policy {
            min: 0,
            max: 0,
            letter: ' ',
            pw: "".to_string(),
        };

        if let Some(first) = itr.next() {
            let mut first_itr = first.split(" ");
            if let Some(ranges) = first_itr.next() {
                let nums: Vec<usize> = ranges
                    .split("-")
                    .map(|number| {
                        number
                            .parse()
                            .expect(&format!("Could not parse {}", number))
                    })
                    .collect();
                policy.min = nums[0];
                policy.max = nums[1];
            }

            if let Some(letter) = first_itr.next() {
                policy.letter = letter.chars().next().unwrap();
            }
        }

        if let Some(second) = itr.next() {
            policy.pw.push_str(second);
        }

        policies.push(policy);
    }

    policies
}

fn main() {
    let text = read_text("2/input.txt").unwrap();

    let policies = get_policies_from_input(&text);

    let mut correct_count = 0;
    let mut correct_count_p2 = 0;

    for policy in &policies {
        let mut count = 0;
        let mut p2_count = 0;
        for (i, letter) in policy.pw.chars().enumerate() {
            if letter == policy.letter {
                count += 1;
                if i + 1 == policy.min || i + 1 == policy.max {
                    p2_count += 1;
                }
            }
        }

        if count <= policy.max && count >= policy.min {
            correct_count += 1;
        }

        if p2_count == 1 {
            correct_count_p2 += 1;
        }
    }

    println!("{}", correct_count);
    println!("{}", correct_count_p2);
}
