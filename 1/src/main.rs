use read_input::read_text;

fn main() {
    let text = read_text("1/input.txt").unwrap();

    let mut nums: Vec<u32> = text.lines().filter(|v| {
        *v != ""
    }).map(|v| v.parse().expect("Couldn't parse number")).collect();

    nums.sort();

    let mut left_i = 0;
    let mut right_i = 1;

    loop {
        // just a safety
        if right_i >= nums.len() {
            break
        }

        if nums[left_i] + nums[right_i] == 2020 {
            break
        }

        right_i += 1;
        if right_i >= nums.len() {
            left_i += 1;
            right_i = left_i + 1;
            if left_i >= nums.len() {
                panic!("couldnt find the number");
            }
        }
    }

    if left_i >= nums.len() || right_i >= nums.len() {
        panic!("Couldnt finish program: {} {}", left_i, right_i);
    }

    println!("{}", nums[left_i] * nums[right_i]);
}
