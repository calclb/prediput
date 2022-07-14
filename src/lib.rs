//! An intuitive, no-frills command-line prompting library for Rust.
#![deny(missing_docs, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

#![allow(clippy::module_name_repetitions)]

#![allow(clippy::missing_const_for_fn)]

/// Module for text-based prompts with custom validation.
pub mod prompting;
/// Module for single-select dialogs.
pub mod select;
/// Module for library macros.
mod macros;

use std::io;
use std::io::{stdout, Write};
use std::str::FromStr;
use console::{Key, Term};

/// A convenience function to get a user input.
/// Note that this function uses the [`print!`](std::print) macro (before flushing stdout) so that the programmer can make prompts in-line.
/// 
/// This function continues to prompt the user until an input can be properly converted to the desired type; `invalid_msg` is printed when the attempted conversion fails.
///
/// # Errors
/// As this function internally uses the [`input()`](crate::input) function, errors that occur there will be propgated to the caller.
#[must_use = "this function returns the converted value, which should be used"]
pub fn prompt<T: FromStr>(prompt: &str, invalid_msg: &str) -> T {
    loop {
        if let Ok(s) = input(prompt) {
            if let Ok(val) = s.trim().parse::<T>() {
                return val;
            }
        }
    
        println!("{}", invalid_msg);
    }
}

/// A convenience function to get a user input.
/// Note that this function uses the [`print!`](std::print) macro and flushes `stdout` for printing, so that the programmer can make prompts in-line.
///
/// # Errors
/// Propogates any internal I/O errors.
pub fn input(prompt: &str) -> io::Result<String> {
    let stdin = io::stdin();

    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush()?;
    stdin.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Prompts for a keystroke (either `'y'` or `'n'`).
/// Returns true when `'y'` is pressed, or false when `'n'` is pressed.
///
/// # Errors
/// Propogates any errors that occur in the [`console`](console) crate dependency
pub fn confirm(prompt: &str, hide_after: bool) -> io::Result<bool> {
    let term = Term::stdout();
    term.hide_cursor()?;
    print!("{}", prompt);
    stdout().flush()?;

    let is_confirmed = loop { // per keystroke
        let key = term.read_key()?;
        match key {
            Key::Char('y' | 'Y') => { break true; }
            Key::Char('n' | 'N') => { break false; }
            _ => continue
        }
    };
    if hide_after {
        term.clear_line()?;
    }
    term.show_cursor()?;
    Ok(is_confirmed)
}

/// Waits (blocking) for the user to press enter.
///
/// # Errors
/// Propogates errors from the following methods:
/// - [`Stdout::flush`](std::io::stdio::Stdout::flush)
/// - [`Term::hide_cursor`]
/// - [`Term::clear_line`]
/// - [`Term::show_cursor`]
pub fn enter_to_continue() -> io::Result<()> {
    let term = Term::stdout();
    term.hide_cursor()?;
    print!("Press enter to continue...");
    stdout().flush()?;

    loop {
        if let Ok(key) = term.read_key() {
            if key == Key::Enter {
                term.clear_line()?;
                term.show_cursor()?;
                return Ok(());
            }
        }
    }
}

/// Waits (blocking) for the user to press a key. Panics if an error is propogated internally.
///
/// # Errors
/// Propogates errors from the following methods:
/// - [`Stdout::flush`](std::io::stdio::Stdout::flush)
/// - [`Term::hide_cursor`]
/// - [`Term::clear_line`]
/// - [`Term::show_cursor`]
/// - [`Term::read_key`]
pub fn any_key_continue() -> io::Result<()> {
    let term = Term::stdout();
    term.hide_cursor()?;
    print!("Press any key to continue...");
    stdout().flush()?;
    term.read_key()?;
    term.clear_line()?;
    term.show_cursor()?;
    Ok(())
}

/// Clears the terminal. Any errors that occur are propogated to the caller.
///
/// # Errors
/// Propogates any errors from [`Term::clear_screen`].
pub fn clear_terminal() -> io::Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    Ok(())
}
