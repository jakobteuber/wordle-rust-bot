use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::ops::Index;

/// The fixed length of words in the Wordle game. In Wordle, all valid words have
/// a length of 5 characters, though this for this implementation any other constant
/// word size would work.
pub const WORD_LENGTH: usize = 5;

/// Represents a word used in the Wordle game.
///
/// The `Word` struct stores a word as an array of characters with a fixed length of
/// `WORD_LENGTH`. This struct is used for both guesses and possible solutions in the game.
///
/// # Fields
/// * `chars` - An array of `char` representing the individual characters of the word.
///
/// # Derives
/// * `Clone` - Allows the `Word` to be cloned.
/// * `Copy` - Enables the `Word` to be copied by value.
/// * `Eq`, `PartialEq` - Allows for equality comparisons between `Word` instances.
///
/// # Example
/// ```rust
/// let word = Word::from_str("crane");
/// assert_eq!(word[0], 'c');
/// assert_eq!(word.chars.len(), WORD_LENGTH);
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Word {
    chars: [char; WORD_LENGTH],
}

impl Word {

    /// Creates a `Word` from a string slice.
    ///
    /// This function takes a string slice (`&str`), trims any leading or trailing whitespace,
    /// and converts it into a `Word`.
    ///
    /// # Arguments
    /// * `word` - A string slice (`&str`) representing the word to be converted into a `Word`.
    ///
    /// # Panics
    /// This function will panic if the length of the input string, after trimming,
    /// is not equal to `WORD_LENGTH`.
    ///
    /// # See Also
    /// * [`WORD_LENGTH`] - The constant representing the fixed length of a word.
    pub fn from_str(word: &str) -> Word {
        let word = word.trim();
        let chars = word.chars().collect::<Vec<char>>();
        assert_eq!(chars.len(), WORD_LENGTH, "word <{}> has bad length", word);
        let mut word = Word{ chars: ['?'; WORD_LENGTH]};
        for i in 0..WORD_LENGTH {
            word.chars[i] = chars[i];
        }
        word
    }


    /// Reads a word from standard input and converts it into a `Word`.
    ///
    /// This function reads a single line of input from the user, trims any leading or trailing whitespace,
    /// and converts the resulting string into a `Word` using the `Word::from_str` function. If the input
    /// cannot be read or is of incorrect length, the function will panic.
    pub fn read() -> Word {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read failed");
        Word::from_str(&line)
    }
}


impl Index<usize> for Word {
    type Output = char;

    /// Allows indexing into a `Word` using the `[]` syntax to access individual characters in the word.
    /// This implementation will panic if the index is out of bounds (i.e., greater than or equal to `WORD_LENGTH`).
    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}",
               self[0], self[1], self[2], self[3], self[4])
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}