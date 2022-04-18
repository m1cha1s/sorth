use super::prelude::Sorth;

struct Standard {
    words: Vec<(String, fn(s: &mut Sorth) -> Result<String, String>)>,
}

impl Standard {
    fn new() -> Self {
        Standard {
            words: vec![
                ("+".to_string(), add),
                ("-".to_string(), subtract),
                ("*".to_string(), multiply),
                ("/".to_string(), divide),
                ("=".to_string(), equal),
                ("<>".to_string(), not_equal),
                ("and".to_string(), and),
                ("or".to_string(), or),
                (">".to_string(), grater_than),
                ("<".to_string(), less_than),
            ],
        }
    }
}

fn add(s: &mut Sorth) -> Result<String, String> {
    let x = s.int_stack.pop();
    match x {
        Some(_) => {}
        None => return Err("Error <std_add> int stack underflow".to_string()),
    }

    let y = s.int_stack.pop();
    match y {
        Some(_) => {}
        None => return Err("Error <std_add> int stack underflow".to_string()),
    }

    let result = y.unwrap() + x.unwrap();

    s.int_stack.push(result);

    Ok("".to_string())
}

fn subtract(s: &mut Sorth) -> Result<String, String> {
    let x = s.int_stack.pop();
    match x {
        Some(_) => {}
        None => return Err("Error <std_subtract> int stack underflow".to_string()),
    }

    let y = s.int_stack.pop();
    match y {
        Some(_) => {}
        None => return Err("Error <std_subtract> int stack underflow".to_string()),
    }

    let result = y.unwrap() - x.unwrap();

    s.int_stack.push(result);

    Ok("".to_string())
}

fn multiply(s: &mut Sorth) -> Result<String, String> {
    let x = s.int_stack.pop();
    match x {
        Some(_) => {}
        None => return Err("Error <std_multiply> int stack underflow".to_string()),
    }

    let y = s.int_stack.pop();
    match y {
        Some(_) => {}
        None => return Err("Error <std_multiply> int stack underflow".to_string()),
    }

    let result = y.unwrap() * x.unwrap();

    s.int_stack.push(result);

    Ok("".to_string())
}

fn divide(s: &mut Sorth) -> Result<String, String> {
    let x = s.int_stack.pop();
    match x {
        Some(_) => {}
        None => return Err("Error <std_divide> int stack underflow".to_string()),
    }

    let y = s.int_stack.pop();
    match y {
        Some(_) => {}
        None => return Err("Error <std_divide> int stack underflow".to_string()),
    }

    let result = y.unwrap() / x.unwrap();

    s.int_stack.push(result);

    Ok("".to_string())
}

// TODO Exception handling from here down on
// TODO Rework these functions to have proper return values

fn dup(s: &mut Sorth) {
    let head = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(head);
    s.int_stack.push(head);
}

fn two_dup(s: &mut Sorth) {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(y);
    s.int_stack.push(x);
    s.int_stack.push(y);
    s.int_stack.push(x);
}

fn drop(s: &mut Sorth) {
    let _ = s.int_stack.pop().unwrap_or(0);
}

fn swap(s: &mut Sorth) {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(x);
    s.int_stack.push(y);
}

fn rot(s: &mut Sorth) {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    let z = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(y);
    s.int_stack.push(x);
    s.int_stack.push(z);
}

fn dot(s: &mut Sorth) -> String {
    let head = s.int_stack.pop().unwrap_or(0);
    head.to_string()
}

fn peek(s: &mut Sorth) -> String {
    s.int_stack.last().unwrap_or(&0).to_string()
}

fn start_compile(s: &mut Sorth) {
    s.compile_mode = true;
}

fn end_compile(s: &mut Sorth) {
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
    s.compile_mode = false;
}

fn compile(s: &mut Sorth, word: &str) {
    s.new_compiled_word += " ";
    s.new_compiled_word += word;
}

fn run_compiled(s: &mut Sorth, word: &str) -> Result<String, String> {
    let compiled_word = s
        .compiled_words
        .iter()
        .find(|compiled| compiled.starts_with(word))
        .unwrap();

    let mut word_copy = compiled_word.clone();

    for _ in 0..word.len() {
        word_copy.remove(0);
    }

    return s.eval(word_copy, true);
}

fn equal(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a == b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}

fn not_equal(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a != b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}

fn and(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a == b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}

fn or(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if (a == -1) || (b == -1) {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}

fn grate_than(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a > b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}

fn less_than(s: &mut Sorth) {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a < b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }
}
