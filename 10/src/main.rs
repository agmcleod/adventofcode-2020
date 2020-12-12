use read_input::read_text;

fn p2(adapters: &Vec<usize>) {
    let mut permutations: Vec<Vec<usize>> = vec![vec![0]];

    let mut count = 0;

    while permutations.len() > 0 {
        let mut permutations_to_ditch = Vec::new();
        let mut permutations_to_add = Vec::new();
        for (p_index, permutation) in permutations.iter_mut().enumerate() {
            let index = *permutation.last().unwrap();
            if index == adapters.len() - 1 {
                count += 1;
                permutations_to_ditch.push(p_index);
                continue;
            }
            let n = adapters[index];
            let next_indexes: Vec<usize> = (index + 1..=index + 3)
                .filter(|i| {
                    if *i >= adapters.len() {
                        return false;
                    }

                    adapters[*i] - n <= 3
                })
                .collect();

            if next_indexes.len() == 0 {
                permutations_to_ditch.push(p_index);
            } else {
                for i in next_indexes.iter().skip(1) {
                    let mut perm = permutation.clone();
                    perm.push(*i);
                    permutations_to_add.push(perm);
                }
                permutation.push(next_indexes[0]);
            }
        }

        for (offset_index, index_to_remove) in permutations_to_ditch.iter().enumerate() {
            permutations.remove(*index_to_remove - offset_index);
        }

        permutations.append(&mut permutations_to_add);
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
