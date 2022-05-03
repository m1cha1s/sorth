use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Engine, EngineMode, Types},
};

pub fn string_mode_toggle(s: &mut Engine) -> Result<String, String> {
    if s.mode_string() {
        s.mode = EngineMode::NORMAL;
        s.main_stack
            .push(Types::Str(s.string_buffer.trim_end().to_string().clone()));
        s.string_buffer.drain(..);
    } else {
        s.mode = EngineMode::STRING;
    }
    Ok("".to_string())
}

pub fn string_creation(s: &mut Engine) -> Result<String, String> {
    s.string_buffer.push_str(s.get_curr_word().as_str());
    s.string_buffer.push_str(" ");
    Ok("".to_string())
}

// Join two strings without a space in between
pub fn string_concat(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (a.unwrap(), b.unwrap()) {
        (Types::Str(b), Types::Str(a)) => s
            .main_stack
            .push(Types::Str(a.trim_end().to_string() + b.trim_start())),
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

// Split string by a delimiter
pub fn string_split(s: &mut Engine) -> Result<String, String> {
    let delimiter = s.main_stack.pop();
    let a = s.main_stack.pop();

    if delimiter.is_none() || a.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (delimiter.unwrap(), a.unwrap()) {
        (Types::Str(delimiter), Types::Str(a)) => {
            for splitted in a.split(delimiter.as_str()) {
                println!("{}, {}", delimiter, splitted);
                s.main_stack.push(Types::Str(splitted.to_string()));
            }
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

// Split string by a whitespace
pub fn string_split_whitespace(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();

    if a.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match a.unwrap() {
        Types::Str(a) => {
            for splitted in a.split_whitespace() {
                s.main_stack.push(Types::Str(splitted.to_string()));
            }
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}
