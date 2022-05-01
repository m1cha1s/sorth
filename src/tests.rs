use crate::prelude::{Engine, Standard, WordList};

#[test]
fn math_ops() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "2 3 + . 2 3 * . 4 2 / . 1 1 - .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "5 6 2 0 Ok.");
}

#[test]
fn custom_words() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = ": sq dup * ; 2 sq . 2 sq sq .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "4 16 Ok.");
}

#[test]
fn variables() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "let a @a 5 push @a pop . @a dup 6 push 9 push @a dup pop swap pop . .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "5 6 9 Ok.");
}
