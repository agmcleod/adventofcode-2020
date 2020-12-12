use read_input::read_text;

fn p2(adapters: &Vec<usize>) {
    let mut work: Vec<usize> = vec![0];

    let mut count = 0;

    println!("{:?}", adapters);

    while work.len() > 0 {
        let index = work.pop().unwrap();
        if index == adapters.len() - 1 {
            count += 1;
            continue;
        }
        let n = adapters[index];
        let mut next_indexes = (index + 1..index + 3)
            .filter(|i| {
                if *i >= adapters.len() {
                    return false;
                }

                adapters[*i] - n <= 3
            })
            .collect();

        work.append(&mut next_indexes);
    }

    println!("{}", count);
}

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

    p2(&adapters);
}
