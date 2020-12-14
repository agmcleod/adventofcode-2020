use read_input::read_text;

fn get_next_indexes(adapters: &Vec<usize>, index: usize) -> Vec<usize> {
    let n = adapters[index];
    let index = index as i32;
    (index - 3..=index - 1)
        .rev()
        .filter(|i| {
            if *i < 0 {
                return false;
            }

            n - adapters[*i as usize] <= 3
        })
        .map(|i| i as usize)
        .collect()
}

fn p2(adapters: &Vec<usize>) {
    let mut work: Vec<usize> = vec![adapters.len() - 1];

    let mut count = 0;

    while work.len() > 0 {
        let index = work.pop().unwrap();
        if index == 0 {
            count += 1;
            continue;
        }
        let mut next_indexes = get_next_indexes(adapters, index);

        work.append(&mut next_indexes);
    }

    println!("{}", count);
}

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut adapters: Vec<usize> = text.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);

    let mut ones = 0;
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
