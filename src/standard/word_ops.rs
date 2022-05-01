use crate::prelude::{Engine, EngineMode};

pub fn start_compile(s: &mut Engine) -> Result<String, String> {
    s.mode = EngineMode::COMPILE;
    Ok("".to_string())
}

pub fn end_compile(s: &mut Engine) -> Result<String, String> {
    s.new_compiled_word = s.new_compiled_word.trim().to_string();
    let new_word_name = s
        .new_compiled_word
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();

    let possibly_existing_word_position = s
        .compiled_words
        .iter()
        .position(|compiled| compiled.starts_with(new_word_name.as_str()));
    if possibly_existing_word_position.is_some() {
        s.compiled_words[possibly_existing_word_position.unwrap()] = s.new_compiled_word.clone();
    }

    s.compiled_words.push(s.new_compiled_word.clone());
    s.new_compiled_word.clear();
    s.mode = EngineMode::NORMAL;

    Ok("".to_string())
}

pub fn compile(s: &mut Engine) -> Result<String, String> {
    s.new_compiled_word += " ";
    s.new_compiled_word += s.get_curr_word().as_str();

    Ok("".to_string())
}

pub fn run_compiled(s: &mut Engine) -> Result<String, String> {
    let compiled_word = s
        .compiled_words
        .iter()
        .find(|compiled| compiled.starts_with(s.get_curr_word().as_str()))
        .unwrap();

    let mut word_copy = compiled_word.clone();

    for _ in 0..s.get_curr_word().len() {
        word_copy.remove(0);
    }

    s.compiled_exec = true;
    let ret = s.eval(word_copy);
    s.compiled_exec = false;

    ret
}
