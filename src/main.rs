use std::{fs::File, io::{self, BufRead, Write}, path::Path};

use clap::{Parser, Subcommand};

fn main() -> Result<(), String> {
    println!("Welcome to Tying Gym");
    println!("Choose your level");
    println!("hard | normal | easy");
    let command = readline()?;

    let args = shlex::split(command.trim()).unwrap();
    let parsed_cli = Cli::try_parse_from(args).map_err(|e|e.to_string());

    match parsed_cli {
        Ok(cli) => match cli.level {
            Level::Hard => level(Level::Hard)?,
            Level::Normal => level(Level::Normal)?,
            Level::Easy => level(Level::Easy)?,
        },
        Err(e) => println!("{}",e),
    }
    Ok(())
}


#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    // training level
    #[command(subcommand)]
    level: Level,
}

#[derive(Debug, Subcommand)]
enum Level {
    Hard,
    Normal,
    Easy,
}

fn readline() -> Result<String, String> {
    
    write!(std::io::stdout(), "$ ").map_err(|e|e.to_string())?;
    std::io::stdout().flush().map_err(|e|e.to_string())?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(|e|e.to_string())?;
    Ok(buffer.to_lowercase())
}

fn level(level: Level) -> Result<(), String> {
    let level_file = load_level(level)?;

    let level_reader = io::BufReader::new(level_file);

    let mut score = 0;
    
    for line in level_reader.lines() {
        let challenge = line.unwrap();
        
        println!("{challenge}");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).map_err(|e|e.to_string())?;

        if buffer.trim() == "quit" {
            println!("Quiting Game...");
            println!("Score: {score}");
            return Ok(())
        }

        if challenge == buffer.trim() {
            score += 1;
            println!("  {score}");
        } else {
            println!("❌ {score}");
        }
        
    }

    println!("Total Score: {}", score);
    Ok(())
}

fn load_level(level: Level) -> Result<File, String> {
    
    let (message, path) = match level {
        Level::Hard => (
        r#"Hard selected!
type given sentences correctly."#,
        Path::new("levels/hard.txt")),
        Level::Normal => (
        r#"Normal selected!
type given sentences correctly."#,
            Path::new("levels/normal.txt")
        ),
        Level::Easy => (
        r#"Easy selected!
type given words correctly.
        "#,
        Path::new("levels/easy.txt")
        )
    };

    println!("{message}");
    println!("type \'quit\' to quit game");
    File::open(path).map_err(|e|format!("Failed to open file\n {}",e))
}



