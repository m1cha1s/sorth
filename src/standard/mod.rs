mod conditional_ops;
mod logic_ops;
mod loop_ops;
mod math_ops;
mod miscellaneus_ops;
mod stack_ops;
mod string_ops;
mod type_ops;
mod value_ops;
mod variable_ops;
mod word_ops;

use crate::prelude::{Engine, Word, WordList};

use self::{
    conditional_ops::{conditional_skip, current_cond, else_word, if_word, then_word},
    logic_ops::{and, equal, grater_than, less_than, not, not_equal, or},
    loop_ops::{again_word, bynext_word, do_word, for_word, i_word, next_word, while_word},
    math_ops::{add, divide, multiply, subtract},
    miscellaneus_ops::bye,
    stack_ops::{dot, drop_word, dup, peek, rot, swap_word, two_dup},
    string_ops::{
        string_concat, string_creation, string_mode_toggle, string_split, string_split_whitespace,
    },
    type_ops::{to_byte, to_double, to_float, to_int, to_long, to_string},
    value_ops::{byte_number, double_number, float_number, int_number, long_number},
    variable_ops::{
        get_from_index_word, get_var_addr_word, let_word, pop_word, push_word, set_in_index_word,
    },
    word_ops::{compile, end_compile, run_compiled, start_compile},
};

pub struct Standard {
    words: Vec<Word>,
}

