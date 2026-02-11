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


/// `b` is assumed to be positive
pub fn positive_mod(a: isize, b: isize) -> isize {
    if a < 0 {
        a + b * (a.abs() / b + 1)
    } else {
        a % b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_mod_tests() {
        assert_eq!(positive_mod(-3, 5), 2)
    }

}