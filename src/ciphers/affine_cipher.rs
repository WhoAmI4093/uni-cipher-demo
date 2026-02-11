use std::collections::HashSet;
use log::{error, info, warn};
use crate::ciphers::{get_chars, inquire_isize, CharacterSet, CipherOperations};
use crate::{main_menu, unwrap_or_return};
use crate::util::clear_line;
use crate::util::inverse::mod_inverse;

pub fn demo(_: ()) {
    let character_set = CharacterSet::new("Enter an alphabet to use:", &main_menu);

    let alphabet_length: isize = match isize::try_from(character_set.alphabet.len()) {
        Ok(n) => n,
        Err(_) => {
            error!("Length of character set is too large");
            return demo(());
        }
    };

    demo_get_key_and_continue(&DemoKeyAndContinueArgs {
        character_set: &character_set,
        alphabet_length
    });
}

struct DemoKeyAndContinueArgs<'a> {
    character_set: &'a CharacterSet,
    alphabet_length: isize,
}

fn demo_get_key_and_continue(DemoKeyAndContinueArgs{character_set, alphabet_length}: &DemoKeyAndContinueArgs) {
    info!("Alphabet length is {}", character_set.char_to_idx.len());

    let key_a = inquire_isize("Enter multiplicative key:", &demo, ());

    let inverse_key_a = match mod_inverse(key_a, *alphabet_length) {
        Some(n) => n,
        None => {
            error!("This value does not have an inverse mod {alphabet_length}. Try again");
            return demo_get_key_and_continue(&DemoKeyAndContinueArgs {
                character_set: &character_set,
                alphabet_length: *alphabet_length
            });
        }
    };

    let key_b = inquire_isize("Enter additive key:", &demo_get_key_and_continue, &DemoKeyAndContinueArgs {
        character_set: &character_set,
        alphabet_length: *alphabet_length
    });

    demo_operation(&character_set, *alphabet_length, &Key {
        multiplicative: key_a,
        inverse_multiplicative: inverse_key_a,
        additive: key_b,
    });
}

struct Key {
    multiplicative: isize,
    inverse_multiplicative: isize,
    additive: isize,
}

fn demo_operation(character_set: &CharacterSet, alphabet_length: isize, key: &Key) {
    let desired_operation = CipherOperations::select("What function do you want to use?").prompt();

    if desired_operation.is_err() {
        clear_line();

        demo_get_key_and_continue(&DemoKeyAndContinueArgs {
            character_set,
            alphabet_length
        });

        std::process::exit(0);
    }
    let desired_operation = desired_operation.unwrap();

    match &desired_operation {
        CipherOperations::Encrypt => demo_encrypt(character_set, alphabet_length, key),
        CipherOperations::Decrypt => demo_decrypt(character_set, alphabet_length, key),
    }
}

fn demo_encrypt(character_set: &CharacterSet, alphabet_length: isize, key: &Key) {
    let text = match get_chars("Enter plaintext:") {
        Some(text) => text,
        None => {
            demo_operation(character_set, alphabet_length, key);
            std::process::exit(0);
        }
    };

    let set_text: HashSet<char> = HashSet::from_iter(text.iter().cloned());

    let diff_with_alphabet: HashSet<&char> = set_text.difference(&character_set.set_characters).collect();

    if diff_with_alphabet.len() > 0 {
        error!("Plaintext contains character(s) that is(are) not in alphabet: {:#?}", diff_with_alphabet);
        demo_encrypt(character_set, alphabet_length, key);
    }

    info!("Ciphertext is: {}", text.iter().map(|char| character_set.idx_to_char.get(&usize::try_from((key.multiplicative * isize::try_from(*character_set.char_to_idx.get(char).unwrap()).unwrap() + key.additive) % alphabet_length).unwrap()).unwrap() ).collect::<String>());
    demo_operation(character_set, alphabet_length, key);
}

fn demo_decrypt(character_set: &CharacterSet, alphabet_length: isize, key: &Key) {
    let text = match get_chars("Enter ciphertext:") {
        Some(text) => text,
        None => {
            demo_operation(character_set, alphabet_length, key);
            std::process::exit(0);
        }
    };

    let set_text: HashSet<char> = HashSet::from_iter(text.iter().cloned());

    let diff_with_alphabet: HashSet<&char> = set_text.difference(&character_set.set_characters).collect();

    if diff_with_alphabet.len() > 0 {
        error!("Ciphertext contains character(s) that is(are) not in alphabet: {:#?}", diff_with_alphabet);
        demo_encrypt(character_set, alphabet_length, key);
    }

    info!("Plaintext is: {}", text.iter().map(|char| character_set.idx_to_char.get(&usize::try_from((key.inverse_multiplicative * isize::try_from(*character_set.char_to_idx.get(char).unwrap()).unwrap() + key.additive) % alphabet_length).unwrap()).unwrap() ).collect::<String>());
    demo_operation(character_set, alphabet_length, key);
}