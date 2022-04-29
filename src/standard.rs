use std::{thread::sleep, time::Duration};

use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Engine, EngineMode, Types, Word, WordList},
};

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
                (|s| s.get_curr_word() == "for" && s.compiled_exec, for_word),
                (
                    |s| s.get_curr_word() == "next" && s.compiled_exec,
                    next_word,
                ),
                (
                    |s| s.get_curr_word() == "bynext" && s.compiled_exec,
                    bynext_word,
                ),
                (
                    |s| s.get_curr_word() == "while" && s.compiled_exec,
                    while_word,
                ),
                (|s| s.get_curr_word() == "do" && s.compiled_exec, do_word),
                (
                    |s| s.get_curr_word() == "again" && s.compiled_exec,
                    again_word,
                ),
                (|s| s.get_curr_word() == "i" && s.compiled_exec, i_word),
                // Rest of words
                (|s| s.get_curr_word() == "+", add),
                (|s| s.get_curr_word() == "-", subtract),
                (|s| s.get_curr_word() == "*", multiply),
                (|s| s.get_curr_word() == "/", divide),
                (|s| s.get_curr_word() == "==", equal),
                (|s| s.get_curr_word() == "!=", not_equal),
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
    s.main_stack.push(Types::Int(number));
    Ok("".to_string())
}

// Loop logic

fn while_word(_s: &mut Engine) -> Result<String, String> {
    Ok("".to_string())
}

