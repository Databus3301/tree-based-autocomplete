use std::cmp::min;
use std::io::{self, Write};
use termion::{clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use crate::tree::Tree;

pub fn track(mut t: Tree<char>) {
    // Set the terminal to raw mode to capture input character by character
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "> ").unwrap();
    stdout.flush().unwrap();

    // Capture input and handle each character as it's typed
    let mut input = String::new();
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char(ch) => {
                write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                stdout.flush().unwrap();

                // Handle character input
                if ch == '\n' {
                    break;
                }
                // Query suggestions
                input.push(ch);
                let suggestions = Tree::<char>::complete_sentence(&mut t, &input);
                let out = &suggestions[0..min(5, suggestions.len())].join(" ");

                print!("> {}\n", input);
                println!("{}", out);
                stdout.flush().unwrap();
            }
            termion::event::Key::Backspace => {
                // Handle backspace
                input.pop();
                write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                stdout.flush().unwrap();
                print!("> {}\n", input);
            }
            _ => {}
        }
    }
}
