use clap::{CommandFactory, Parser, Subcommand};
use json::object;

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
        /// The amount of players
        #[arg(short, long, value_name="PLAYER_COUNT")]
        player_count: usize,

        /// The path to write the game state
        #[arg(short, long="output", default_value="./game_state.json", value_name="OUTPUT_PATH")]
        output_path: PathBuf,

        /// If the flag is present it will print the output instead of saving to a file
        #[arg(short, long, default_value="false")]
        display_json: bool,
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

#[derive(PartialEq)]
enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE,
}

const COLOR_TILE_COUNT: usize = 20;
const PLATE_COUNT: usize = 7;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::New { output_path, player_count, display_json }) => {
            println!("Generating a new game state in {:?}", output_path);
            let mut tiles = vec![];
            for _ in 0..COLOR_TILE_COUNT {
                tiles.push(Tile::BLUE);
                tiles.push(Tile::YELLOW);
                tiles.push(Tile::RED);
                tiles.push(Tile::BLACK);
                tiles.push(Tile::WHITE);
            }

            let mut plates: Vec<[Tile; 4]> = Vec::new();
            for _ in 0..PLATE_COUNT {
                plates.push(generate_plate(&mut tiles));
            }

            let mut jsonified_plates: Vec<json::JsonValue> = Vec::new();
            for plate in plates { jsonified_plates.push(plate_to_json(plate)); }

            let mut player_states = Vec::new();
            for i in 0..*player_count {
                player_states.push(object! {
                    playerId: i,
                    points: 0,
                    scoreGrid: [
                        "00000",
                        "00000",
                        "00000",
                        "00000",
                        "00000",
                    ],
                    prepGrid: [ {}, {}, {}, {}, {} ]
                });
            }

            let output = object! {
                gameState: {
                    availablePlates: jsonified_plates,
                    discardPile: { penaltyTaken: false },
                },
                playerStates: player_states,
            };

            if *display_json {
                println!("{}", output);
            } else {
                std::fs::write(output_path, output.dump()).expect("Error writing file");
            }
        },
        Some(Command::Play { state, r#move, output_path }) => {
            println!("Generating a new game state from {:?} using move {:?} and outputing to {:?}", state, r#move, output_path);
            todo!("Unimplemented");
        },
        Some(Command::Simulate { bot_paths, display }) => {
            if bot_paths.len() < 2 { panic!("Less than two bots have been inputed"); }
            if bot_paths.len() > 4 { panic!("A game cannot have more than 4 players"); }
            println!("Pitting bots {:?} against each other", bot_paths);
            if *display {
                println!("displaying...");
            }
            todo!("Unimplemented");
        }
        None => { Cli::command().print_help().unwrap(); },
    }
}

fn generate_plate(available_tiles: &mut Vec<Tile>) -> [Tile; 4] {
    [
        available_tiles.swap_remove(rand::random_range(0..available_tiles.len())),
        available_tiles.swap_remove(rand::random_range(0..available_tiles.len())),
        available_tiles.swap_remove(rand::random_range(0..available_tiles.len())),
        available_tiles.swap_remove(rand::random_range(0..available_tiles.len())),
    ]
}

fn plate_to_json(plate: [Tile; 4]) -> json::JsonValue {
    object! {
        blue: count(&plate, Tile::BLUE),
        yellow: count(&plate, Tile::YELLOW),
        red: count(&plate, Tile::RED),
        black: count(&plate, Tile::BLACK),
        white: count(&plate, Tile::WHITE),
    }
}

fn count(object: &[Tile; 4], tile: Tile) -> usize {
    let mut count = 0;
    for obj in object {
        if obj == &tile { count += 1}
    }
    count
}
