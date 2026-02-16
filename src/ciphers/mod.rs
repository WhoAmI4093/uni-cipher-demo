use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::io::Stdout;
use inquire_derive::Selectable;
use log::{error, info};
use crate::util::clear_line;
use rand::seq::SliceRandom;
use rand::rngs::{SysRng, ThreadRng};

pub mod substitution_cipher;
pub mod affine_cipher;
pub mod affine_recursive_cipher;

pub struct CharacterSet {
    pub alphabet: Vec<char>,
    pub char_to_idx: HashMap<char, usize>,
    pub idx_to_char: HashMap<usize, char>,
    pub set_characters: HashSet<char>,
}

#[derive(Selectable)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug, PartialEq)]
pub enum AlphabetSelect {
    Input,
    CyrillicAndSpecial,
    LatinLower,
    Numbers,
    Combined
}

impl Display for AlphabetSelect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlphabetSelect::Input => f.write_str("From input"),
            AlphabetSelect::CyrillicAndSpecial => f.write_str("Cyrillic and special characters"),
            AlphabetSelect::LatinLower => f.write_str("Lowercase latin"),
            AlphabetSelect::Numbers => f.write_str("Numbers"),
            AlphabetSelect::Combined => f.write_str("Combined (with newline & tab)"),
        }
    }
}

const LATIN_ALPHABET: &str = include_str!(r"..\..\alphabets\latin.txt");
const CYRILLIC_ALPHABET: &str = include_str!(r"..\..\alphabets\cyrillic_upper_lower_special.txt");
const NUMBERS_ALPHABET: &str = include_str!(r"..\..\alphabets\numbers.txt");
const COMBINED_ALPHABET: &str = include_str!(r"..\..\alphabets\combined.txt");

fn alphabet_select(prompt: &str, previous_menu: &dyn Fn(), shuffle: bool) -> Vec<char> {
    let selected_option = AlphabetSelect::select(prompt).prompt().ok();

    if let Some(selected_option) = selected_option {
        let mut chars = match selected_option {
            AlphabetSelect::Input => {
                let alph = get_chars(prompt);

                if alph.is_none() {
                    alphabet_select(prompt, previous_menu, shuffle)
                } else {
                    alph.unwrap()
                }
            },
            AlphabetSelect::Combined => COMBINED_ALPHABET.chars().collect(),
            AlphabetSelect::CyrillicAndSpecial => CYRILLIC_ALPHABET.chars().collect(),
            AlphabetSelect::LatinLower => LATIN_ALPHABET.chars().collect(),
            AlphabetSelect::Numbers => NUMBERS_ALPHABET.chars().collect(),
        };
        if shuffle && selected_option != AlphabetSelect::Input {
            let mut rng = ThreadRng::default();
            chars.shuffle(&mut rng);
        }
        chars
    } else {
        clear_line();
        previous_menu();

        std::process::exit(0);
    }
}

impl CharacterSet {
    pub fn new(prompt: &str, previous_menu: &dyn Fn(), shuffle: bool) -> CharacterSet {
        let alph = alphabet_select(prompt, previous_menu, shuffle);

        info!("Selected alphabet: {}", alph.iter().collect::<String>());

        let char_to_idx: HashMap<char, usize> = alph.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let idx_to_char: HashMap<usize, char> = alph.iter().enumerate().map(|(i, c)| (i, *c)).collect();

        // Check for collisions

        if alph.len() != char_to_idx.len() {
            error!("The string contains repeating characters, reinput it again");
            CharacterSet::new(prompt, previous_menu, shuffle)
        } else {
            let set_characters = HashSet::from_iter(alph.iter().cloned()); 
            CharacterSet {
                alphabet: alph,
                char_to_idx,
                idx_to_char,
                set_characters
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Selectable)]
enum CipherOperations {
    Encrypt,
    Decrypt
}

impl CipherOperations {
    pub fn text_type_name(&self) -> &'static str {
        match &self {
            CipherOperations::Encrypt => "plaintext",
            CipherOperations::Decrypt => "ciphertext",
        }
    }

    pub fn other_type_name(&self) -> &'static str {
        match &self {
            CipherOperations::Encrypt => "cyphertext",
            CipherOperations::Decrypt => "plaintext",
        }
    }
}

impl Display for CipherOperations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CipherOperations::Encrypt => {
                f.write_str("Encrypt")
            }
            CipherOperations::Decrypt => {
                f.write_str("Decrypt")
            }
        }
    }
}

fn get_chars(prompt: &str) -> Option<Vec<char>> {
    let alphabet: Result<Vec<char>, _> = inquire::Text::new(prompt).prompt().map(|ok| ok.chars().collect());

    alphabet.ok()
}

fn inquire_isize<T>(prompt: &str, back: &dyn Fn(T), args: T) -> isize {
    let text = inquire::Text::new(prompt).prompt().ok();
    let text = match text {
        Some(text) => text,
        None => {
            clear_line();
            back(args);
            std::process::exit(0)
        }
    };

    let number = text.parse::<isize>();

    match number {
        Ok(n) => n,
        Err(_) => {
            error!("Failed to parse number. Try again");
            inquire_isize(prompt, back, args)
        }
    }
}
