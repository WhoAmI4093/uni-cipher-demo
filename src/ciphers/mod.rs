use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::fs;
use std::io::Stdout;
use std::path::PathBuf;
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
            AlphabetSelect::CyrillicAndSpecial => f.write_str("Lowercase cyrillic and special characters"),
            AlphabetSelect::LatinLower => f.write_str("Lowercase latin"),
            AlphabetSelect::Numbers => f.write_str("Numbers"),
            AlphabetSelect::Combined => f.write_str("Combined (with newline & tab)"),
        }
    }
}

const LATIN_ALPHABET: &str = include_str!(r"..\..\alphabets\latin.txt");
const CYRILLIC_ALPHABET: &str = include_str!(r"..\..\alphabets\cyrillic_lower_special.txt");
const NUMBERS_ALPHABET: &str = include_str!(r"..\..\alphabets\numbers.txt");
const COMBINED_ALPHABET: &str = include_str!(r"..\..\alphabets\combined.txt");

fn alphabet_select(prompt: &str, previous_menu: &dyn Fn(), shuffle: bool) -> Vec<char> {
    let selected_option = AlphabetSelect::select(prompt).prompt().ok();

    if let Some(selected_option) = selected_option {
        let mut chars = match selected_option {
            AlphabetSelect::Input => {
                let alph = get_chars(prompt  /*, &|| {
                    clear_line();
                    alphabet_select(prompt, previous_menu, shuffle);
                } */, None);

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

#[derive(Copy, Clone, Debug, Selectable)]
enum CharsOptions {
    Input,
    File
}

impl Display for CharsOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharsOptions::Input => f.write_str("From input"),
            CharsOptions::File => f.write_str("From file (will be lowercased and filtered)"),
        }
    }
}

fn get_chars(prompt: &str /*, previous_menu: &dyn Fn() */, allowed_characters: Option<&HashSet<char>>) -> Option<Vec<char>> {
    info!("{prompt}");

    let chosen_method = CharsOptions::select("Chose the way to get characters:").prompt().ok();

    if let Some(method) = chosen_method {
        match method {
            CharsOptions::Input => {
                let chars: Result<Vec<char>, _> = inquire::Text::new("Enter characters:").prompt().map(|ok| ok.chars().collect());

                chars.ok()
            }
            CharsOptions::File => {
                let chars: Option<Vec<char>> = inquire::Text::new("Enter file path: ").prompt().map(|ok| ok.chars().collect()).ok();

                if chars.is_none() {
                    return get_chars(prompt, allowed_characters)
                }

                let path = chars.unwrap().iter().collect::<String>();
                let path_buf = PathBuf::from(path);
                let file = fs::read(&path_buf).ok();
                if file.is_none() {
                    error!("Invalid file path");
                    return get_chars(prompt, allowed_characters)
                }

                let file = file.unwrap();
                let mut string = String::from_utf8_lossy(&file).to_lowercase();

                if let Some(allowed_characters) = allowed_characters {
                    string = string.chars().filter(|char| allowed_characters.contains(char)).collect();
                }

                Some(string.chars().collect())
            }
        }
    } else {
        return None
    }
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

#[derive(Copy, Clone, Debug, Selectable)]
pub enum OutputOptions {
    Stdout,
    File
}

impl Display for OutputOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputOptions::Stdout => f.write_str("Stdout"),
            OutputOptions::File => f.write_str("File")
        }
    }
}

impl OutputOptions {
    pub fn write(formatted_string: &String, previous_menu: &dyn Fn()) {
        let selected_option = OutputOptions::select("Where to output results?").prompt().ok();

        if selected_option.is_none() {
            previous_menu();

            std::process::exit(0);
        }

        let selected_option = selected_option.unwrap();

        match selected_option {
            OutputOptions::Stdout => info!("{formatted_string}"),
            OutputOptions::File => {
                let chars: Option<Vec<char>> = inquire::Text::new("Enter file path: ").prompt().map(|ok| ok.chars().collect()).ok();

                if chars.is_none() {
                    OutputOptions::write(formatted_string, previous_menu);
                }

                let path = chars.unwrap().iter().collect::<String>();
                let path_buf = PathBuf::from(path);
                let file = fs::write(&path_buf, formatted_string).ok();
                if file.is_none() {
                    error!("Invalid file path or cannot write to file");
                    OutputOptions::write(formatted_string, previous_menu);
                }
            }
        }
    }
}