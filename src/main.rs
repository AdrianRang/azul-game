use clap::{CommandFactory, Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Generates a fresh game state
    New {
        /// The path to write the game state
        #[arg(short, long="output", default_value="./game_state.json", value_name="OUTPUT_PATH")]
        output_path: PathBuf,
    },
    /// Generates a new game state from the original state and move to be made
    Play {
        /// The path to the original state
        #[arg(short, long, value_name="PATH")]
        state: PathBuf,
        /// The path to the move to be made
        #[arg(short, long, value_name="PATH")]
        r#move: PathBuf,
        /// The path to write the new game state
        #[arg(short, long="output", default_value="./game_state.json", value_name="OUTPUT_PATH")]
        output_path: PathBuf,
    },
    /// Make two or more bots play each other
    Simulate {
        /// The paths to the bots' executables
        #[arg(short='b', long="bot", value_name="PATH")]
        bot_paths: Vec<PathBuf>,

        #[arg(short, long, default_value="false")]
        display: bool
    }
}

enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::New { output_path }) => {
            println!("Generating a new game state in {:?}", output_path);
        },
        Some(Command::Play { state, r#move, output_path }) => {
            println!("Generating a new game state from {:?} using move {:?} and outputing to {:?}", state, r#move, output_path);
        },
        Some(Command::Simulate { bot_paths, display }) => {
            if bot_paths.len() < 2 { panic!("Less than two bots have been inputed"); }
            if bot_paths.len() > 4 { panic!("A game cannot have more than 4 players"); }
            println!("Pitting bots {:?} against each other", bot_paths);
            if *display {
                println!("displaying...");
            }
        }
        None => { Cli::command().print_help().unwrap(); },
    }
}
