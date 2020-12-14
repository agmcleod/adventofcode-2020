use std::collections::HashSet;

use read_input::read_text;

fn run_program(commands: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut index: i32 = 0;
    let mut acc = 0;
    let mut used_indexes = HashSet::new();
    let mut exited_successfuly = false;

    loop {
        let (cmd, n) = commands.get(index as usize).unwrap();

        match *cmd {
            "nop" => {
                index += 1;
            }
            "acc" => {
                acc += *n;
                index += 1;
            }
            "jmp" => {
                index += *n;
            }
            _ => panic!("unrecognized command {}", cmd),
        }

        if used_indexes.contains(&index) {
            break;
        }
        used_indexes.insert(index);

        if index as usize >= commands.len() {
            exited_successfuly = true;
            break;
        }
    }

    (acc, exited_successfuly)
}

fn main() {
    let text = read_text("8/input.txt").unwrap();

    let mut commands = Vec::new();

    for line in text.lines() {
        let mut iter = line.split(" ");
        let cmd = iter.next().unwrap();
        let amt: i32 = iter.next().unwrap().parse().unwrap();

        commands.push((cmd, amt));
    }

    let (acc, _) = run_program(&commands);
    println!("{}", acc);

    let mut index = 0;
    loop {
        let (cmd, amt) = commands.get(index).unwrap();

        if *cmd == "nop" || *cmd == "jmp" {
            let mut commands = commands.clone();
            let mut should_run = false;
            if *cmd == "nop" && *amt != 0 {
                commands[index].0 = "jmp";
                should_run = true;
            } else if *cmd == "jmp" {
                commands[index].0 = "nop";
                should_run = true;
            }

            if should_run {
                let (acc, exited) = run_program(&commands);
                if exited {
                    println!("{}", acc);
                    break;
                }
            }
        }

        index += 1;
        if index == commands.len() {
            panic!("Couldnt find answer");
        }
    }
}
