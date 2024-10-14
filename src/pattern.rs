use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::ops::Index;
use crate::word::WORD_LENGTH;

/// Represents the color feedback in a Wordle game.
///
/// # Variants
/// * `Green` - Indicates a correct letter in the correct position.
/// * `Yellow` - Indicates a correct letter in the wrong position.
/// * `Black` - Indicates a letter that is not present in the word.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color { Green, Yellow, Black, }

impl Color {
    const SIZE: u8 = 3;

    const fn value(&self) -> u8 {
        match self {
            Color::Green => {2}
            Color::Yellow => {1}
            Color::Black => {0}
        }
    }
}


impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Green => {"\x1b[32mg\x1b[0m"}
            Color::Yellow => {"\x1b[33my\x1b[0m"}
            Color::Black => {"\x1b[30mb\x1b[0m"}
        })
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Pattern {
    pattern: u8
}

impl Pattern {
    const BASES: [u8; WORD_LENGTH + 1] =
        [1 /* = 3^0 */,  3 /* = 3^1 */,  9 /* = 3^2 */,
            27 /* = 3^3 */, 81 /* = 3^4 */, 243 /* = 3^5 */];

    pub fn all_black() -> Pattern { Pattern{ pattern: 0 } }

    pub fn index(&self) -> usize { self.pattern as usize }

    pub fn set(&mut self, i: usize, color: Color) {
        let lower = self.pattern % Self::BASES[i];
        let higher = self.pattern / Self::BASES[i + 1] * Self::BASES[i + 1];
        self.pattern = lower + higher + Self::BASES[i] * color.value();
    }

    pub fn from_string(line: &str) -> Pattern {
        let line = line.trim();
        let mut pattern = Pattern::all_black();
        let line = line.chars().collect::<Vec<char>>();
        assert_eq!(line.len(), WORD_LENGTH);
        for i in 0..WORD_LENGTH {
            let color = match line[i] {
                'b' => Color::Black,
                'y' => Color::Yellow,
                'g' => Color::Green,
                _ => panic!("unknown char {}. Use g = green, y = yellow, b = black.",
                            line[i]),
            };
            pattern.set(i, color);
        }
        pattern
    }

    pub fn read() -> Pattern {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Read failed");
        Pattern::from_string(&line)
    }

    pub const MAX: usize = usize::pow(Color::SIZE as usize, WORD_LENGTH as u32);
}

impl Index<usize> for Pattern {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < WORD_LENGTH);
        let code = (self.pattern % Self::BASES[index + 1]) / Self::BASES[index];
        match code {
            0 => &Color::Black,
            1 => &Color::Yellow,
            2 => &Color::Green,
            _ => panic!()
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}",
               self[0], self[1], self[2], self[3], self[4])
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
