extern crate rand;
extern crate serde;
extern crate colored;
extern crate crossterm;
extern crate open;

use std::env::{ self };
use std::fs::{ self, File };
use std::io::{ self, Write, Read };
use rand::Rng;
use serde::{ Deserialize, Serialize };
use colored::*;
use crossterm::{ event::{ read, Event, KeyCode }, terminal::{ disable_raw_mode, enable_raw_mode } };

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    selected_category: Option<String>,
    kanji: Vec<Kanji>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    let args: Vec<String> = env::args().collect();
    let selected_category_arg = args.get(1);

    let mut file = File::open("kanji.json")?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;
    let mut config: Config = serde_json::from_str(&json_content).unwrap();

    if let Some(category) = selected_category_arg {
        if category != "all" {
            let valid_categories: Vec<_> = config.kanji
                .iter()
                .map(|k| k.category.clone())
                .collect();
            if !valid_categories.contains(&category.to_string()) {
                println!(
                    "Invalid category specified. Available categories: {:?}",
                    valid_categories
                );
                return Ok(());
            }
        }
        config.selected_category = if category == "all" {
            None
        } else {
            Some(category.to_string())
        };

        fs::write("kanji.json", serde_json::to_string_pretty(&config)?)?;
    }

    let filtered_kanji = match &config.selected_category {
        Some(category) =>
            config.kanji
                .iter()
                .filter(|k| &k.category == category)
                .cloned()
                .collect::<Vec<Kanji>>(),
        None => config.kanji.clone(),
    };

    let random_kanji = &filtered_kanji[rand::thread_rng().gen_range(0..filtered_kanji.len())];

    println!("Kanji: {}", random_kanji.character);
    println!("Onyomi: {}", random_kanji.onyomi);
    println!("Kunyomi: {}", random_kanji.kunyomi);

    println!("Do you know this kanji? (Y/N). Press Q to quit, or O to open in full screen.");

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
                show_kanji(&random_kanji.character)?;
                continue;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                disable_raw_mode()?;
                return Ok(());
            }
            _ => {
                continue;
            }
        }
    }
    disable_raw_mode()?;

    if response == 'y' {
        println!("What is the meaning of this kanji?");
        let mut input_guess = String::new();
        io::stdin().read_line(&mut input_guess)?;

        if input_guess.trim().eq_ignore_ascii_case(&random_kanji.meaning) {
            println!("{}", "Correct!".green());
        } else {
            println!("{}", "Incorrect.".red());
            println!("The correct meaning is: {}", random_kanji.meaning);
        }
    } else if response == 'n' {
        println!("The meaning of this kanji is: {}", random_kanji.meaning);
    }

    Ok(())
}
