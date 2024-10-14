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

/// Calculates the entropy of a given word in relation to a set of possible solutions.
///
/// The entropy measures how much information a word can provide about the correct solution
/// in a Wordle game. The goal is to find the word that maximizes the reduction of the solution
/// space by providing the most informative feedback when guessed.
///
/// # Arguments
///
/// * `word` - A reference to the word for which entropy is being calculated.
/// * `solution_space` - A reference to a vector containing all possible solution words.
///   Each word is compared to the given `word` to determine how much information can be gained.
///
/// # Returns
///
/// Returns an `Eval` struct containing:
/// * `word` - The word that was evaluated.
/// * `entropy` - A `f64` representing the entropy (information gain) of the word.
///
///
/// # Example
///
/// ```rust
/// let word = "crane";
/// let solution_space = vec![&"apple", &"grape", &"flint"];
/// let evaluation = entropy(word, &solution_space);
/// assert!(evaluation.entropy - 1.58 < 0.05);
/// ```
///
/// In this example, the function calculates how much information the word "crane" can provide
/// about the correct solution given the remaining possible solutions in `solution_space`.
/// # See Also
///
/// * [`score`] - Function that computes the result pattern between two words.
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

/// Prints the first few elements of a vector, along with the total number of entries.
///
/// This function displays the name of the vector, the total number of elements it contains,
/// and the first few elements up to a specified limit. If the vector has more elements than
/// the `max_length` parameter, an ellipsis (`...`) is printed to indicate truncation.
///
/// # Arguments
///
/// * `name` - Some info to print as header
/// * `vector` - A reference to a vector containing the elements to print. The elements must implement
///   the [`Display`] trait to be printed.
/// * `max_length` - The maximum number of elements to print from the start of the vector.
///
/// # Example
///
/// ```rust
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// print_start("Numbers", &numbers, 3);
/// ```
///
/// Output:
/// ```text
/// Numbers (6 entries): 1, 2, 3, ...
/// ```
///
/// In this example, the function prints the first 3 elements of the `numbers` vector, followed by an ellipsis
/// to indicate that the vector contains more elements.
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

/// Represents the state of a Wordle game.
///
/// The `Game` struct keeps track of the words available for guesses, the remaining possible
/// solutions, and the current number of rounds.
///
/// # Fields
///
/// * `words` - A reference to a vector of all possible words that can be used as guesses. This includes both
///   valid solutions and other potential guesses that can help reduce the solution space.
/// * `solution_space` - A vector containing references to the remaining possible solutions. This vector shrinks
///   as the game progresses, based on the feedback from guesses.
/// * `round` - The current round of the game. A round corresponds to a single guess and its feedback.
///   Typically, Wordle games last up to six rounds, see [Game::MAX_ROUNDS].
///
/// # Lifetime Parameters
///
/// * `'a` - The lifetime of the word reference `words`. The references in `solution_space` refer
///    into `words` and has the same lifetime.
///
/// # Example
///
/// ```rust
/// let game = Game::new(&read_file(File::open("wordle.txt")));
/// ```
///
/// # See Also
/// * [crate::read_file] - to obtain word lists for a game.
/// * [PlayGame], [SimulatedGame] - structs that use this one.
struct Game<'a> {
    words: &'a Vec<Word>,
    solution_space: Vec<&'a Word>,
    round: u8,
}

impl Game<'_> {

    /// The maximum number of rounds allowed in a Wordle game.
    ///
    /// In Wordle, players have up to six attempts to guess the correct word. This constant defines
    /// the upper limit on the number of rounds that a game can have.
    ///
    /// # See Also
    ///
    /// * [`Game::round`] - The current round of the game, which is compared against `MAX_ROUNDS`.
    const MAX_ROUNDS: u8 = 6;

    /// Creates a new `Game` instance with the given list of words.
    ///
    /// # Arguments
    ///
    /// * `words` - A reference to a vector of `Word`s listing all possible words that can be used in the game.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Game` with:
    /// * `words` - Set to the input vector of words.
    /// * `solution_space` - Initially set to include all words from the `words` vector. As the game
    /// progresses, this solution space can be reduced based on feedback from guesses.
    /// * `round` - Initialized to 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// let word_list = read_file("wordle.txt");
    /// let game = Game::new(&word_list);
    /// ```
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

    /// Filters the solution space based on the result of a guess.
    ///
    /// This function refines the game's solution space by eliminating words that do not match the
    /// feedback pattern from a given guess. After a guess is made and a feedback pattern (green, yellow, gray)
    /// is received, this function removes words from the solution space that would not produce the same
    /// feedback pattern if they were the correct solution. The remaining words in the solution space are
    /// the ones still consistent with the given feedback.
    ///
    /// # Arguments
    /// * `guess` - A reference to the word that was guessed.
    /// * `result` - The `Pattern` representing the feedback received from the guess (e.g., which letters are
    ///   correct and in the right position, which are correct but in the wrong position, and which are incorrect).
    ///
    /// # See Also
    /// * [`score`] - Function that compares two words and returns the feedback pattern.
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