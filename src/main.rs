mod pattern;
mod word;
mod game;

use crate::word::*;
use clap::{Parser, Subcommand};
use clio::Input;
use std::io::{BufRead, BufReader, Read};
use crate::game::{HelpGame, PlayGame, SimulatedGame};

/// A program to solve wordle for you!
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand
}

#[derive(Subcommand)]
enum SubCommand {
    /// Help with a game you are playing. The program will ask you to enter your guesses
    /// and the result you got, and from that will figure out candidate words to guess.
    Assist {
        /// The list of all allowed five-letter words
        #[clap(value_parser)]
        word_file: Input
    },
    /// Runs a batch of games to gather data about the algorithm’s performance.
    Batch {
        /// The list of all allowed five-letter words
        #[clap(value_parser)]
        word_file: Input,
        /// The list of words to use as solutions for the games.
        #[clap(value_parser)]
        solution_file: Input,
    },
    /// Play a normal game of wordle against this program.
    Play {
        /// The list of all allowed five-letter words
        #[clap(value_parser)]
        word_file: Input,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        SubCommand::Assist {word_file} => {
            run_game(word_file)
        }
        SubCommand::Batch {word_file, solution_file} => {
            full_runs(word_file, solution_file);
        }
        SubCommand::Play {word_file} => {
            play_game(word_file);
        }
    }
}

fn read_file<R: Read>(name: R) -> Vec<Word> {
    let p = BufReader::new(name).lines().map(|line| {
        Word::from_str(&line.unwrap())
    }).collect();
    p
}

fn run_game<R: Read>(word_file: R) {
    let words = read_file(word_file);
    let mut game = HelpGame::new(&words);
    game.run_game();
}


fn full_runs<R: Read>(words_file: R, solutions_file: R) {
    let words = read_file(words_file);
    let solutions = read_file(solutions_file);
    let first_guess = Word::from_str("tears");
    for s in solutions {
        let mut game = SimulatedGame::new(&words, s, first_guess);
        game.run_game();
    }
}

fn play_game<R: Read>(word_file: R) {
    let words = read_file(word_file);
    PlayGame::new(&words).run_game();
}


