use console::{Key, Term};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

/// Represents a single-select dialog.
#[must_use]
pub struct Selection<'a, T>
where
    T: Copy,
{
    /// The index of the default option (e.g. 0 represents the first optio in the `options` vector).
    default_index: usize,
    /// Number of lines that separates the prompt from other text
    padding: usize,
    /// Determines if the selected and unselected answers should be aligned.
    is_aligned: bool,
    /// The prefix to print ahead of the selected item.
    prefix: &'a str,
    /// Alternate prefix length to use in the case of external escape sequences (e.g. for colorizing).
    overridden_prefix_len: Option<usize>,
    /// Determines whether to clear the prompt after an answer is given.
    clear_after_response: bool,
    /// A vector of tuples that contain three values: (1) the string to display for the value by default, (2) a string to display when such value is selected, and (3) the type's value.
    options: Vec<(&'a str, Option<&'a str>, T)>,
}

impl<'a, T> Selection<'a, T>
where
    T: Copy,
{
    /// Creates a new selection with a collection of tuples containing the following items:
    /// - the text to print
    /// - the thing that maps to that text (i.e. if that text is selected, the corresponding thing is returned by the [`prompt()`](Selection::prompt) function).
    pub fn new(selected_prefix: &'a str, options: Vec<(&'a str, Option<&'a str>, T)>) -> Self {
        Self {
            default_index: 0,
            padding: 0,
            is_aligned: false,
            prefix: selected_prefix,
            overridden_prefix_len: None,
            clear_after_response: false,
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
    ///
    /// **It is strongly recommended to call [`override_prefix_len()`](Selection::override_prefix_len) when aligning with external escape sequences, particularly from color crates.**
    pub fn prefix(self, selected_prefix: &'a str) -> Self {
        Self {
            prefix: selected_prefix,
            ..self
        }
    }

    /// Sets the spacing for unselected items when alignment is toggled with [`aligned()`](Selection::aligned).
    ///
    /// **It is strongly recommended to call this method when aligning with external escape sequences, particularly from color crates.**
    ///
    /// Overrides the prefix length for the selected item. This value will be used for alignment if it is a value other than `None`.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn override_prefix_len(self, prefix_len: usize) -> Self {
        Self {
            overridden_prefix_len: Some(prefix_len),
            ..self
        }
    }

    /// Makes the options aligned together, instead of having to manually indent them in the selection's options. Note that added spaces in the default text may cause unexpected spacing.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn aligned(self) -> Self {
        Self {
            is_aligned: true,
            ..self
        }
    }

    /// Sets whether the prompt should be cleared after a response is given.
    /// Consumes the `Selection` and returns a transformed one.
    pub fn clear_after(self) -> Self {
        Self {
            clear_after_response: true,
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

    /// Prompts the user for an input by printing `msg` with `println!()`.
    /// This function will print the textual part of all options, and return the corresponding part represented by it (i.e. the value passed as `T`).
    ///
    /// # Errors
    /// Propogates the following errors:
    /// - [`Term::read_key`]
    /// - [`Term::hide_cursor`]
    /// - [`Term::show_cursor`]
    /// - [`Term::clear_last_lines`]
    pub fn prompt(&self, msg: &str) -> io::Result<(&'a str, Option<&'a str>, T)> {
        let term = Term::stdout();
        let mut selected_index = self.default_index;
        let prefix_char_count = self.overridden_prefix_len.map_or_else(|| self.prefix.graphemes(true).count(), |l| l);

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
            for (i, (displayed_str, selected_option, _)) in self.options.iter().enumerate() 
            {
                let s = match (i == selected_index, selected_option) {
                    (true, Some(sel_str)) => format!("{}{}", self.prefix, sel_str),
                    (true, None) => (*displayed_str).to_string(),
                    _ => {
                        if self.is_aligned {
                            let mut spacing = String::new();
                            for _ in 0..prefix_char_count {
                                spacing.push(' ');
                            }
                            format!("{}{}", spacing, displayed_str)
                        } else {
                            (*displayed_str).to_string() // dereferencing &str and calling str::to_string is faster than &str::to_string
                        }
                    }
                };

                println!("{}", s);

                // println!("{}", if i == selected_index && selected_option.is_some() {format!("{}{}", self.selected_prefix, selected_option.unwrap()) } else {s});
            }

            term.hide_cursor()?;

            // TODO consider integer wrapping
            match term.read_key()? {
                Key::ArrowUp => {
                    if selected_index as isize == -1 {
                        selected_index = self.options.len() - 1;
                    } else {
                        selected_index = ((selected_index as i32 - 1 + self.options.len() as i32)
                            % self.options.len() as i32)
                            as usize;
                    }
                }

                Key::ArrowDown => {
                    if selected_index as isize == -1 {
                        selected_index = 0;
                    } else {
                        selected_index =
                            ((selected_index as u64 + 1) % self.options.len() as u64) as usize;
                    }
                }

                Key::Enter => {
                    let tup = *self
                        .options
                        .get(selected_index)
                        .expect("unexpectedly failed to get selected item");

                    if self.clear_after_response {
                        term.clear_last_lines(self.options.len() + self.padding + 1)?; // + 1 implies we also want to clear the prompt line
                    }
                    term.show_cursor()?;
                    return Ok(tup);
                }
                _ => {}
            }
        }
    }
}
