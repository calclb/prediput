use std::io;
use console::{Key, Term};

/// Represents a single-select dialog.
pub struct Selection<'a, T>
where T: Copy
{
    /// The index of the default option (e.g. 0 represents the first optio in the `options` vector).
    default_index: usize,
    /// Number of lines that separates the prompt from other text
    padding: usize,
    /// The prefix to print ahead of the selected item.
    selected_prefix: &'a str,
    /// Determines whether to clear the prompt after an answer is given.
    clear_after_response: bool,
    /// A vector of tuples that contain three values: (1) the string to display for the value by default, (2) a string to display when such value is selected, and (3) the type's value.
    options: Vec<(&'a str, Option<&'a str>, T)>,
}

impl<'a, T> Selection<'a, T>
where T: Copy
{
    /// Creates a new selection with a collection of tuples containing the following items:
    /// - the text to print
    /// - the thing that maps to that text (i.e. if that text is selected, the corresponding thing is returned by the [`prompt()`](Selection::prompt) function).
    pub fn new(selected_prefix: &'a str, options: Vec<(&'a str, Option<&'a str>, T)>) -> Self {
        Self {
            default_index: 0,
            padding: 0,
            clear_after_response: false,
            selected_prefix,
            options,
        }
    }

    /// Adds an option to the selection; consumes the calling instance and returning the transformed one.
    pub fn opt(self, tup: (&'a str, Option<&'a str>, T)) -> Self {
        // self.options.push(tup);
        // self
        let mut options_vec = self.options;
        options_vec.push(tup);
        Self {
            options: options_vec,
            ..self
        }
    }

    /// Sets the padding, or the number of lines that separates the selection from the text above it.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn padding(self, num_lines: usize) -> Self {
        Self {
            padding: num_lines,
            ..self
        }
    }

    /// Sets the prefix for the selected item.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn prefix(self, selected_prefix: &'a str) -> Self {
        Self {
            selected_prefix,
            ..self
        }
    }

    /// Sets the default option (the thing that's initially selected).
    /// Consumes the `Selection` and returns a transformed one.
    pub fn default_opt(self, default_index: usize) -> Self {
        Self {
            default_index,
            ..self
        }
    }

    /// Sets whether the prompt should be cleared after a response is given.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn clear_after_response(self, clear_after_response: bool) -> Self {
        Self {
            clear_after_response,
            ..self
        }
    }

    /// Prompts the user for an input by printing `msg` with `println!()`.
    /// This function will print the textual part of all options, and return the corresponding part represented by it (i.e. the value passed as `T`).
    pub fn prompt(&self, msg: &str) -> io::Result<(&'a str, Option<&'a str>, T)> {
        let term = Term::stdout();
        let mut selected_index = self.default_index;
        // use a selection dialog - consider console crate
        // index the vector by calling .get() and passing the index of the option chosen

        // loop to listen for keystrokes
            // on enter, return the result;
            // on arrow key, re-render the dialog and select the item that lies in the corresponding direction
        for _ in 0..self.padding {
            println!();
        }

        println!("{}", msg);

        // print lines to redraw over
        for _ in 0..self.options.len() {
            println!();
        }

        loop {
            // redraw over last x lines
            term.clear_last_lines(self.options.len())?;
            // print the items
            for (i, (s, selected_option, _)) in self.options.iter().enumerate() {
                println!("{}", if i == selected_index && selected_option.is_some() {format!("{}{}", self.selected_prefix, selected_option.unwrap()) } else {s.to_string()});
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

                    if self.clear_after_response {
                        term.clear_last_lines(self.options.len() + self.padding + 1)?; // + 1 implies we also want to clear the prompt line
                        // term.move_cursor_up(self.options.len() + 1)?;
                        // print!("cleared {} lines", self.options.len() + 1);
                        // stdout().flush()?;
                    }
                    term.show_cursor()?;
                    return Ok(tup);
                }
                _ => {}
            }
        }
    }
}