use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("21/input.txt")?;

    for line in text.lines() {
        let mut iter = line.split(" (");
        let ingredients = iter.next().unwrap().split(" ").collect::<Vec<&str>>();
        let allergens = iter
            .next()
            .unwrap()
            .replace(")", "")
            .replace("contains ", "");
        let allergens = allergens.split(", ").collect::<Vec<&str>>();

        println!("{:?} {:?}", ingredients, allergens);
    }

    Ok(())
}
