use std::array::TryFromSliceError;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::ops::Index;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Word([u8; 5]);

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.0 {
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

impl TryFrom<&[u8]> for Word {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl Word {
    pub fn value(&self, freq: &[usize; 26]) -> usize {
        // the value of a word is based on the weights of the letters in it
        let mut cpresent = [false; 26];
        self.0.iter().for_each(|&c| {
            cpresent[(c - b'a') as usize] = true;
        });
        cpresent
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(freq[i]) } else { None })
            .sum()
    }
}

pub struct Wordlist {
    words: HashSet<Word>,
}

impl Wordlist {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let lines = content
            .trim()
            .lines()
            .map(str::trim)
            .filter(|s| s.len() == 5);
        let words = lines
            .map(|l| l.as_bytes()[0..5].try_into())
            .collect::<Result<_, TryFromSliceError>>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Self { words })
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn letter_frequencies(&self) -> [usize; 26] {
        let mut ret = [0; 26];
        for word in &self.words {
            let mut cpresent = [false; 26];
            word.0.iter().for_each(|&c| {
                cpresent[(c - b'a') as usize] = true;
            });
            cpresent.into_iter().enumerate().for_each(|(i, p)| {
                if p {
                    ret[i] += 1;
                }
            });
        }
        ret
    }

    pub fn print_letter_frequencies(&self) {
        let freq = self.letter_frequencies();
        for (i, f) in freq.iter().enumerate() {
            println!("{}: {}", (b'a' + (i as u8)) as char, f);
        }
    }

    pub fn words_by_value(&self) -> impl Iterator<Item = Word> {
        let freq = self.letter_frequencies();
        let mut all_words: Vec<Word> = self.words.iter().copied().collect();
        all_words.sort_unstable_by_key(|k| k.value(&freq));
        all_words.into_iter().rev()
    }

    pub fn words_by_eliminate(&self) -> impl Iterator<Item = Word> {
        // order words by most eliminations first
        let mut all_words: Vec<Word> = self.words.iter().copied().collect();
        all_words.sort_unstable_by_key(|k| {
            (
                self.words
                    .iter()
                    .filter(|w| w.0.iter().any(|c| !k.0.contains(c)))
                    .count(),
                k.0,
            )
        });
        all_words.into_iter()
    }

    pub fn eliminate_char(&mut self, ch: u8) {
        self.words.retain(|w| !w.0.contains(&ch));
    }

    pub fn eliminate_non_exact(&mut self, pos: usize, ch: u8) {
        self.words.retain(|w| w[pos] == ch);
    }

    pub fn eliminate_exact(&mut self, pos: usize, ch: u8) {
        self.words.retain(|w| w[pos] != ch);
    }
}
