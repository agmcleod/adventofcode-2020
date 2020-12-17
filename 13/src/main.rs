use std::usize::MAX;

use read_input::read_text;

fn main() {
    let text = read_text("13/input.txt").unwrap();
    let mut lines = text.lines();
    let arrival_time: usize = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap();
    let busses: Vec<usize> = busses
        .split(",")
        .filter(|v| *v != "x")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut least_wait_route = 0;
    let mut least_time = MAX;
    for bus in &busses {
        let amt = bus - (arrival_time % bus);
        println!("{} - {} % {} = {}", bus, arrival_time, bus, amt);
        if amt < least_time {
            least_time = amt;
            least_wait_route = *bus;
        }
    }

    println!("{}", least_wait_route * least_time);
}
