# Rust Wordle Bot

## What is Wordle?

[Wordle](https://www.nytimes.com/games/wordle/index.html) is a word puzzle game where the objective is 
to guess a secret five-letter word within six attempts.
After each guess, the game provides feedback in the 
form of colored tiles or symbols indicating how close 
your guess is to the secret word:

- **Green:** The letter is correct and in the right 
  position.
- **Yellow:** The letter is in the word, but in the 
  wrong position.
- **Gray:** The letter is not in the word at all.

The goal is to correctly guess the word using as 
few attempts as possible by cleverly interpreting 
the clues provided after each guess.

## Usage
1. Use Cargo to build the project

       cargo build
2. run one of the subcommands

       ./wordle-rust-bot play
       ./wordle-rust-bot assist wordle.txt
       ./wordle-rust-bot batch wordle.txt wordle-answers.txt

**Usage:** `ẁordle-rust-bot <COMMAND>`

**Commands:**
1. **assist** `<WORD_FILE>`: Help with a game you are playing. 
   The program will ask you to enter your guesses 
   and the result you got, and from that will figure
   out candidate words to guess.
   - `<ẀORDF_FILE>`: The list of all allowed five-letter words
2. **batch** `<WORD_FILE>` `<SOLUTION_FILE>`:
   Runs a batch of games to gather data about the
   algorithm’s performance.
   - `<WORD_FILE>`: The list of all allowed five-letter words.
   - `<SOLUTION_FILE>`:  The list of words to use as solutions for the test games. 
3. **play** `<WORD_FILE>`: Play a normal game of wordle against this program.
   - `<WORD_FILE>`:  The list of all allowed five-letter words.

## Word Lists
This repository includes several files to start playing and
testing immediately:
1. `wordle.txt`: All the words that the [New York Times
   Wordle](https://www.nytimes.com/games/wordle/index.html) Implementation will accept as guesses.
2. `wordle-answers.txt`: The answers that will come up
   on the New York Times Wordle. Using this for `assist`
   will result in an artificial performance increase
   (*i.e.* is cheating), because this list does not come
   close to including all common five-letter words.
3. `lordle.txt`: All five-letter words from 
   *The Lord of the Rings*, meant to resemble the 
   data set for the *[Digital Tolkien Project](https://digitaltolkien.com/)’s [Lordle](https://lordle.digitaltolkien.com/)*.
   This list is not well curated and may contain some
   hyphenation artefacts.