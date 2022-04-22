use super::prelude::{Engine, EngineMode, Word, WordList};

pub struct Standard {
    normal: Vec<Word>,
    compiled: Vec<Word>,
    see: Vec<Word>,
    comment: Vec<Word>,
}

impl WordList for Standard {
    fn new() -> Self {
        Standard {
            normal: vec![
                // Conditional words
                (|s| s.get_curr_word() == "if" && s.compiled_exec, if_word),
                (
                    |s| s.get_curr_word() == "else" && s.compiled_exec,
                    else_word,
                ),
                (
                    |s| s.get_curr_word() == "then" && s.compiled_exec,
                    then_word,
                ),
                (|s: &Engine| 1 != current_cond(s), conditional_skip),
                // Loop words
                (|s| s.get_curr_word() == "do" && s.compiled_exec, do_word),
                (
                    |s| s.get_curr_word() == "loop" && s.compiled_exec,
                    loop_word,
                ),
                (|s| s.get_curr_word() == "i" && s.compiled_exec, i_word),
                // Rest of words
                (|s| s.get_curr_word() == "+", add),
                (|s| s.get_curr_word() == "-", subtract),
                (|s| s.get_curr_word() == "*", multiply),
                (|s| s.get_curr_word() == "/", divide),
                (|s| s.get_curr_word() == "=", equal),
                (|s| s.get_curr_word() == "<>", not_equal),
                (|s| s.get_curr_word() == "and", and),
                (|s| s.get_curr_word() == "or", or),
                (|s| s.get_curr_word() == ">", grater_than),
                (|s| s.get_curr_word() == "<", less_than),
                (|s| s.get_curr_word() == ".", dot),
                (|s| s.get_curr_word() == "dup", dup),
                (|s| s.get_curr_word() == "2dup", two_dup),
                (|s| s.get_curr_word() == "drop", drop),
                (|s| s.get_curr_word() == "swap", swap),
                (|s| s.get_curr_word() == "rot", rot),
                (|s| s.get_curr_word() == "peek", peek),
                (|s| s.get_curr_word() == ":", start_compile),
                (
                    |s: &Engine| {
                        s.compiled_words
                            .iter()
                            .any(|c| c.starts_with(s.get_curr_word().as_str()))
                    },
                    run_compiled,
                ),
                (|s| s.get_curr_word() == "bye", bye),
                // Read number
                (|s| s.get_curr_word().parse::<i32>().is_ok(), int_number),
            ],
            compiled: vec![
                (|s| s.get_curr_word() != ";", compile),
                (|s| s.get_curr_word() == ";", end_compile),
            ],
            see: vec![],
            comment: vec![],
        }
    }

    fn get_normal(&self) -> Vec<Word> {
        self.normal.clone()
    }

    fn get_compile(&self) -> Vec<Word> {
        self.compiled.clone()
    }

    fn get_see(&self) -> Vec<Word> {
        self.see.clone()
    }

    fn get_comment(&self) -> Vec<Word> {
        self.comment.clone()
    }
}

fn int_number(s: &mut Engine) -> Result<String, String> {
    let number = s.get_curr_word().parse::<i32>().unwrap();
    s.int_stack.push(number);
    Ok("".to_string())
}

// Loop logic

fn do_word(s: &mut Engine) -> Result<String, String> {
    let index = s.int_stack.pop();
    match index {
        Some(_) => {}
        None => return Err("Error Int stack underflow!".to_string()),
    }
    let limit = s.int_stack.pop();
    match limit {
        Some(_) => {}
        None => return Err("Error Int stack underflow!".to_string()),
    }

    s.loop_stack.push((limit.unwrap(), index.unwrap()));

    Ok("".to_string())
}

fn loop_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.pop();
    match curr_loop {
        Some(_) => {}
        None => return Err("Error Loop control stack underflow!".to_string()),
    }

    let mut curr_loop_contents = curr_loop.unwrap();
    curr_loop_contents.1 += 1;

    if curr_loop_contents.1 < curr_loop_contents.0 {
        let mut cntr = 1;
        while cntr != 0 {
            s.curr_word_idx -= 1;

            if s.get_curr_word() == "loop" {
                cntr += 1;
            }

            if s.get_curr_word() == "do" {
                cntr -= 1;
            }
        }
        s.loop_stack.push(curr_loop_contents);
    }

    Ok("".to_string())
}

