use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Engine, Types},
};

pub fn add(s: &mut Engine) -> Result<String, String> {
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

        (Types::Str(a), Types::Str(b)) => s
            .main_stack
            .push(Types::Str(a.trim().to_string() + " " + b.trim())),
        (Types::Str(_), _) => return Err(INVALID_TYPE_ERROR.to_string()),
        (_, Types::Str(_)) => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

pub fn subtract(s: &mut Engine) -> Result<String, String> {
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

pub fn multiply(s: &mut Engine) -> Result<String, String> {
    let x = s.main_stack.pop();
    let y = s.main_stack.pop();

    if x.is_none() || y.is_none() {
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

pub fn divide(s: &mut Engine) -> Result<String, String> {
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
