use read_input::read_text;

fn sum_numbers_to_2020(nums: &Vec<u32>, index_count: usize) -> u32 {
    let mut indexes: Vec<usize> = (0..index_count).collect();

    loop {
        // just a safety
        if indexes[index_count - 1] >= nums.len() {
            break
        }

        if indexes.iter().fold(0, |sum, idx| {
            nums[*idx] + sum
        }) == 2020 {
            break
        }

        let mut incr_index = index_count - 1;
        loop {
            indexes[incr_index] += 1;
            if indexes[incr_index] >= nums.len() {
                if incr_index == 0 {
                    panic!("Could not find a combination. indexes: {:?}", indexes);
                }
                incr_index -= 1;
            } else {
                break
            }
        }

        for i in (incr_index + 1..indexes.len()) {
            indexes[i] = indexes[i - 1] + 1;
        }
    }

    indexes.iter().fold(1, |prod, idx| {
        nums[*idx] * prod
    })
}


fn main() {
    let text = read_text("1/input.txt").unwrap();

    let mut nums: Vec<u32> = text.lines().filter(|v| {
        *v != ""
    }).map(|v| v.parse().expect("Couldn't parse number")).collect();

    nums.sort();

    println!("{}", sum_numbers_to_2020(&nums, 2));
    println!("{}", sum_numbers_to_2020(&nums, 3));
}
