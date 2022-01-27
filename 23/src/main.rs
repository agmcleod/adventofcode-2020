use std::collections::HashMap;
use std::fmt::Debug;

struct Link {
    left: u32,
    right: u32,
}

impl Link {
    fn new(left: u32, right: u32) -> Self {
        if left == right {
            panic!("Left & right cannot be the same {} {}", left, right);
        }
        Link { left, right }
    }
}

impl Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("L: {} R: {}", self.left, self.right))
    }
}

struct Cups {
    data: HashMap<u32, Link>,
}

impl Cups {
    fn new() -> Self {
        Cups {
            data: HashMap::new(),
        }
    }

    fn has(&self, cup: &u32) -> bool {
        self.data.contains_key(cup)
    }

    fn insert_after(&mut self, target_cup: &u32, insert_cup: u32) {
        if let Some(mut target) = self.data.get_mut(target_cup) {
            let right_of_target = target.right;
            target.right = insert_cup;
            let right_of_target_link = self.data.get_mut(&right_of_target).unwrap();
            right_of_target_link.left = insert_cup;
            self.data
                .insert(insert_cup, Link::new(*target_cup, right_of_target));
        } else {
            panic!(
                "Could not insert after {}, as it is not in the data map",
                target_cup
            );
        }
    }

    fn remove(&mut self, cup: &u32) -> Option<u32> {
        if self.data.contains_key(cup) {
            let link = self.data.remove(cup).unwrap();

            if let Some(left) = self.data.get_mut(&link.left) {
                left.right = link.right.clone();
            } else {
                panic!("Could not find left link for cup: {} {:?}", cup, link);
            }

            if let Some(right) = self.data.get_mut(&link.right) {
                right.left = link.left.clone();
            } else {
                panic!("Could not find right link for cup: {} {:?}", cup, link);
            }

            return Some(cup.to_owned());
        }

        None
    }

    fn iter<'a>(&'a self, start_from: &'a u32) -> CupsIterator<'a> {
        CupsIterator {
            cursor: start_from,
            data: &self.data,
        }
    }
}

struct CupsIterator<'a> {
    cursor: &'a u32,
    data: &'a HashMap<u32, Link>,
}

impl<'a> Iterator for CupsIterator<'a> {
    type Item = &'a u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.cursor);
        if let Some(v) = result {
            self.cursor = &v.right;
            Some(&v.right)
        } else {
            None
        }
    }
}

fn next_cup(cup: u32, highest_cup: &u32) -> u32 {
    if cup == 1 {
        *highest_cup
    } else {
        cup - 1
    }
}

fn get_destination(cups: &Cups, current_cup: &u32, highest_cup: &u32) -> u32 {
    let mut destination_cup = next_cup(*current_cup, highest_cup);
    while !cups.has(&destination_cup) {
        destination_cup = next_cup(destination_cup, highest_cup);
    }

    destination_cup
}

fn take_turn(cups: &mut Cups, current_cup: &mut u32, highest_cup: u32) {
    let mut picked_up = Vec::with_capacity(3);
    // pick up the next 3 cups, remove them from the input
    let mut to_remove = if let Some(link) = cups.data.get(&current_cup) {
        link.right
    } else {
        panic!("Could not find current cup: {}", current_cup);
    };
    for _ in 0..3 {
        let next_remove = cups.data.get(&to_remove).unwrap().right;
        if let Some(removed) = cups.remove(&to_remove) {
            picked_up.push(removed);
        }
        to_remove = next_remove;
    }

    // find the destination
    let mut destination = get_destination(&cups, current_cup, &highest_cup);
    for picked_up_cup in &picked_up {
        cups.insert_after(&destination, *picked_up_cup);
        destination = *picked_up_cup;
    }

    *current_cup = *cups.iter(&current_cup).next().unwrap();
}

fn main() {
    let mut highest_cup = 0;
    let input = "459672813"
        .chars()
        .map(|v| {
            let n = v.to_digit(10).unwrap();
            highest_cup = highest_cup.max(n);
            n
        })
        .collect::<Vec<u32>>();

    let mut cups = Cups::new();
    let mut cups_p2 = Cups::new();

    for (i, cup) in input.iter().enumerate() {
        let left = if i == 0 {
            input[input.len() - 1]
        } else {
            input[i - 1]
        };
        let right = input[(i + 1) % input.len()];
        cups.data.insert(*cup, Link::new(left, right));
        cups_p2.data.insert(*cup, Link::new(left, right));
    }

    let mut current_cup = input[0];

    for _ in 0..100 {
        take_turn(&mut cups, &mut current_cup, highest_cup);
    }

    // create p1 answer
    let output: String = cups
        .iter(&1)
        .take_while(|v| **v != 1)
        .map(|v| v.to_string())
        .collect();
    println!("{:?}", output);

    // p2
    let mut insert_target = input[input.len() - 1];
    for n in (highest_cup + 1)..=1_000_000 {
        cups_p2.insert_after(&insert_target, n);
        insert_target = n;
    }

    let mut current_cup = input[0];
    for _ in 0..10_000_000 {
        take_turn(&mut cups_p2, &mut current_cup, 10_000_000);
    }

    let mut iter = cups_p2.iter(&1);
    let one = *iter.next().unwrap() as usize;
    let two = *iter.next().unwrap() as usize;
    println!("{}", one * two);
}
