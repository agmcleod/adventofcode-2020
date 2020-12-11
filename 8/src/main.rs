use std::collections::HashSet;

use read_input::read_text;

fn main() {
    let text = read_text("8/input.txt").unwrap();

    let mut commands = Vec::new();

    for line in text.lines() {
        let mut iter = line.split(" ");
        let cmd = iter.next().unwrap();
        let amt: i32 = iter.next().unwrap().parse().unwrap();

        commands.push((cmd, amt));
    }

    let mut index: i32 = 0;
    let mut acc = 0;
    let mut used_indexes = HashSet::new();

    loop {
        let (cmd, n) = commands.get(index as usize).unwrap();

        match *cmd {
            "nop" => {
                index += 1;
            },
            "acc" => {
                acc += *n;
                index += 1;
            },
            "jmp" => {
                index += *n;
            },
            _ => panic!("unrecognized command {}", cmd)
        }

        if used_indexes.contains(&index) {
            break
        }
        used_indexes.insert(index);
    }

    println!("{}", acc);
}
