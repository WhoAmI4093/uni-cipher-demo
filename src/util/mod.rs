use terminal::Action;

pub fn clear_line() {
    let terminal = terminal::stdout();
    let _ = terminal.act(Action::ClearTerminal(terminal::Clear::CurrentLine));
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