fn i_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.last();

    match curr_loop {
        Some(_) => {}
        None => return Err("Error Loop control stack underflow!".to_string()),
    }

    s.int_stack.push(curr_loop.unwrap().1);

    Ok("".to_string())
}

// Conditional logic

fn current_cond(s: &Engine) -> i8 {
    match s.conditional_stack.last() {
        Some(&val) => val,
        None => 1,
    }
}

fn if_word(s: &mut Engine) -> Result<String, String> {
    if current_cond(s) != 1 {
        s.conditional_stack.push(0);
        return Ok("".to_string());
    }

    let cond = s.int_stack.pop();
    match cond {
        Some(cond_val) => {
            if cond_val == -1 {
                s.conditional_stack.push(1);
            } else if cond_val == 0 {
                s.conditional_stack.push(2);
            }
        }
        None => return Err("Error Int stack underflow!".to_string()),
    }
    Ok("".to_string())
}

fn else_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.conditional_stack.pop();
    match cond {
        Some(cond_val) => {
            s.conditional_stack.push(cond_val - 1);
        }
        None => return Err("Error Conditional stack underflow!".to_string()),
    }
    Ok("".to_string())
}

fn then_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.conditional_stack.pop();
    match cond {
        Some(_) => {}
        None => return Err("Error Conditional stack underflow!".to_string()),
    }
    Ok("".to_string())
}

fn conditional_skip(_s: &mut Engine) -> Result<String, String> {
    Ok("".to_string())
}

// Rest of logic

fn bye(s: &mut Engine) -> Result<String, String> {
    if !s.compiled_exec {
        s.running = false;
    } else {
        s.compiled_exec = false;
    }
    Ok("".to_string())
}

fn add(s: &mut Engine) -> Result<String, String> {
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

fn subtract(s: &mut Engine) -> Result<String, String> {
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

fn multiply(s: &mut Engine) -> Result<String, String> {
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

fn divide(s: &mut Engine) -> Result<String, String> {
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

fn dup(s: &mut Engine) -> Result<String, String> {
    let head = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(head);
    s.int_stack.push(head);
    Ok("".to_string())
}

fn two_dup(s: &mut Engine) -> Result<String, String> {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(y);
    s.int_stack.push(x);
    s.int_stack.push(y);
    s.int_stack.push(x);
    Ok("".to_string())
}

fn drop(s: &mut Engine) -> Result<String, String> {
    let _ = s.int_stack.pop().unwrap_or(0);
    Ok("".to_string())
}

fn swap(s: &mut Engine) -> Result<String, String> {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(x);
    s.int_stack.push(y);
    Ok("".to_string())
}

fn rot(s: &mut Engine) -> Result<String, String> {
    let x = s.int_stack.pop().unwrap_or(0);
    let y = s.int_stack.pop().unwrap_or(0);
    let z = s.int_stack.pop().unwrap_or(0);
    s.int_stack.push(y);
    s.int_stack.push(x);
    s.int_stack.push(z);
    Ok("".to_string())
}

fn dot(s: &mut Engine) -> Result<String, String> {
    let head = s.int_stack.pop().unwrap_or(0);
    Ok(head.to_string())
}

fn peek(s: &mut Engine) -> Result<String, String> {
    Ok(s.int_stack.last().unwrap_or(&0).to_string())
}

fn start_compile(s: &mut Engine) -> Result<String, String> {
    s.mode = EngineMode::COMPILE;
    Ok("".to_string())
}

fn end_compile(s: &mut Engine) -> Result<String, String> {
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

fn compile(s: &mut Engine) -> Result<String, String> {
    s.new_compiled_word += " ";
    s.new_compiled_word += s.get_curr_word().as_str();

    Ok("".to_string())
}

fn run_compiled(s: &mut Engine) -> Result<String, String> {
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

fn equal(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a == b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}

fn not_equal(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a != b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}

fn and(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a == b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}

fn or(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if (a == -1) || (b == -1) {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}

fn grater_than(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a > b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}

fn less_than(s: &mut Engine) -> Result<String, String> {
    let a = s.int_stack.pop().unwrap_or(0);
    let b = s.int_stack.pop().unwrap_or(0);

    if a < b {
        s.int_stack.push(-1);
    } else {
        s.int_stack.push(0);
    }

    Ok("".to_string())
}
