use std::str::FromStr;
use crate::input;


/// Type used to validate a value of a type under one or more validation (boolean) functions.
pub struct Predicate<'a, T> {
    /// Function that determines whether the predicate passes.
    boxed_validation_fn: Box<dyn Fn(&T) -> bool>,
    /// Message passed back when validation fails.
    validation_msg: &'a str,
}

impl<'a, T> Predicate<'a, T> {
    /// Creates a [`Predicate`] with the following arguments:
    /// - A validation message `val_msg` that will be printed when validation fails
    /// - A validation function `val_fn`, wrapped in a [`Box`](std::boxed::Box), that will return a boolean indicating whether or not the the value being checked is valid
    pub fn new(validation_msg: &'a str, validation_fn: Box<dyn Fn(&T) -> bool>) -> Self {
        Self {
            validation_msg,
            boxed_validation_fn: validation_fn,
        }
    }

    /// Calls the predicate's validation function, returning a boolean indicating if `val` passed (is valid).
    pub fn validate(&self, val: &T) -> bool {
        (*self.boxed_validation_fn)(val)
    }

    /// Returns the predicate's validation message.
    pub fn invalid_msg(&self) -> &str {
        self.validation_msg
    }
}

/// Type that is used to...
/// 1. Validate **conversions** from a type that can be converted from a string (that is, it implements [`FromStr`])
/// 2. Validate the **value** of the type an input is being converted into.
///     - This is done using a vector of [`Predicate`]s.
/// 3. Print conversion or validation messages when validation fails in the cases above, in the following order:
///     - If the conversion fails, the conversion error message will be printed.
///     - Else, if any predicate fails, the validation message of the **first** predicate to fail will be printed.
/// 4. Continue prompting for input until both the conversion and every predicate passes.
///     - If the conversion or any predicate fails, the user will be prompted again.
///
/// Note that predicates are ordered first-in. In other words, the first predicate added to the prompter will be the first one tested (whereas the last one added will be tested last).
pub struct Prompter<'a, T>
    where T: FromStr,
{
    /// Message passed back when conversion fails.
    conversion_err_msg: &'a str,
    /// Collection of predicates to evaluate before accepting the input.
    predicates: Vec<Predicate<'a, T>>,
}

impl<'a, T> Prompter<'a, T>
    where T: FromStr
{
    /// Creates a `PromptBuilder` with a `conversion_err_msg` to print if the type conversion fails.
    ///
    /// Use the [`pred()`](PromptBuilder::pred) function to add [`Predicate`]s to use when validating the value of the converted type, and the ['build()'](PromptBuilder::build) function to construct the final `Prompter` instance.
    pub fn builder(conversion_err_msg: &'a str) -> PromptBuilder<'a, T> {
        PromptBuilder {
            conversion_err_msg,
            predicates: Vec::new(),
        }
    }

    /// Prompts the user for an input.
    /// This function will continue prompting if either the user's input cannot be converted to the desired type or if any of the predicates fail.
    pub fn prompt(&self, msg: &str) -> T
    {
        'input: loop
        {
            match input(msg) // essentially, if this matches an Err(_) result, repeat the loop. The error shouldn't make the program panic.
            {
                Ok(val) => { // now actually convert the value and test the predicates.
                    match val.trim().parse::<T>()
                    {
                        Ok(val) => {
                            for p in &self.predicates
                            {
                                if !p.validate(&val)
                                {
                                    println!("{}", p.invalid_msg());
                                    continue 'input;
                                }
                            } // at this point, all predicates pass
                            return val;
                        },
                        Err(_) => {
                            println!("{}", self.conversion_err_msg);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Something went wrong with reading the input.");
                }
            }
        }
    }
}

/// Type used to create a [`Prompter`].
///
/// Call the [`pred()`](PromptBuilder::pred) function on this struct to add a [`Predicate`].
///
/// Call [`build()`](PromptBuilder::build) to get the prompter back with the predicates added in the way above.
pub struct PromptBuilder<'a, T>
    where T: FromStr
{
    conversion_err_msg: &'a str,
    predicates: Vec<Predicate<'a, T>>,
}

impl<'a, T> PromptBuilder<'a, T>
    where T: FromStr
{

    /// Consumes the existing `Prompter` and returns a new `Prompter` with the predicate.
    pub fn pred(mut self, predicate: Predicate<'a, T>) -> Self {
        self.predicates.push(predicate);
        self
    }

    /// Builds a `Prompter` with the properties of a `PromptBuilder` instance (namely, the `Predicate`s). The `PromptBuilder` being passed is consumed.
    pub fn build(self) -> Prompter<'a, T> {
        Prompter {
            conversion_err_msg: self.conversion_err_msg,
            predicates: self.predicates
        }
    }
}