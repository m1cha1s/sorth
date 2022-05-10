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

    assert_eq!(output.as_str(), "5 6 2 0\nOk.");
}

#[test]
fn logic_ops() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "2 3 > . 2 3 < . 4 2 != . 1 1 == .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "-1 0 -1 -1\nOk.");
}

#[test]
fn stack_ops() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "2 dup . . 2 3 2dup . . . . 4 2 swap . . 1 2 3 rot . . . 7 peek .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "2 2 3 2 3 2 4 2 1 3 2 7 7\nOk.");
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

    assert_eq!(output.as_str(), "4 16\nOk.");
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

    assert_eq!(output.as_str(), "5 6 9\nOk.");
}

#[test]
fn strings() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "\" hello \" \" world \" 2dup + peek wsplit . . concat .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "hello world world hello helloworld\nOk.");
}

#[test]
fn type_conversion() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line = "17 to_long peek to_float peek to_double peek to_byte peek to_str .".to_string();

    let mut output = String::new();

    match engine.eval(line) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "17 17 17 17 17\nOk.");
}

#[test]
fn input() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line1 = "\" abc \" input".to_string();
    let line2 = "cba".to_string();
    let line3 = ".".to_string();

    let mut output = String::new();

    match engine.eval(line1) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "abc\nOk.");

    output.drain(..);

    match engine.eval(line2) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "Ok.");

    output.drain(..);

    match engine.eval(line3) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "cba\nOk.");

    output.drain(..);
}

#[test]
fn silent() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    let line1 = "17".to_string();
    let line2 = "-1 silent".to_string();
    let line3 = ".".to_string();
    let line4 = "0 silent".to_string();

    let mut output = String::new();

    match engine.eval(line1) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "Ok.");
    output.drain(..);

    match engine.eval(line2) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "");
    output.drain(..);

    match engine.eval(line3) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "17");
    output.drain(..);

    match engine.eval(line4) {
        Ok(ok) => output += ok.trim(),
        Err(err) => output += err.trim(),
    }

    assert_eq!(output.as_str(), "Ok.");
    output.drain(..);
}
