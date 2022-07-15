use std::fmt::Display;
use std::io;

use console::{Key, Term};
use unicode_segmentation::UnicodeSegmentation;

/// Represents a single-select dialog.
#[must_use]
pub struct Select<C, D>
where
    C: Copy,
    D: Display,
{
    /// The index of the default option (e.g. 0 represents the first option in the `options` vector).
    default_index: usize,
    /// The number of lines that separates the prompt from other text.
    padding: usize,
    /// Determines if the selected and unselected answers should be aligned.
    is_aligned: bool,
    /// The prefix to print ahead of the currently selected item.
    prefix: D,
    /// Determines whether to clear the prompt after an answer is given.
    clear_after_response: bool,
    /// The options that the selection displays when prompting.
    options: Vec<SelectOpt<C, D>>,
}

/// Represents a single option in a [`Select`](Select).
pub struct SelectOpt<C, D>
where
    C: Copy,
    D: Display,
{
    /// The text to render by default.
    pub display_text: D,
    /// The text to render when the option is currently selected.
    pub selected_text: Option<D>,
    /// The value that the option represents. This will be returned by the prompter.
    pub value: C,
}

impl<C, D> SelectOpt<C, D>
where
    C: Copy,
    D: Display
{
    /// Constructs a new option.
    /// Prompts will return the `value` passed into this struct.
    pub fn new(display_text: D, selected_text: Option<D>, value: C) -> Self {
        Self {
            display_text,
            selected_text,
            value,
        }
    }
}

impl<C, D> Select<C, D>
where
    C: Copy,
    D: Display,
{
    /// Creates a new selection with a collection of tuples containing the following items:
    /// - the text to print
    /// - the thing that maps to that text (i.e. if that text is selected, the corresponding thing is returned by the [`prompt()`](Select::prompt) function).
    pub fn new(selected_prefix: D, options: Vec<SelectOpt<C, D>>) -> Self {

        Self {
            default_index: 0,
            padding: 0,
            is_aligned: false,
            prefix: selected_prefix,
            clear_after_response: false,
            options,
        }
    }

    /// Adds an option to the selection; consumes the calling instance and returns the transformed one.
    pub fn opt(self, select_opt: SelectOpt<C, D>) -> Self {
        let mut options_vec = self.options;
        options_vec.push(select_opt);
        Self {
            options: options_vec,
            ..self
        }
    }

    /// Sets the padding, or the number of lines that separates the selection from the text above it.
    /// Consumes the `Select` and returns a transformed one.
    pub fn padding(self, num_lines: usize) -> Self {
        Self {
            padding: num_lines,
            ..self
        }
    }

    /// Sets the prefix for the selected item.
    /// Consumes the `Select` and returns a transformed one.
    pub fn prefix(self, selected_prefix: D) -> Self {
        Self {
            prefix: selected_prefix,
            ..self
        }
    }

    /// Makes the options aligned together, instead of having to manually indent them in the selection's options. Note that added spaces in the default text may cause unexpected spacing.
    /// Consumes the `Select` and returns a transformed one.
    pub fn aligned(self) -> Self {
        Self {
            is_aligned: true,
            ..self
        }
    }

    /// Sets whether the prompt should be cleared after a response is given.
    /// Consumes the `Select` and returns a transformed one.
    pub fn clear_after(self) -> Self {
        Self {
            clear_after_response: true,
            ..self
        }
    }

    /// Sets the default option (the thing that's initially selected).
    /// Consumes the `Select` and returns a transformed one.
    pub fn default_opt(self, default_index: usize) -> Self {
        Self {
            default_index,
            ..self
        }
    }

    /// Prompts the user for an input by printing `msg` with `println!()`.
    /// This function will print the textual part of all options, and return the corresponding value represented by it (i.e. a `value` -- which conforms to type `C`).
    ///
    /// # Errors
    /// Propogates the following errors:
    /// - [`Term::read_key`]
    /// - [`Term::hide_cursor`]
    /// - [`Term::show_cursor`]
    /// - [`Term::clear_last_lines`]
    pub fn prompt(&self, msg: D) -> io::Result<C> {
        let term = Term::stdout();
        let mut selected_index = self.default_index;
        let prefix_char_count = self.prefix.to_string().decolored().graphemes(true).count();

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
            for (i, SelectOpt { display_text, selected_text, .. }) in self.options.iter().enumerate()
            {
                let s = match (i == selected_index, selected_text)
                {
                    (true, None) => format!("{}{}", self.prefix, display_text),
                    (true, Some(sel_str)) => format!("{}{}", self.prefix, sel_str),
                    _ => {
                        if self.is_aligned {
                            format!("{}{}", " ".repeat(prefix_char_count), display_text)
                        } else {
                            display_text.to_string() // TODO consider if dereferencing &str and calling str::to_string is faster than &str::to_string
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
                    let select_opt = self
                        .options
                        .get(selected_index)
                        .expect("unexpectedly failed to get selected item");

                    if self.clear_after_response {
                        term.clear_last_lines(self.options.len() + self.padding + 1)?; // + 1 implies we also want to clear the prompt line
                    }
                    term.show_cursor()?;
                    return Ok(select_opt.value);
                }
                _ => {}
            }
        }
    }
}

trait Decolor {
    /// Removes color escape sequences from a string.
    fn decolored(&self) -> Self;
}

impl Decolor for String {
    fn decolored(&self) -> Self {
        let mut s = Self::new();

        let mut tail_str = &self[..];
        while let Some(split_index) = tail_str.find(|c: char| c.is_ascii_control() && c == '\x1B')
        {
            let (start_str, split_str) = tail_str.split_at(split_index);

            if let Some(split_end_inc_index) = split_str.find('m') {
                s.push_str(start_str);
                tail_str = &split_str[split_end_inc_index+1..]; // excludes 'm' from the remaining string ref to parse
            }
        }
        s.push_str(tail_str); // if there aren't any other color codes, just concat the rest of the string since there's nothing to remove
        s
    }
}