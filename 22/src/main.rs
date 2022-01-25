use std::collections::HashSet;
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

fn resolve_simple_winner(
    deck_one: &mut Vec<usize>,
    deck_two: &mut Vec<usize>,
    one: usize,
    two: usize,
) {
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

fn play_game(mut deck_one: Vec<usize>, mut deck_two: Vec<usize>) -> usize {
    while deck_one.len() > 0 && deck_two.len() > 0 {
        let one = deck_one.remove(0);
        let two = deck_two.remove(0);

        resolve_simple_winner(&mut deck_one, &mut deck_two, one, two);
    }

    if deck_one.len() > 0 {
        calculate_score(&deck_one)
    } else {
        calculate_score(&deck_two)
    }
}

fn play_game_p2(mut deck_one: Vec<usize>, mut deck_two: Vec<usize>) -> (usize, usize) {
    let mut previous_state_one: HashSet<Vec<usize>> = HashSet::new();
    let mut previous_state_two: HashSet<Vec<usize>> = HashSet::new();
    while deck_one.len() > 0 && deck_two.len() > 0 {
        if previous_state_one.contains(&deck_one) && previous_state_two.contains(&deck_two) {
            let score = calculate_score(&deck_one);
            return (score, 0);
        }

        previous_state_one.insert(deck_one.clone());
        previous_state_two.insert(deck_two.clone());

        let one = deck_one.remove(0);
        let two = deck_two.remove(0);

        // println!("{} {}, {:?} {:?}", one, two, deck_one, deck_two);

        if deck_one.len() >= one && deck_two.len() >= two {
            let (_, winner) = play_game_p2(deck_one[0..one].to_vec(), deck_two[0..two].to_vec());
            if winner == 0 {
                deck_one.push(one);
                deck_one.push(two);
            } else if winner == 1 {
                deck_two.push(two);
                deck_two.push(one);
            } else {
                panic!("Unknown winner value from sub game {}", winner);
            }
        } else {
            resolve_simple_winner(&mut deck_one, &mut deck_two, one, two);
        }
    }

    let mut winner_index = 0;
    let winning_deck = if deck_one.len() == 0 {
        winner_index = 1;
        deck_two
    } else {
        deck_one
    };
    (calculate_score(&winning_deck), winner_index)
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

    let score = play_game(deck_one.clone(), deck_two.clone());
    println!("{}", score);

    let score = play_game_p2(deck_one, deck_two);
    println!("{:?}", score);

    Ok(())
}
