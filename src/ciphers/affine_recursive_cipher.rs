use std::collections::HashSet;
use log::{error, info};
use crate::ciphers::{get_chars, inquire_isize, CharacterSet, CipherOperations};
use crate::main_menu;
use crate::util::{clear_line, positive_mod};
use crate::util::inverse::mod_inverse;

pub fn demo(_: ()) {
    let character_set = CharacterSet::new("Enter an alphabet to use:", &main_menu);

    let alphabet_length: isize = match isize::try_from(character_set.alphabet.len()) {
        Ok(n) => n,
        Err(_) => {
            error!("Length of character set is too large");
            return crate::ciphers::affine_cipher::demo(());
        }
    };

    demo_get_first_key_and_continue(&DemoFirstKeyAndContinueArgs {
        character_set: &character_set,
        alphabet_length
    });
}

struct DemoFirstKeyAndContinueArgs<'a> {
    character_set: &'a CharacterSet,
    alphabet_length: isize,
}

#[derive(Clone)]
struct PartialKey {
    multiplicative: isize,
    additive: isize,
}

impl From<(isize, isize)> for PartialKey {
    fn from((multiplicative, additive): (isize, isize)) -> Self {
        Self {
            multiplicative,
            additive
        }
    }
}

#[derive(Clone)]
struct Key {
    multiplicative_first: isize,
    multiplicative_second: isize,

    additive_first: isize,
    additive_second: isize
}

impl From<(PartialKey, PartialKey)> for Key {
    fn from((key_1, key_2): (PartialKey, PartialKey)) -> Self {
        Self {
            multiplicative_first: key_1.multiplicative,
            multiplicative_second: key_2.multiplicative,

            additive_first: key_1.additive,
            additive_second: key_2.additive,
        }
    }
}

impl Key {
    pub fn next(&mut self, modulus: isize) {
        let partial_from_last = PartialKey {
            multiplicative: self.multiplicative_second,
            additive: self.additive_second,
        };
        let partial_current = PartialKey {
            multiplicative: (self.multiplicative_first * self.multiplicative_second) % modulus,
            additive: (self.additive_first + self.additive_second) % modulus,
        };

        *self = (partial_from_last, partial_current).into()
    }
}

//noinspection DuplicatedCode
fn demo_get_first_key_and_continue(DemoFirstKeyAndContinueArgs {character_set, alphabet_length}: &DemoFirstKeyAndContinueArgs) {
    info!("Alphabet length is {}", character_set.char_to_idx.len());

    let key_a = inquire_isize("Enter the first multiplicative key:", &crate::ciphers::affine_cipher::demo, ());

    let _ = match mod_inverse(key_a, *alphabet_length) {
        Some(n) => n,
        None => {
            error!("This value does not have an inverse mod {alphabet_length}. Try again");
            return demo_get_first_key_and_continue(&DemoFirstKeyAndContinueArgs {
                character_set: &character_set,
                alphabet_length: *alphabet_length
            });
        }
    };

    let key_b = inquire_isize("Enter the first additive key:", &demo_get_first_key_and_continue, &DemoFirstKeyAndContinueArgs {
        character_set: &character_set,
        alphabet_length: *alphabet_length
    });

    demo_get_second_key_and_continue(&DemoSecondKeyAndContinueArgs {
        character_set,
        alphabet_length: *alphabet_length,
        first_key: &(key_a, key_b).into()
    })
}

struct DemoSecondKeyAndContinueArgs<'a> {
    character_set: &'a CharacterSet,
    alphabet_length: isize,
    first_key: &'a PartialKey,
}


//noinspection DuplicatedCode
fn demo_get_second_key_and_continue(DemoSecondKeyAndContinueArgs {character_set, alphabet_length, first_key}: &DemoSecondKeyAndContinueArgs) {
    let key_a = inquire_isize("Enter the second multiplicative key:", &demo_get_first_key_and_continue, &DemoFirstKeyAndContinueArgs {
        character_set,
        alphabet_length: *alphabet_length,
    });

    let _ = match mod_inverse(key_a, *alphabet_length) {
        Some(n) => n,
        None => {
            error!("This value does not have an inverse mod {alphabet_length}. Try again");
            return demo_get_second_key_and_continue(&DemoSecondKeyAndContinueArgs {
                character_set: &character_set,
                alphabet_length: *alphabet_length,
                first_key
            });
        }
    };

    let key_b = inquire_isize("Enter the first additive key:", &demo_get_second_key_and_continue, &DemoSecondKeyAndContinueArgs {
        character_set: &character_set,
        alphabet_length: *alphabet_length,
        first_key: &first_key
    });

    let key: Key = Key::from(((*first_key).clone(), (key_a, key_b).into()));

    demo_operation(character_set, *alphabet_length, &key);
}

fn demo_operation(character_set: &CharacterSet, alphabet_length: isize, key: &Key) {
    let desired_operation = CipherOperations::select("What function do you want to use?").prompt();

    if desired_operation.is_err() {
        clear_line();

        demo_get_second_key_and_continue(&DemoSecondKeyAndContinueArgs {
            character_set,
            alphabet_length,
            first_key: &PartialKey {
                multiplicative: key.multiplicative_first,
                additive: key.additive_first
            }
        });

        std::process::exit(0);
    }
    let desired_operation = desired_operation.unwrap();

    match &desired_operation {
        CipherOperations::Encrypt => demo_encrypt(character_set, alphabet_length, key),
        CipherOperations::Decrypt => demo_decrypt(character_set, alphabet_length, key),
    }
}

//noinspection DuplicatedCode
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

    let mut ciphertext: Vec<char> = vec![];
    let mut key = key.clone();

    for char in text {
        let resulting_index = &isize::try_from(*character_set.char_to_idx.get(&char).unwrap()).unwrap() * key.multiplicative_first + key.additive_first;
        let resulting_index = positive_mod(resulting_index, alphabet_length);

        ciphertext.push(*character_set.idx_to_char.get(&usize::try_from(resulting_index).unwrap()).unwrap());

        key.next(alphabet_length);
    }

    info!("Ciphertext is: {}", ciphertext.iter().collect::<String>());

    demo_operation(character_set, alphabet_length, &key);
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
        demo_decrypt(character_set, alphabet_length, key);
    }

    let mut plaintext: Vec<char> = vec![];
    let mut key = key.clone();

    for char in text {
        let inverse_key = mod_inverse(key.multiplicative_first, alphabet_length).unwrap();

        let character_index = isize::try_from(*character_set.char_to_idx.get(&char).unwrap()).unwrap();

        let resulting_index = (character_index - key.additive_first) * inverse_key;
        let resulting_index = positive_mod(resulting_index, alphabet_length);

        plaintext.push(*character_set.idx_to_char.get(&usize::try_from(resulting_index).unwrap()).unwrap());

        key.next(alphabet_length);
    }

    info!("Plaintext is: {}", plaintext.iter().collect::<String>());

    demo_operation(character_set, alphabet_length, &key);
}