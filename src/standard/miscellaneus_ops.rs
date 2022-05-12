use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Engine, Types},
};

pub fn bye(s: &mut Engine) -> Result<String, String> {
    if !s.get_compiled_exec() {
        s.running = false;
    }
    Ok("".to_string())
}

pub fn silent(s: &mut Engine) -> Result<String, String> {
    let value = s.main_stack.pop();

    if let Some(value) = value {
        if let Types::Int(value) = value {
            if value == -1 {
                s.silent = true;
            } else if value == 0 {
                s.silent = false
            }

            return Ok("".to_string());
        }
        return Err(INVALID_TYPE_ERROR.to_string());
    }
    Err(STACK_UNDERFLOW_ERROR.to_string())
}

pub fn input(s: &mut Engine) -> Result<String, String> {
    let question = s.main_stack.pop();

    if let Some(question) = question {
        if let Types::Str(question) = question {
            s.waiting_for_input = true;
            return Ok(question + "\n");
        }
        return Err(INVALID_TYPE_ERROR.to_string());
    }
    Err(STACK_UNDERFLOW_ERROR.to_string())
}

pub fn nl(_s: &mut Engine) -> Result<String, String> {
    Ok("\n".to_string())
}

pub fn emit(s: &mut Engine) -> Result<String, String> {
    let character = s.main_stack.pop();

    if character.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    if let Types::Byte(character) = character.unwrap() {
        return Ok(String::from(character as char));
    } else {
        return Err(INVALID_TYPE_ERROR.to_string());
    }
}
