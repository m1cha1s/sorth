use crate::{
    errors::STACK_UNDERFLOW_ERROR,
    prelude::{Engine, Types},
};

pub fn dup(s: &mut Engine) -> Result<String, String> {
    let head = s.main_stack.pop();

    if head.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let head_val = head.unwrap();

    s.main_stack.push(head_val.clone());
    s.main_stack.push(head_val);
    Ok("".to_string())
}

pub fn two_dup(s: &mut Engine) -> Result<String, String> {
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

pub fn drop_word(s: &mut Engine) -> Result<String, String> {
    let _ = s.main_stack.pop();
    Ok("".to_string())
}

pub fn swap_word(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    let y = s.main_stack.pop();

    if x.is_none() || y.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    s.main_stack.push(x.unwrap());
    s.main_stack.push(y.unwrap());
    Ok("".to_string())
}

pub fn rot(s: &mut Engine) -> Result<String, String> {
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

pub fn dot(s: &mut Engine) -> Result<String, String> {
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

pub fn peek(s: &mut Engine) -> Result<String, String> {
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