fn do_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.main_stack.pop();
    if cond.is_none() {
        return Err("Error Int stack underflow!".to_string());
    }

    match cond.unwrap() {
        Types::Int(x) => {
            if x == 0 {
                let mut controll = 1;
                while controll != 0 {
                    s.curr_word_idx += 1;

                    match s.get_curr_word().as_str() {
                        "while" => controll += 1,
                        "again" => controll -= 1,
                        _ => {}
                    }
                }
            }
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

fn again_word(s: &mut Engine) -> Result<String, String> {
    let mut controll = 1;
    while controll > 0 {
        s.curr_word_idx -= 1;

        match s.get_curr_word().as_str() {
            "while" => controll -= 1,
            "again" => controll += 1,
            _ => {}
        }
    }

    Ok("".to_string())
}

fn for_word(s: &mut Engine) -> Result<String, String> {
    let index = s.main_stack.pop();
    if index.is_none() {
        return Err("Error Int stack underflow!".to_string());
    }

    let limit = s.main_stack.pop();
    if limit.is_none() {
        return Err("Error Int stack underflow!".to_string());
    }

    let index_int: i32;

    match index.unwrap() {
        Types::Int(x) => index_int = x,
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    let limit_int: i32;

    match limit.unwrap() {
        Types::Int(x) => limit_int = x,
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    s.loop_stack.push((limit_int, index_int));

    Ok("".to_string())
}

fn next_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.pop();
    if curr_loop.is_none() {
        return Err("Error Loop control stack underflow!".to_string());
    }

    let mut curr_loop_contents = curr_loop.unwrap();
    curr_loop_contents.1 += 1;

    if curr_loop_contents.1 < curr_loop_contents.0 {
        let mut cntr = 1;
        while cntr != 0 {
            s.curr_word_idx -= 1;

            if s.get_curr_word() == "next" || s.get_curr_word() == "bynext" {
                cntr += 1;
            }

            if s.get_curr_word() == "for" {
                cntr -= 1;
            }
        }
        s.loop_stack.push(curr_loop_contents);
    }

    Ok("".to_string())
}

fn bynext_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.pop();
    match curr_loop {
        Some(_) => {}
        None => return Err("Error Loop control stack underflow!".to_string()),
    }

    let mut curr_loop_contents = curr_loop.unwrap();

    let increment = s.main_stack.pop();
    if increment.is_none() {
        return Err("Error Int stack underflow!".to_string());
    }

    let loop_increment: i32;

    match increment.unwrap() {
        Types::Int(x) => loop_increment = x,
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    curr_loop_contents.1 += loop_increment;

    if curr_loop_contents.1 < curr_loop_contents.0 {
        let mut cntr = 1;
        while cntr != 0 {
            s.curr_word_idx -= 1;

            if s.get_curr_word() == "next" || s.get_curr_word() == "bynext" {
                cntr += 1;
            }

            if s.get_curr_word() == "for" {
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

    s.main_stack.push(Types::Int(curr_loop.unwrap().1));

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

    let cond = s.main_stack.pop();
    match cond {
        Some(cond_val) => match cond_val {
            Types::Int(val) => {
                if val == -1 {
                    s.conditional_stack.push(1);
                } else if val == 0 {
                    s.conditional_stack.push(2);
                }
            }
            _ => return Err(INVALID_TYPE_ERROR.to_string()),
        },
        None => return Err(STACK_UNDERFLOW_ERROR.to_string()),
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
    let x = s.main_stack.pop();
    if x.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let y = s.main_stack.pop();
    if y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (y.unwrap(), x.unwrap()) {
        (Types::Int(a), Types::Int(b)) => s.main_stack.push(Types::Int(a + b)),
        (Types::Int(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 + b)),
        (Types::Int(a), Types::Byte(b)) => s.main_stack.push(Types::Int(a + b as i32)),
        (Types::Int(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 + b)),
        (Types::Int(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 + b)),

        (Types::Float(a), Types::Int(b)) => s.main_stack.push(Types::Float(a + b as f32)),
        (Types::Float(a), Types::Float(b)) => s.main_stack.push(Types::Float(a + b)),
        (Types::Float(a), Types::Byte(b)) => s.main_stack.push(Types::Float(a + b as f32)),
        (Types::Float(a), Types::Long(b)) => s.main_stack.push(Types::Float(a + b as f32)),
        (Types::Float(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 + b)),

        (Types::Byte(a), Types::Int(b)) => s.main_stack.push(Types::Int(a as i32 + b)),
        (Types::Byte(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 + b)),
        (Types::Byte(a), Types::Byte(b)) => s.main_stack.push(Types::Byte(a + b)),
        (Types::Byte(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 + b)),
        (Types::Byte(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 + b)),

        (Types::Long(a), Types::Int(b)) => s.main_stack.push(Types::Long(a + b as i64)),
        (Types::Long(a), Types::Long(b)) => s.main_stack.push(Types::Long(a + b)),
        (Types::Long(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 + b)),
        (Types::Long(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 + b)),
        (Types::Long(a), Types::Byte(b)) => s.main_stack.push(Types::Long(a + b as i64)),

        (Types::Double(a), Types::Int(b)) => s.main_stack.push(Types::Double(a + b as f64)),
        (Types::Double(a), Types::Long(b)) => s.main_stack.push(Types::Double(a + b as f64)),
        (Types::Double(a), Types::Float(b)) => s.main_stack.push(Types::Double(a + b as f64)),
        (Types::Double(a), Types::Double(b)) => s.main_stack.push(Types::Double(a + b)),
        (Types::Double(a), Types::Byte(b)) => s.main_stack.push(Types::Double(a + b as f64)),

        (Types::Str(a), Types::Str(b)) => s.main_stack.push(Types::Str(a + b.as_str())),
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

fn subtract(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    if x.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let y = s.main_stack.pop();
    if y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (y.unwrap(), x.unwrap()) {
        (Types::Int(a), Types::Int(b)) => s.main_stack.push(Types::Int(a - b)),
        (Types::Int(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 - b)),
        (Types::Int(a), Types::Byte(b)) => s.main_stack.push(Types::Int(a - b as i32)),
        (Types::Int(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 - b)),
        (Types::Int(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 - b)),

        (Types::Float(a), Types::Int(b)) => s.main_stack.push(Types::Float(a - b as f32)),
        (Types::Float(a), Types::Float(b)) => s.main_stack.push(Types::Float(a - b)),
        (Types::Float(a), Types::Byte(b)) => s.main_stack.push(Types::Float(a - b as f32)),
        (Types::Float(a), Types::Long(b)) => s.main_stack.push(Types::Float(a - b as f32)),
        (Types::Float(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 - b)),

        (Types::Byte(a), Types::Int(b)) => s.main_stack.push(Types::Int(a as i32 - b)),
        (Types::Byte(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 - b)),
        (Types::Byte(a), Types::Byte(b)) => s.main_stack.push(Types::Byte(a - b)),
        (Types::Byte(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 - b)),
        (Types::Byte(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 - b)),

        (Types::Long(a), Types::Int(b)) => s.main_stack.push(Types::Long(a - b as i64)),
        (Types::Long(a), Types::Long(b)) => s.main_stack.push(Types::Long(a - b)),
        (Types::Long(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 - b)),
        (Types::Long(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 - b)),
        (Types::Long(a), Types::Byte(b)) => s.main_stack.push(Types::Long(a - b as i64)),

        (Types::Double(a), Types::Int(b)) => s.main_stack.push(Types::Double(a - b as f64)),
        (Types::Double(a), Types::Long(b)) => s.main_stack.push(Types::Double(a - b as f64)),
        (Types::Double(a), Types::Float(b)) => s.main_stack.push(Types::Double(a - b as f64)),
        (Types::Double(a), Types::Double(b)) => s.main_stack.push(Types::Double(a - b)),
        (Types::Double(a), Types::Byte(b)) => s.main_stack.push(Types::Double(a - b as f64)),

        //(Types::Str(a), Types::Str(b)) => s.main_stack.push(Types::Str(a + b.as_str())),
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

fn multiply(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    if x.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let y = s.main_stack.pop();
    if y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (y.unwrap(), x.unwrap()) {
        (Types::Int(a), Types::Int(b)) => s.main_stack.push(Types::Int(a * b)),
        (Types::Int(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 * b)),
        (Types::Int(a), Types::Byte(b)) => s.main_stack.push(Types::Int(a * b as i32)),
        (Types::Int(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 * b)),
        (Types::Int(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 * b)),

        (Types::Float(a), Types::Int(b)) => s.main_stack.push(Types::Float(a * b as f32)),
        (Types::Float(a), Types::Float(b)) => s.main_stack.push(Types::Float(a * b)),
        (Types::Float(a), Types::Byte(b)) => s.main_stack.push(Types::Float(a * b as f32)),
        (Types::Float(a), Types::Long(b)) => s.main_stack.push(Types::Float(a * b as f32)),
        (Types::Float(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 * b)),

        (Types::Byte(a), Types::Int(b)) => s.main_stack.push(Types::Int(a as i32 * b)),
        (Types::Byte(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 * b)),
        (Types::Byte(a), Types::Byte(b)) => s.main_stack.push(Types::Byte(a * b)),
        (Types::Byte(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 * b)),
        (Types::Byte(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 * b)),

        (Types::Long(a), Types::Int(b)) => s.main_stack.push(Types::Long(a * b as i64)),
        (Types::Long(a), Types::Long(b)) => s.main_stack.push(Types::Long(a * b)),
        (Types::Long(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 * b)),
        (Types::Long(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 * b)),
        (Types::Long(a), Types::Byte(b)) => s.main_stack.push(Types::Long(a * b as i64)),

        (Types::Double(a), Types::Int(b)) => s.main_stack.push(Types::Double(a * b as f64)),
        (Types::Double(a), Types::Long(b)) => s.main_stack.push(Types::Double(a * b as f64)),
        (Types::Double(a), Types::Float(b)) => s.main_stack.push(Types::Double(a * b as f64)),
        (Types::Double(a), Types::Double(b)) => s.main_stack.push(Types::Double(a * b)),
        (Types::Double(a), Types::Byte(b)) => s.main_stack.push(Types::Double(a * b as f64)),

        //(Types::Str(a), Types::Str(b)) => s.main_stack.push(Types::Str(a + b.as_str())),
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

fn divide(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    if x.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let y = s.main_stack.pop();
    if y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (y.unwrap(), x.unwrap()) {
        (Types::Int(a), Types::Int(b)) => s.main_stack.push(Types::Int(a / b)),
        (Types::Int(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 / b)),
        (Types::Int(a), Types::Byte(b)) => s.main_stack.push(Types::Int(a / b as i32)),
        (Types::Int(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 / b)),
        (Types::Int(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 / b)),

        (Types::Float(a), Types::Int(b)) => s.main_stack.push(Types::Float(a / b as f32)),
        (Types::Float(a), Types::Float(b)) => s.main_stack.push(Types::Float(a / b)),
        (Types::Float(a), Types::Byte(b)) => s.main_stack.push(Types::Float(a / b as f32)),
        (Types::Float(a), Types::Long(b)) => s.main_stack.push(Types::Float(a / b as f32)),
        (Types::Float(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 / b)),

        (Types::Byte(a), Types::Int(b)) => s.main_stack.push(Types::Int(a as i32 / b)),
        (Types::Byte(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 / b)),
        (Types::Byte(a), Types::Byte(b)) => s.main_stack.push(Types::Byte(a / b)),
        (Types::Byte(a), Types::Long(b)) => s.main_stack.push(Types::Long(a as i64 / b)),
        (Types::Byte(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 / b)),

        (Types::Long(a), Types::Int(b)) => s.main_stack.push(Types::Long(a / b as i64)),
        (Types::Long(a), Types::Long(b)) => s.main_stack.push(Types::Long(a / b)),
        (Types::Long(a), Types::Float(b)) => s.main_stack.push(Types::Float(a as f32 / b)),
        (Types::Long(a), Types::Double(b)) => s.main_stack.push(Types::Double(a as f64 / b)),
        (Types::Long(a), Types::Byte(b)) => s.main_stack.push(Types::Long(a / b as i64)),

        (Types::Double(a), Types::Int(b)) => s.main_stack.push(Types::Double(a / b as f64)),
        (Types::Double(a), Types::Long(b)) => s.main_stack.push(Types::Double(a / b as f64)),
        (Types::Double(a), Types::Float(b)) => s.main_stack.push(Types::Double(a / b as f64)),
        (Types::Double(a), Types::Double(b)) => s.main_stack.push(Types::Double(a / b)),
        (Types::Double(a), Types::Byte(b)) => s.main_stack.push(Types::Double(a / b as f64)),

        //(Types::Str(a), Types::Str(b)) => s.main_stack.push(Types::Str(a + b.as_str())),
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

fn dup(s: &mut Engine) -> Result<String, String> {
    let head = s.main_stack.pop();

    if head.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let head_val = head.unwrap();

    s.main_stack.push(head_val.clone());
    s.main_stack.push(head_val);
    Ok("".to_string())
}

fn two_dup(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    let y = s.main_stack.pop();

    if x.is_none() || y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let x_val = x.unwrap();
    let y_val = y.unwrap();

    s.main_stack.push(y_val.clone());
    s.main_stack.push(x_val.clone());
    s.main_stack.push(y_val);
    s.main_stack.push(x_val);
    Ok("".to_string())
}

fn drop(s: &mut Engine) -> Result<String, String> {
    let _ = s.main_stack.pop();
    Ok("".to_string())
}

fn swap(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    let y = s.main_stack.pop();

    if x.is_none() || y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    s.main_stack.push(x.unwrap());
    s.main_stack.push(y.unwrap());
    Ok("".to_string())
}

fn rot(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    let y = s.main_stack.pop();
    let z = s.main_stack.pop();

    if x.is_none() || y.is_none() || z.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    s.main_stack.push(y.unwrap());
    s.main_stack.push(x.unwrap());
    s.main_stack.push(z.unwrap());
    Ok("".to_string())
}

fn dot(s: &mut Engine) -> Result<String, String> {
    let head = s.main_stack.pop();

    if head.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match head.unwrap() {
        Types::Int(a) => Ok(a.to_string()),
        Types::Long(a) => Ok(a.to_string()),
        Types::Float(a) => Ok(a.to_string()),
        Types::Double(a) => Ok(a.to_string()),
        Types::Byte(a) => Ok(a.to_string()),
        Types::Str(a) => Ok(a),
    }
}

fn peek(s: &mut Engine) -> Result<String, String> {
    let last = s.main_stack.last();

    if last.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match last.unwrap() {
        Types::Int(a) => Ok(a.to_string()),
        Types::Long(a) => Ok(a.to_string()),
        Types::Float(a) => Ok(a.to_string()),
        Types::Double(a) => Ok(a.to_string()),
        Types::Byte(a) => Ok(a.to_string()),
        Types::Str(a) => Ok(a.to_string()),
    }
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
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a == b,
        (Types::Int(a), Types::Long(b)) => a as i64 == b,
        (Types::Int(a), Types::Float(b)) => a as f32 == b,
        (Types::Int(a), Types::Double(b)) => a as f64 == b,
        (Types::Int(a), Types::Byte(b)) => a == b as i32,

        (Types::Long(a), Types::Int(b)) => a == b as i64,
        (Types::Long(a), Types::Long(b)) => a == b,
        (Types::Long(a), Types::Float(b)) => a as f32 == b,
        (Types::Long(a), Types::Double(b)) => a as f64 == b,
        (Types::Long(a), Types::Byte(b)) => a == b as i64,

        (Types::Float(a), Types::Int(b)) => a == b as f32,
        (Types::Float(a), Types::Long(b)) => a == b as f32,
        (Types::Float(a), Types::Float(b)) => a == b,
        (Types::Float(a), Types::Double(b)) => a as f64 == b,
        (Types::Float(a), Types::Byte(b)) => a == b as f32,

        (Types::Double(a), Types::Int(b)) => a == b as f64,
        (Types::Double(a), Types::Long(b)) => a == b as f64,
        (Types::Double(a), Types::Float(b)) => a == b as f64,
        (Types::Double(a), Types::Double(b)) => a == b,
        (Types::Double(a), Types::Byte(b)) => a == b as f64,

        (Types::Byte(a), Types::Int(b)) => a as i32 == b,
        (Types::Byte(a), Types::Long(b)) => a as i64 == b,
        (Types::Byte(a), Types::Float(b)) => a as f32 == b,
        (Types::Byte(a), Types::Double(b)) => a as f64 == b,
        (Types::Byte(a), Types::Byte(b)) => a == b,

        (Types::Str(a), Types::Str(b)) => a == b,
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

fn not_equal(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a != b,
        (Types::Int(a), Types::Long(b)) => a as i64 != b,
        (Types::Int(a), Types::Float(b)) => a as f32 != b,
        (Types::Int(a), Types::Double(b)) => a as f64 != b,
        (Types::Int(a), Types::Byte(b)) => a != b as i32,

        (Types::Long(a), Types::Int(b)) => a != b as i64,
        (Types::Long(a), Types::Long(b)) => a != b,
        (Types::Long(a), Types::Float(b)) => a as f32 != b,
        (Types::Long(a), Types::Double(b)) => a as f64 != b,
        (Types::Long(a), Types::Byte(b)) => a != b as i64,

        (Types::Float(a), Types::Int(b)) => a != b as f32,
        (Types::Float(a), Types::Long(b)) => a != b as f32,
        (Types::Float(a), Types::Float(b)) => a != b,
        (Types::Float(a), Types::Double(b)) => a as f64 != b,
        (Types::Float(a), Types::Byte(b)) => a != b as f32,

        (Types::Double(a), Types::Int(b)) => a != b as f64,
        (Types::Double(a), Types::Long(b)) => a != b as f64,
        (Types::Double(a), Types::Float(b)) => a != b as f64,
        (Types::Double(a), Types::Double(b)) => a != b,
        (Types::Double(a), Types::Byte(b)) => a != b as f64,

        (Types::Byte(a), Types::Int(b)) => a as i32 != b,
        (Types::Byte(a), Types::Long(b)) => a as i64 != b,
        (Types::Byte(a), Types::Float(b)) => a as f32 != b,
        (Types::Byte(a), Types::Double(b)) => a as f64 != b,
        (Types::Byte(a), Types::Byte(b)) => a != b,

        (Types::Str(a), Types::Str(b)) => a != b,
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

fn and(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a == b,

        (_, _) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

fn or(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => (a == -1) || (b == -1),

        (_, _) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

fn grater_than(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a > b,
        (Types::Int(a), Types::Long(b)) => a as i64 > b,
        (Types::Int(a), Types::Float(b)) => a as f32 > b,
        (Types::Int(a), Types::Double(b)) => a as f64 > b,
        (Types::Int(a), Types::Byte(b)) => a > b as i32,

        (Types::Long(a), Types::Int(b)) => a > b as i64,
        (Types::Long(a), Types::Long(b)) => a > b,
        (Types::Long(a), Types::Float(b)) => a as f32 > b,
        (Types::Long(a), Types::Double(b)) => a as f64 > b,
        (Types::Long(a), Types::Byte(b)) => a > b as i64,

        (Types::Float(a), Types::Int(b)) => a > b as f32,
        (Types::Float(a), Types::Long(b)) => a > b as f32,
        (Types::Float(a), Types::Float(b)) => a > b,
        (Types::Float(a), Types::Double(b)) => a as f64 > b,
        (Types::Float(a), Types::Byte(b)) => a > b as f32,

        (Types::Double(a), Types::Int(b)) => a > b as f64,
        (Types::Double(a), Types::Long(b)) => a > b as f64,
        (Types::Double(a), Types::Float(b)) => a > b as f64,
        (Types::Double(a), Types::Double(b)) => a > b,
        (Types::Double(a), Types::Byte(b)) => a > b as f64,

        (Types::Byte(a), Types::Int(b)) => a as i32 > b,
        (Types::Byte(a), Types::Long(b)) => a as i64 > b,
        (Types::Byte(a), Types::Float(b)) => a as f32 > b,
        (Types::Byte(a), Types::Double(b)) => a as f64 > b,
        (Types::Byte(a), Types::Byte(b)) => a > b,

        (Types::Str(a), Types::Str(b)) => a > b,
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

fn less_than(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (b.unwrap(), a.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a > b,
        (Types::Int(a), Types::Long(b)) => a as i64 > b,
        (Types::Int(a), Types::Float(b)) => a as f32 > b,
        (Types::Int(a), Types::Double(b)) => a as f64 > b,
        (Types::Int(a), Types::Byte(b)) => a > b as i32,

        (Types::Long(a), Types::Int(b)) => a > b as i64,
        (Types::Long(a), Types::Long(b)) => a > b,
        (Types::Long(a), Types::Float(b)) => a as f32 > b,
        (Types::Long(a), Types::Double(b)) => a as f64 > b,
        (Types::Long(a), Types::Byte(b)) => a > b as i64,

        (Types::Float(a), Types::Int(b)) => a > b as f32,
        (Types::Float(a), Types::Long(b)) => a > b as f32,
        (Types::Float(a), Types::Float(b)) => a > b,
        (Types::Float(a), Types::Double(b)) => a as f64 > b,
        (Types::Float(a), Types::Byte(b)) => a > b as f32,

        (Types::Double(a), Types::Int(b)) => a > b as f64,
        (Types::Double(a), Types::Long(b)) => a > b as f64,
        (Types::Double(a), Types::Float(b)) => a > b as f64,
        (Types::Double(a), Types::Double(b)) => a > b,
        (Types::Double(a), Types::Byte(b)) => a > b as f64,

        (Types::Byte(a), Types::Int(b)) => a as i32 > b,
        (Types::Byte(a), Types::Long(b)) => a as i64 > b,
        (Types::Byte(a), Types::Float(b)) => a as f32 > b,
        (Types::Byte(a), Types::Double(b)) => a as f64 > b,
        (Types::Byte(a), Types::Byte(b)) => a > b,

        (Types::Str(a), Types::Str(b)) => a > b,
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}
