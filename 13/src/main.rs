use std::usize::MAX;

use read_input::read_text;

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

    busses.reverse();
    let (highest_offset, _) = busses.first().unwrap();
    let mut timestamp = *highest_offset;
    loop {
        for (ts, bus) in busses.iter().skip(1) {
            if ts % timestamp ==
        }
        timestamp += highest_offset;
    }
}
