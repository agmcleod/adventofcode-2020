use std::usize::MAX;

use read_input::read_text;

fn get_lcm(x: usize, y: usize) -> usize {
    x * y / gcd_two_numbers(x, y)
}

fn gcd_two_numbers(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn main() {
    let text = read_text("13/input.txt").unwrap();
    let mut lines = text.lines();
    let arrival_time: usize = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap();
    let busses: Vec<(usize, usize)> = busses
        .split(",")
        .enumerate()
        .filter(|(_i, v)| *v != "x")
        .map(|(i, n)| (i, n.parse::<usize>().unwrap()))
        .collect();

    let mut least_wait_route = 0;
    let mut least_time = MAX;
    for (_, bus) in &busses {
        let amt = bus - (arrival_time % bus);
        if amt < least_time {
            least_time = amt;
            least_wait_route = *bus;
        }
    }

    println!("{}", least_wait_route * least_time);

    let (_offset, bus) = busses[0];
    let mut base_num = bus;
    let (next_offset, next_bus) = busses[1];
    loop {
        if (base_num + next_offset) % next_bus == 0 {
            break;
        }
        base_num += bus;
    }

    let mut lcm = get_lcm(busses[0].1, busses[1].1);
    let mut iteration = 1;
    // we start at 3rd bus, as 1st & 2nd are covered by the above loop
    let mut target_index = 2;
    loop {
        let (offset, bus) = busses[target_index];
        let product = base_num + (lcm * iteration);
        if (product + offset) % bus == 0 {
            base_num = product;
            iteration = 1;
            target_index += 1;
            if target_index == busses.len() {
                break;
            }

            lcm = (0..target_index).fold(1, |lcm, index| get_lcm(lcm, busses[index].1));
        } else {
            iteration += 1;
        }
    }
    println!("{}", base_num);
}
