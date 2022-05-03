use crate::{
    errors::{STACK_UNDERFLOW_ERROR, TYPE_CONVERSION_FAILURE_ERROR},
    prelude::{Engine, Types},
};

pub fn to_string(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Str(val.to_string())),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Str(val.to_string())),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Str(val.to_string())),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Str(val.to_string())),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Str(val.to_string())),
        crate::prelude::Types::Str(val) => s.main_stack.push(Types::Str(val)),
    }

    Ok("".to_string())
}

pub fn to_int(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Int(val)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Int(val as i32)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Int(val as i32)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Int(val as i32)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Int(val as i32)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<i32>();
            if parsed.is_ok() {
                s.main_stack.push(Types::Int(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}

pub fn to_long(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Long(val as i64)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Long(val)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Long(val as i64)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Long(val as i64)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Long(val as i64)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<i64>();
            if parsed.is_ok() {
                s.main_stack.push(Types::Long(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}

pub fn to_float(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Float(val as f32)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Float(val as f32)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Float(val as f32)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Float(val as f32)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Float(val as f32)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<f32>();
            if parsed.is_ok() {
                s.main_stack.push(Types::Float(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}

pub fn to_double(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Double(val as f64)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Double(val as f64)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Double(val as f64)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Double(val as f64)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Double(val as f64)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<f64>();
            if parsed.is_ok() {
                s.main_stack.push(Types::Double(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}

pub fn to_byte(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();

    if val.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match val.unwrap() {
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Byte(val as u8)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Byte(val as u8)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Byte(val as u8)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Byte(val as u8)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Byte(val as u8)),
        crate::prelude::Types::Str(val) => {
            let parsed = u8::from_str_radix(val.as_str(), 16);
            if parsed.is_ok() {
                s.main_stack.push(Types::Byte(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}
