pub mod inverse;

use terminal::Action;
use crate::TERMINAL;

pub fn clear_line() {
    let _ = TERMINAL.act(Action::ClearTerminal(terminal::Clear::CurrentLine));
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($to_unwrap:expr) => {
        match $to_unwrap {
            Some(value) => value,
            None => return
        }
    };
}