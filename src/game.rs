use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use rand::Rng;
use rayon::prelude::*;
use crate::pattern::{Color, Pattern};
use crate::word::{Word, WORD_LENGTH};

/// Computes the score of a word given a solution. The rules are as follows:
/// 1. All positions where the letters of guess and solution are the same,
///    are marked green.
/// 2. For the other positions in the guess, check if the letter occurs in the
///    solution. If it does, mark erase it and mark it as yellow.
/// 3. All remaining position remain black.
///
/// # Arguments
///
/// * `guess`: The word the user has guessed.
/// * `solution`: The correct solution to the Wordle game.
///
/// returns: `Pattern`: The result of entering the `guess` in a Wordle game with the
/// given `solution`.
///
/// # Examples
///
/// ```
/// assert_equals!(
///     score(Word::from_str("tears"), Word::from_str("bears")),
///     Pattern::from_string("bgggg"));
///  assert_equals!(
///     score(Word::from_str("tears"), Word::from_str("stear")),
///     Pattern::from_string("yyyyy"));
///  assert_equals!(
///     score(Word::from_str("atttt"), Word::from_str("txxxx")),
///     Pattern::from_string("bbybb"));
/// ```
fn score(guess: &Word, solution: &Word) -> Pattern {
    let mut pattern = Pattern::all_black();
    let mut letter_count: HashMap<char, u8> = HashMap::with_capacity(WORD_LENGTH);
    for i in 0..WORD_LENGTH {
        if guess[i] == solution[i] {
            pattern.set(i, Color::Green)
        } else {
            letter_count.entry(solution[i])
                .and_modify(|count|{ *count += 1 })
                .or_insert(1);
        }
    }

    for i in 0..WORD_LENGTH {
        let count = *letter_count.get(&guess[i]).unwrap_or(&0);
        let is_yellow = pattern[i] != Color::Green
            && count > 0;
        if is_yellow {
            pattern.set(i, Color::Yellow);
            letter_count.entry(guess[i])
                .and_modify(|count| {*count -= 1});
        }
    }

    pattern
}

struct Eval<'a> {
    word: &'a Word,
    entropy: f64,
}

impl Display for Eval<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:.3})", self.word, self.entropy)
    }
}

/// Computes the entropy of a given word. It is the expected information
/// content that guessing a given word will reveal.
///
/// # Arguments
///
/// * `word`: The word for which the entropy is calculated.
/// * `solution_space`: The space of potential solution that should be narrowed down.
///
/// returns: `Eval`
fn entropy<'a>(word: &'a Word, solution_space: &Vec<&Word>) -> Eval<'a> {
    let mut pattern_count = [0_u32; Pattern::MAX];
    for solution in solution_space {
        let result = score(&word, solution);
        pattern_count[result.index()] += 1;
    }
    let entropy = -pattern_count.par_iter().map(
        |count| if *count > 0 {
            let p = *count as f64 / solution_space.len() as f64;
            p * p.log2()
        } else { 0.0 }
    ).sum::<f64>();
    Eval{word, entropy}
}

fn print_start<T>(name: &str, vector: &Vec<T>, max_length: usize) where T: Display {
    let length = usize::min(max_length, vector.len());
    print!("\x1b[1m{} ({} entries):\x1b[0m ", name, vector.len());
    for i in 0..length {
        print!("{}, ", vector[i]);
    }
    if length < vector.len() {
        print!("...");
    }
    println!();
}

struct Game<'a> {
    words: &'a Vec<Word>,
    solution_space: Vec<&'a Word>,
    round: u8,
}

impl Game<'_> {

    const MAX_ROUNDS: u8 = 6;

    fn new(words: &Vec<Word>) -> Game {
        Game {
            words,
            solution_space: words.iter().collect(),
            round: 0
        }
    }

    fn evaluate_words(&self) -> Vec<Eval> {
        let mut evaluation = self.words.par_iter().map(|w| {
            entropy(w, &self.solution_space)
        }).collect::<Vec<Eval>>();
        evaluation.sort_unstable_by(|a, b| f64::total_cmp(&b.entropy, &a.entropy));
        evaluation
    }

    fn filter(&mut self, guess: &Word, result: Pattern) {
        self.solution_space = self.solution_space.par_iter().filter_map(|w| {
            if score(guess, w) == result {
                Some(*w)
            } else {
                None
            }
        }).collect()
    }

}

pub struct HelpGame<'a> {
    game: Game<'a>
}

