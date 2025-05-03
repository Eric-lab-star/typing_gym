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

    // quit
    quit: bool,
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
    let file: &Path;
    match level {
        Level::Hard => {
            file = Path::new("levels/hard.txt");
            println!(
        r#"Hard selected!
type given sentences correctly.
        "#)
        },
        Level::Normal => { 
            file = Path::new("levels/normal.txt");
            println!(
        r#"Normal selected!
type given sentences correctly.
        "#)
        },
        Level::Easy => {
            file = Path::new("levels/easy.txt");
            println!(
        r#"Easy selected!
type given words correctly.
        "#)
        }
    }

    let buffreader = File::open(file)
        .map_err(|e|format!("Failed to open file\n {}",e))?;
    let reader = io::BufReader::new(buffreader);
    let mut score = 0;
    for line in reader.lines() {
        let challenge = line.unwrap();
        
        println!("{}",challenge);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).map_err(|e|e.to_string())?;
        if challenge == buffer.trim() {
            score += 1;
            println!(" {}", score);
        } else {
            println!("❌ {}", score);
        }
    }

    println!("Total Score: {}", score);
    Ok(())
}

