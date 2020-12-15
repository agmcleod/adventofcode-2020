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

fn get_variations_in_set(set: &Vec<usize>) -> usize {
    let mut work: Vec<usize> = vec![set.len() - 1];

    let mut count = 0;

    while work.len() > 0 {
        let index = work.pop().unwrap();
        if index == 0 {
            count += 1;
            continue;
        }
        let mut next_indexes = get_next_indexes(set, index);

        work.append(&mut next_indexes);
    }

    count
}

fn p2(adapters: &Vec<usize>) {
    let mut sets = Vec::new();
    let mut set = Vec::new();
    let window_count = adapters.windows(2).count();
    for (i, pair) in adapters.windows(2).enumerate() {
        set.push(pair[0]);
        let diff = pair[1] - pair[0];
        if diff == 3 {
            sets.push(set);
            set = Vec::new();
        }

        if i == window_count - 1 {
            set.push(pair[1]);
        }
    }
    sets.push(set);

    println!("{}", sets.iter().fold(1, |product, set| {
        product * get_variations_in_set(set)
    }));
}

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut adapters: Vec<usize> = text.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);

    let mut ones = 0;
    let mut threes = 1;
    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    println!("{} * {} = {}", ones, threes, ones * threes);

    p2(&adapters);
}
