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
    selected_category: Option<Vec<String>>,
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

    let mut categories: Vec<String> = Vec::new();
    let mut idx = 1;

    // selecting categories
    while idx < args.len() {
        match args[idx].as_str() {
            "-n" | "--jlptn" if idx + 1 < args.len() => {
                let value = &args[idx + 1];
                match value.as_str() {
                    "all" => {
                        categories = (1..=5).map(|n| format!("jlptn{}", n)).collect();
                    }
                    range if range.contains('-') => {
                        let parts: Vec<&str> = range.split('-').collect();
                        if parts.len() == 2 {
                            let start = parts[0].parse::<u32>().expect("Invalid range start");
                            let end = parts[1].parse::<u32>().expect("Invalid range end");
                            categories.extend((start..=end).map(|n| format!("jlptn{}", n)));
                        } else {
                            eprintln!("Invalid range format. Format should be like '-n 1-3'.");
                            return Ok(());
                        }
                    }
                    _ => {
                        value.split(',').for_each(|num| {
                            categories.push(format!("jlptn{}", num.trim()));
                        });
                    }
                }
                idx += 2;
            }
            _ => {
                idx += 1;
            }
        }
    }

    let mut file = File::open("kanji.json")?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;
    let mut config: Config = serde_json::from_str(&json_content).unwrap();

    if !categories.is_empty() {
        config.selected_category = Some(categories.clone());
        fs::write("kanji.json", serde_json::to_string_pretty(&config)?)?;
    }

    let filtered_kanji: Vec<Kanji> = match &config.selected_category {
        Some(cats) =>
            config.kanji
                .iter()
                .filter(|k| cats.contains(&k.category))
                .cloned()
                .collect(),
        None => config.kanji.clone(), // If categories are empty display all kanji
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

        let meanings: Vec<&str> = random_kanji.meaning.split(',').map(str::trim).collect();
        let input_trimmed = input_guess.trim();

        if meanings.iter().any(|&meaning| input_trimmed.eq_ignore_ascii_case(meaning)) {
            println!("{}", "Correct!".green());
            println!("Meanings include: {}", random_kanji.meaning);
        } else {
            println!("{}", "Incorrect.".red());
            println!("The meaning of this kanji is: {}", random_kanji.meaning);
        }
    } else if response == 'n' {
        println!("The meaning of this kanji is: {}", random_kanji.meaning);
    }

    Ok(())
}