impl WordList for Standard {
    fn new() -> Self {
        Standard {
            words: vec![
                // Conditional words
                (
                    |s| s.get_curr_word() == "if" && s.compiled_exec && s.mode_normal(),
                    if_word,
                ),
                (
                    |s| s.get_curr_word() == "else" && s.compiled_exec && s.mode_normal(),
                    else_word,
                ),
                (
                    |s| s.get_curr_word() == "then" && s.compiled_exec && s.mode_normal(),
                    then_word,
                ),
                (
                    |s: &Engine| 1 != current_cond(s) && s.mode_normal(),
                    conditional_skip,
                ),
                // Loop words
                (
                    |s| s.get_curr_word() == "for" && s.compiled_exec && s.mode_normal(),
                    for_word,
                ),
                (
                    |s| s.get_curr_word() == "next" && s.compiled_exec && s.mode_normal(),
                    next_word,
                ),
                (
                    |s| s.get_curr_word() == "bynext" && s.compiled_exec && s.mode_normal(),
                    bynext_word,
                ),
                (
                    |s| s.get_curr_word() == "while" && s.compiled_exec && s.mode_normal(),
                    while_word,
                ),
                (
                    |s| s.get_curr_word() == "do" && s.compiled_exec && s.mode_normal(),
                    do_word,
                ),
                (
                    |s| s.get_curr_word() == "again" && s.compiled_exec && s.mode_normal(),
                    again_word,
                ),
                (
                    |s| s.get_curr_word() == "i" && s.compiled_exec && s.mode_normal(),
                    i_word,
                ),
                // Variable words
                (|s| s.get_curr_word() == "let" && s.mode_normal(), let_word),
                (
                    |s| s.get_curr_word().starts_with("@") && s.mode_normal(),
                    get_var_addr_word,
                ),
                (
                    |s| s.get_curr_word() == "push" && s.mode_normal(),
                    push_word,
                ),
                (|s| s.get_curr_word() == "pop" && s.mode_normal(), pop_word),
                (
                    |s| s.get_curr_word() == "get" && s.mode_normal(),
                    get_from_index_word,
                ),
                (
                    |s| s.get_curr_word() == "set" && s.mode_normal(),
                    set_in_index_word,
                ),
                // Math operations
                (|s| s.get_curr_word() == "+" && s.mode_normal(), add),
                (|s| s.get_curr_word() == "-" && s.mode_normal(), subtract),
                (|s| s.get_curr_word() == "*" && s.mode_normal(), multiply),
                (|s| s.get_curr_word() == "/" && s.mode_normal(), divide),
                // Logic operations
                (|s| s.get_curr_word() == "==" && s.mode_normal(), equal),
                (|s| s.get_curr_word() == "!=" && s.mode_normal(), not_equal),
                (|s| s.get_curr_word() == "and" && s.mode_normal(), and),
                (|s| s.get_curr_word() == "or" && s.mode_normal(), or),
                (|s| s.get_curr_word() == "not" && s.mode_normal(), not),
                (|s| s.get_curr_word() == ">" && s.mode_normal(), grater_than),
                (|s| s.get_curr_word() == "<" && s.mode_normal(), less_than),
                // Stack operations
                (|s| s.get_curr_word() == "." && s.mode_normal(), dot),
                (|s| s.get_curr_word() == "dup" && s.mode_normal(), dup),
                (|s| s.get_curr_word() == "2dup" && s.mode_normal(), two_dup),
                (
                    |s| s.get_curr_word() == "drop" && s.mode_normal(),
                    drop_word,
                ),
                (
                    |s| s.get_curr_word() == "swap" && s.mode_normal(),
                    swap_word,
                ),
                (|s| s.get_curr_word() == "rot" && s.mode_normal(), rot),
                (|s| s.get_curr_word() == "peek" && s.mode_normal(), peek),
                (
                    |s| s.get_curr_word() == ":" && s.mode_normal(),
                    start_compile,
                ),
                (
                    |s: &Engine| {
                        s.compiled_words
                            .iter()
                            .any(|c| c.starts_with(s.get_curr_word().as_str()))
                            && s.mode_normal()
                    },
                    run_compiled,
                ),
                (|s| s.get_curr_word() == "bye" && s.mode_normal(), bye),
                // Type ops
                (|s| s.get_curr_word() == "to_int" && s.mode_normal(), to_int),
                (
                    |s| s.get_curr_word() == "to_long" && s.mode_normal(),
                    to_long,
                ),
                (
                    |s| s.get_curr_word() == "to_float" && s.mode_normal(),
                    to_float,
                ),
                (
                    |s| s.get_curr_word() == "to_double" && s.mode_normal(),
                    to_double,
                ),
                (
                    |s| s.get_curr_word() == "to_byte" && s.mode_normal(),
                    to_byte,
                ),
                (
                    |s| s.get_curr_word() == "to_str" && s.mode_normal(),
                    to_string,
                ),
                // String ops
                (
                    |s| s.get_curr_word() == "concat" && s.mode_normal(),
                    string_concat,
                ),
                (
                    |s| s.get_curr_word() == "split" && s.mode_normal(),
                    string_split,
                ),
                (
                    |s| s.get_curr_word() == "wsplit" && s.mode_normal(),
                    string_split_whitespace,
                ),
                // Read string
                (
                    |s| {
                        s.get_curr_word().starts_with("\"")
                            && s.get_curr_word().ends_with("\"")
                            && !(s.mode_comment() || s.mode_compile() || s.mode_see())
                    },
                    string_mode_toggle,
                ),
                (|s| s.mode_string(), string_creation),
                // Read number
                (
                    |s| s.get_curr_word().parse::<i32>().is_ok() && s.mode_normal(),
                    int_number,
                ),
                (
                    |s: &Engine| {
                        let mut word = s.get_curr_word();
                        word.pop();
                        word.parse::<i64>().is_ok()
                            && s.get_curr_word().to_lowercase().ends_with("l")
                            && s.mode_normal()
                    },
                    long_number,
                ),
                (
                    |s: &Engine| {
                        let mut word: String = s.get_curr_word();
                        word.pop();
                        word.parse::<f32>().is_ok()
                            && s.get_curr_word().to_lowercase().ends_with("f")
                            && s.mode_normal()
                    },
                    float_number,
                ),
                (
                    |s: &Engine| s.get_curr_word().parse::<f64>().is_ok() && s.mode_normal(),
                    double_number,
                ),
                (
                    |s: &Engine| {
                        s.get_curr_word().starts_with("0x")
                            && s.get_curr_word().len() == 4
                            && s.mode_normal()
                    },
                    byte_number,
                ),
                (|s| s.get_curr_word() != ";" && s.mode_compile(), compile),
                (
                    |s| s.get_curr_word() == ";" && s.mode_compile(),
                    end_compile,
                ),
            ],
        }
    }

    fn get_words(&self) -> Vec<Word> {
        self.words.clone()
    }
}
