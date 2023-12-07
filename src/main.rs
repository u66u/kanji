extern crate rand;
extern crate serde;
extern crate colored;
extern crate crossterm;

use rand::Rng;
use serde::Deserialize;
use colored::*;
use crossterm::{
    event::{ read, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode },
};
use std::io;

#[derive(Deserialize, Debug)]
struct Kanji {
    category: String,
    character: String,
    onyomi: String,
    kunyomi: String,
    meaning: String,
}

fn main() -> std::io::Result<()> {
    let kanji_list: Vec<Kanji> = serde_json::from_str(include_str!("kanji.json")).unwrap();
    let mut rng = rand::thread_rng();
    let kanji = &kanji_list[rng.gen_range(0..kanji_list.len())];

    println!("{}", kanji.character.repeat(3));
    println!("{}", kanji.character.repeat(3));
    println!("{}", kanji.character.repeat(3));
    println!("Onyomi: {}", kanji.onyomi);
    println!("Kunyomi: {}", kanji.kunyomi);

    println!("Do you know this kanji? (Y/N)");

    enable_raw_mode()?;
    let mut response = ' ';
    while let Event::Key(event) = read()? {
        match event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                response = 'y';
                break;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                response = 'n';
                break;
            }
            _ => {
                continue;
            }
        }
    }
    disable_raw_mode()?;

    match response {
        'y' => {
            println!("What is the meaning of this kanji?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().eq_ignore_ascii_case(&kanji.meaning) {
                println!("{}", "Correct!".green());
            } else {
                println!("{}", "Incorrect.".red());
                println!("The correct meaning is: {}", kanji.meaning);
            }
        }
        'n' => {
            println!("The meaning of this kanji is: {}", kanji.meaning);
        }
        _ => {}
    }

    Ok(())
}
