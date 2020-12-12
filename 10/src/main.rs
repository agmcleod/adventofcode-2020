use read_input::read_text;

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut adapters: Vec<usize> = text.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    adapters.sort();

    let mut ones = 1;
    let mut threes = 1;
    for a in adapters.windows(2) {
        let diff = a[1] - a[0];
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    println!("{} * {} = {}", ones, threes, ones * threes);
}