impl HelpGame<'_> {
    pub fn new<'a>(words: &'a Vec<Word>) -> HelpGame<'a> {
        HelpGame { game: Game::new(words) }
    }

    fn read() -> (Word, Pattern) {
        print!("\x1b[1mEnter guessed word:\x1b[0m ");
        stdout().flush().expect("Could not flush stdout");
        let guess = Word::read();
        print!("\x1b[1mEnter resulting pattern:\x1b[0m ");
        stdout().flush().expect("Could not flush stdout");
        let pattern = Pattern::read();
        println!("You have guessed \x1b[1m{}\x1b[0m with result \x1b[1m{}\x1b[0m", guess, pattern);
        (guess, pattern)
    }

    fn round(&mut self) {
        print_start("Solution Space", &self.game.solution_space, 5);
        let eval = self.game.evaluate_words();
        print_start("Suggested Guesses", &eval, 5);
        let (guess, result) = Self::read();
        self.game.filter(&guess, result);
        self.game.round += 1
    }

    pub fn run_game(&mut self) {
        loop {
            self.round();
            if self.game.solution_space.len() == 1 {
                print!("\x1b[1mSuccess!   →{}.\x1b[0m", self.game.solution_space[0]);
                break;
            } else if self.game.solution_space.len() == 0 {
                print!("\x1b[1mFailure!\x1b[0m   No fitting Word in the list!");
                break;
            } else if self.game.round > Game::MAX_ROUNDS {
                print!("\x1b[1mFailure!\x1b[0m   Rounds exhausted!");
                break;
            }
        }
        println!("Score {}", self.game.round);
    }
}

pub struct PlayGame {
    solution: Word,
    round: u8,
}

impl PlayGame {

    pub fn new(words: &Vec<Word>) -> Self {
        let index = rand::thread_rng().gen_range(0..words.len());
        PlayGame {
            solution: words[index],
            round: 0 }
    }

    fn read() -> Word {
        print!("\x1b[1mGuess a word:\x1b[0m ");
        stdout().flush().expect("Could not flush stdout");
        Word::read()
    }

    fn round(&mut self) -> Word {
        self.round += 1;
        let guess = Self::read();
        let result = score(&guess, &self.solution);
        print!("\x1b[1m→ {}\x1b[0m ", result);
        guess
    }

    pub fn run_game(&mut self) {
        loop {
            let guess = self.round();
            if guess == self.solution {
                println!("\x1b[1mSuccess!   →{}.\x1b[0m", self.solution);
                break;
            } else if self.round > Game::MAX_ROUNDS {
                println!("\x1b[1mFailure!\x1b[0m   Rounds exhausted!");
                println!("\x1b[1mThe word was {}.\x1b[0m", self.solution);
                break;
            }
        }
        println!("Score {}", self.round);
    }

}


pub struct SimulatedGame<'a> {
    game: Game<'a>,
    guesses: Vec<Word>,
    solution: Word,
    first_guess: Word
}

impl SimulatedGame<'_> {
    pub fn new<'a>(words: &'a Vec<Word>, solution: Word, first_guess: Word) -> SimulatedGame<'a> {
        SimulatedGame {
            game: Game::new(words),
            guesses: Vec::with_capacity(Game::MAX_ROUNDS as usize),
            solution,
            first_guess,
        }
    }

    fn guess(&mut self) -> Word {
        self.game.round += 1;
        if self.game.round == 1 {
            self.first_guess
        } else if self.game.solution_space.len() == 1 {
            self.game.solution_space[0].clone()
        } else {
            let eval = self.game.evaluate_words();
            eval.par_iter()
                .max_by(|a, b| f64::total_cmp(&a.entropy, &b.entropy))
                .unwrap().word.clone()
        }
    }

    pub fn run_game(&mut self) -> u8 {
        loop {
            let guess = self.guess();
            let result = score(&guess, &self.solution);
            self.game.filter(&guess, result);
            self.guesses.push(guess);
            if guess == self.solution {
                print_start(format!("Game ({})",
                                    self.solution).as_str(), &self.guesses, self.guesses.len());
                return self.game.round;
            } else if self.game.round > Game::MAX_ROUNDS {
                print_start(format!("Game ({})",
                                    self.solution).as_str(), &self.guesses, self.guesses.len());
                return  Game::MAX_ROUNDS + 1
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_score(solution: &str, guess: &str, pattern: &str) {
        assert_eq!(
            score(&Word::from_str(solution), &Word::from_str(guess)),
            Pattern::from_string(pattern)
        );
    }

    #[test]
    fn test_score() {
        assert_score("tears", "bears", "bgggg");
        assert_score("tears", "stear", "yyyyy");
        assert_score("atttt", "xaaaa", "ybbbb");
        assert_score("aattt", "txxxx", "bbybb");
    }
}