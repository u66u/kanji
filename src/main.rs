extern crate rand;
extern crate serde;
extern crate colored;

use rand::Rng;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct Kanji {
    category: String,
    character: String,
    onyomi: String,
    kunyomi: String,
    meaning: String,
}

fn main() {
    let kanji_list: Vec<Kanji> = serde_json::from_str(include_str!("kanji.json")).unwrap();
    let mut rng = rand::thread_rng();
    let kanji = &kanji_list[rng.gen_range(0..=kanji_list.len())];

    println!("Do you know this kanji? (Y/N) {}", kanji.character);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "yes" {
        println!("What is the meaning of this kanji?");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == kanji.meaning {
            println!("{}", "Correct!".green());
        } else {
            println!("{}", "Incorrect.".red());
            println!("The correct meaning is: {}", kanji.meaning);
        }
    }
}
