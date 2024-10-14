use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::ops::Index;

pub const WORD_LENGTH: usize = 5;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Word {
    chars: [char; WORD_LENGTH],
}

impl Word {
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

    pub fn read() -> Word {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read failed");
        Word::from_str(&line)
    }
}

impl Index<usize> for Word {
    type Output = char;

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