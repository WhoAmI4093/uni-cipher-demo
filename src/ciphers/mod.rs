use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use inquire_derive::Selectable;
use log::error;

pub mod substitution_cipher;

pub struct CharacterSet {
    pub alphabet: Vec<char>,
    pub char_to_idx: HashMap<char, usize>,
    pub idx_to_char: HashMap<usize, char>,
}

impl CharacterSet {
    pub fn new(prompt: &str, previous_menu: &dyn Fn()) -> Option<CharacterSet> {
        let alph = get_chars(prompt);

        if alph.is_none() {
            previous_menu();
            // Todo! Use threads to not pollute the stack
            return None;
        }
        let alph = alph.unwrap();

        let char_to_idx: HashMap<char, usize> = alph.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let idx_to_char: HashMap<usize, char> = alph.iter().enumerate().map(|(i, c)| (i, *c)).collect();

        // Check for collisions

        if alph.len() != char_to_idx.len() {
            error!("The string contains repeating characters, reinput it again");
            CharacterSet::new(prompt, previous_menu)
        } else {
            Some(CharacterSet {
                alphabet: alph,
                char_to_idx,
                idx_to_char
            })
        }
    }
}

fn get_chars(prompt: &str) -> Option<Vec<char>> {
    let alphabet: Result<Vec<char>, _> = inquire::Text::new(prompt).prompt().map(|ok| ok.chars().collect());

    alphabet.ok()
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

