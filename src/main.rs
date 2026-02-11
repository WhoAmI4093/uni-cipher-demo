mod logging;
mod ciphers;
mod util;

use std::fmt::{Display, Formatter, Write};
use std::io::Stdout;
use std::sync::LazyLock;
use inquire;
use inquire_derive::Selectable;
use log::info;
use terminal::Terminal;
use crate::ciphers::{affine_cipher, substitution_cipher};
use crate::logging::setup_logger;

#[derive(Debug, Clone, Copy, Selectable)]
pub enum SupportedCiphers {
    SubstitutionCipher,
    AffineCipher,
    AffineRecursiveCipher
}

impl Display for SupportedCiphers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SupportedCiphers::SubstitutionCipher => {
                f.write_str("Substitution Cipher")
            }
            SupportedCiphers::AffineCipher => {
                f.write_str("Affine Cipher")
            }
            SupportedCiphers::AffineRecursiveCipher => {
                f.write_str("Affine Recursive Cipher")
            }
        }
    }
}

static TERMINAL: LazyLock<Terminal<Stdout>> = LazyLock::new(|| terminal::stdout());

fn main() {
    setup_logger().unwrap();

    
    main_menu()
}

fn main_menu() {
    let selected_cipher = unwrap_or_return!( SupportedCiphers::select("Select a cipher: ").prompt().ok() );

    let _ = TERMINAL;

    match selected_cipher {
        SupportedCiphers::SubstitutionCipher => {
            substitution_cipher::demo();
        }
        SupportedCiphers::AffineCipher => {
            affine_cipher::demo(());
        }
        SupportedCiphers::AffineRecursiveCipher => {}
    }
}