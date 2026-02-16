use log::{error, info, warn};
use crate::ciphers::{get_chars, CharacterSet, CipherOperations};
use std::collections::HashSet;
use terminal::Action;
use crate::{main_menu};
use crate::util::clear_line;

pub fn demo() {
    let character_set =  CharacterSet::new("Enter an alphabet to use:", &main_menu, false);
    let key_character_set = CharacterSet::new("Enter the substitution to use (will be shuffled if not input):", &main_menu, true);

    // if lengths dont math its bad :(
    if character_set.char_to_idx.len() != key_character_set.char_to_idx.len() {
        error!("Substitution key and the initial alphabet must have the same amount of characters. Restarting");
        demo()
    }

    let set_initial: HashSet<char> = HashSet::from_iter(character_set.alphabet.iter().cloned());
    let set_substitution: HashSet<char> = HashSet::from_iter(key_character_set.alphabet.iter().cloned());

    if set_initial != set_substitution {
        warn!("Substitution key uses a different alphabet");
    }

    demo_operation(&character_set, &key_character_set, &set_initial, &set_substitution);
}

fn demo_operation(character_set: &CharacterSet, key_character_set: &CharacterSet, set_initial: &HashSet<char>, set_substitution: &HashSet<char>) {
    let desired_operation = CipherOperations::select("What function do you want to use?").prompt();

    if desired_operation.is_err() {
        clear_line();
        demo();
        std::process::exit(0);
    }
    let desired_operation = desired_operation.unwrap();

    match &desired_operation {
        CipherOperations::Encrypt => demo_encrypt(character_set, key_character_set, &desired_operation, &set_initial, &set_substitution),
        CipherOperations::Decrypt => demo_decrypt(character_set, key_character_set, &desired_operation, &set_initial, &set_substitution)
    }
}

fn demo_encrypt(character_set: &CharacterSet, key_character_set: &CharacterSet, desired_operation: &CipherOperations, set_initial: &HashSet<char>, set_substitution: &HashSet<char>) {
    let text = get_chars(&*format!("Input {}:", desired_operation.text_type_name()));

    let text = match text { 
        Some(text) => text,
        None => return demo_operation(character_set, key_character_set, set_initial, set_substitution)
    };
    
    let set_text: HashSet<char> = HashSet::from_iter(text.iter().cloned());

    let diff_with_alphabet: HashSet<&char> = set_text.difference(set_initial).collect();

    if diff_with_alphabet.len() > 0 {
        error!("{} contains character(s) that is(are) not in alphabet: {:#?}", desired_operation.text_type_name(), diff_with_alphabet);
        demo_encrypt(character_set, key_character_set, desired_operation, set_initial, set_substitution);
    }

    info!("Resulting {}: {}", desired_operation.other_type_name(), text.iter().map(|char| key_character_set.idx_to_char.get(character_set.char_to_idx.get(char).unwrap()).unwrap()).collect::<String>());

    demo_operation(character_set, key_character_set, set_initial, set_substitution);
}

fn demo_decrypt(character_set: &CharacterSet, key_character_set: &CharacterSet, desired_operation: &CipherOperations, set_initial: &HashSet<char>, set_substitution: &HashSet<char>) {
    demo_encrypt(key_character_set, character_set, &CipherOperations::Encrypt, set_substitution, set_initial);
}