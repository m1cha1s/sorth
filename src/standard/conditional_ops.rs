use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Engine, Types},
};

pub fn current_cond(s: &Engine) -> i8 {
    match s.conditional_stack.last() {
        Some(&val) => val,
        None => 1,
    }
}

pub fn if_word(s: &mut Engine) -> Result<String, String> {
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

pub fn else_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.conditional_stack.pop();
    match cond {
        Some(cond_val) => {
            s.conditional_stack.push(cond_val - 1);
        }
        None => return Err("Error Conditional stack underflow!".to_string()),
    }
    Ok("".to_string())
}

pub fn then_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.conditional_stack.pop();
    match cond {
        Some(_) => {}
        None => return Err("Error Conditional stack underflow!".to_string()),
    }
    Ok("".to_string())
}

pub fn conditional_skip(_s: &mut Engine) -> Result<String, String> {
    Ok("".to_string())
}
