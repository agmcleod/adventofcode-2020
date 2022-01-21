use std::io::Result;

use read_input::read_text;

fn calculate_score(deck: &Vec<usize>) -> usize {
    let mut product = deck.len();
    deck.iter().fold(0, |sum, card| {
        let new_sum = sum + *card * product;
        product -= 1;
        new_sum
    })
}

fn play_game(mut deck_one: Vec<usize>, mut deck_two: Vec<usize>) -> usize {
    while deck_one.len() > 0 && deck_two.len() > 0 {
        let one = deck_one.remove(0);
        let two = deck_two.remove(0);

        if one > two {
            deck_one.push(one);
            deck_one.push(two);
        } else if two > one {
            deck_two.push(two);
            deck_two.push(one);
        } else {
            panic!("Values were equal: {} {}", one, two);
        }
    }

    if deck_one.len() > 0 {
        calculate_score(&deck_one)
    } else {
        calculate_score(&deck_two)
    }
}

fn main() -> Result<()> {
    let text = read_text("22/input.txt")?;

    let mut deck_one = Vec::new();
    let mut deck_two = Vec::new();

    let mut adding_to_p1 = true;
    for line in text.lines() {
        if line == "" || line == "Player 1:" {
            continue;
        } else if line == "Player 2:" {
            adding_to_p1 = false;
        } else {
            let n: usize = line
                .parse()
                .map_err(|_err| {
                    println!("failed to parse num on {}", line);
                })
                .unwrap();
            if adding_to_p1 {
                deck_one.push(n);
            } else {
                deck_two.push(n);
            }
        }
    }

    let score = play_game(deck_one, deck_two);
    println!("{}", score);

    Ok(())
}
