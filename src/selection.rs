use std::io;
use console::{Key, Term};

/// Represents a single-select dialog.
pub struct Selection<'a, T>
where T: Copy
{
    /// The index of the default option (e.g. 0 represents the first optio in the `options` vector).
    default_index: usize,
    /// The prefix to print ahead of the selected item.
    selected_prefix: &'a str,
    /// A vector of tuples that contain two values: (1) the string to print representing the value, and (2) the type's value.
    options: Vec<(&'a str, T)>,
}

impl<'a, T> Selection<'a, T>
where T: Copy
{
    /// Creates a new selection with a collection of tuples containing the following items:
    /// - the text to print
    /// - the thing that maps to that text (i.e. if that text is selected, the corresponding thing is returned by the [`prompt()`](Selection::prompt) function).
    pub fn new(selected_prefix: &'a str, options: Vec<(&'a str, T)>) -> Self {
        Self {
            default_index: 0,
            selected_prefix,
            options,
        }
    }

    /// Adds an option to the selection; consumes the calling instance and returning the transformed one.
    pub fn opt(mut self, tup: (&'a str, T)) -> Self {
        self.options.push(tup);
        self
    }

    /// Sets the prefix for the selected item.
    pub fn prefix(self, selected_prefix: &'a str) -> Self {
        Self {
            selected_prefix,
            ..self
        }
    }

    /// Sets the default option (the thing that's initially selected).
    pub fn set_default(self, default_index: usize) -> Self {
        Self {
            default_index,
            ..self
        }
    }

    /// Prompts the user for an input by printing `msg` with `println!()`.
    /// This function will print the textual part of all options, and return the corresponding part represented by it (i.e. the value passed as `T`).
    pub fn prompt(&self, msg: &str) -> io::Result<(&'a str, T)> {
        let term = Term::stdout();
        let mut selected_index = self.default_index;
        // use a selection dialog - consider console crate
        // index the vector by calling .get() and passing the index of the option chosen

        // loop to listen for keystrokes
            // on enter, return the result;
            // on arrow key, re-render the dialog and select the item that lies in the corresponding direction
        println!("{}", msg);
        for _ in 0..self.options.len() {
            println!();
        }

        loop {
            term.clear_last_lines(self.options.len())?;
            // print the items
            for (i, (s, _)) in self.options.iter().enumerate() {
                println!("{}{}", if i == selected_index { self.selected_prefix } else { "" }, s);
            }

            term.hide_cursor()?;

            match term.read_key()?
            {
                Key::ArrowUp => {
                    if selected_index as isize == -1 {
                        selected_index = self.options.len() - 1;
                    } else {
                        selected_index = ( (selected_index as i64 - 1 + self.options.len() as i64) % self.options.len() as i64 ) as usize;
                    }
                }

                Key::ArrowDown => {
                    if selected_index as isize == -1 {
                        selected_index = 0;
                    } else {
                        selected_index = ((selected_index as u64 + 1) % self.options.len() as u64) as usize;
                    }
                }

                Key::Enter => {
                    let tup = *self.options.get(selected_index).expect("unexpectedly failed to get selected item");
                    term.show_cursor()?;
                    return Ok(tup);
                }
                _ => {}
            }
        }
    }
}