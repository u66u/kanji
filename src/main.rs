extern crate rand;
extern crate serde;
extern crate colored;
extern crate crossterm;
extern crate open;

use std::fs::File;
use std::io::Write;
use rand::Rng;
use serde::Deserialize;
use colored::*;
use crossterm::{ event::{ read, Event, KeyCode }, terminal::{ disable_raw_mode, enable_raw_mode } };
use std::io;

#[derive(Deserialize, Debug)]
struct Kanji {
    category: String,
    character: String,
    onyomi: String,
    kunyomi: String,
    meaning: String,
}

fn show_kanji(character: &str) -> std::io::Result<()> {
    let html_content =
        format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Kanji Display</title>
                <style>
                    body {{ text-align: center; margin-top: 50px; }}
                    .kanji {{ font-size: 18em; }}
                </style>
            </head>
            <body>
                <div class="kanji">{}</div>
            </body>
            </html>
        "#, character);

    let mut file = File::create("kanji_display.html")?;
    file.write_all(html_content.as_bytes())?;

    open::that("kanji_display.html")
}

fn main() -> std::io::Result<()> {
    let kanji_list: Vec<Kanji> = serde_json::from_str(include_str!("kanji.json")).unwrap();
    let mut rng = rand::thread_rng();
    let kanji = &kanji_list[rng.gen_range(0..kanji_list.len())];

    println!("Kanji: {}", kanji.character);
    println!("Onyomi: {}", kanji.onyomi);
    println!("Kunyomi: {}", kanji.kunyomi);

    println!("Do you know this kanji? (Y/N). Press O to open in full screen");

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
            KeyCode::Char('o') | KeyCode::Char('O') => {
                show_kanji(&kanji.character)?;
                continue;
            }
            _ => {
                continue;
            }
        }
    }
    disable_raw_mode()?;

    if response == 'y' {
        println!("What is the meaning of this kanji?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case(&kanji.meaning) {
            println!("{}", "Correct!".green());
        } else {
            println!("{}", "Incorrect.".red());
            println!("The correct meaning is: {}", kanji.meaning);
        }
    } else if response == 'n' {
        println!("The meaning of this kanji is: {}", kanji.meaning);
    }

    Ok(())
}
