use crate::{
    errors::{STACK_UNDERFLOW_ERROR, TYPE_CONVERSION_FAILURE_ERROR},
    prelude::{Byte, Double, Engine, Float, Int, Long, Types},
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
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Int(val as Int)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Int(val as Int)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Int(val as Int)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Int(val as Int)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<Int>();
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
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Long(val as Long)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Long(val)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Long(val as Long)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Long(val as Long)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Long(val as Long)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<Long>();
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
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Float(val as Float)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Float(val as Float)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Float(val as Float)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Float(val as Float)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Float(val as Float)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<Float>();
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
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Double(val as Double)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Double(val as Double)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Double(val as Double)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Double(val as Double)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Double(val as Double)),
        crate::prelude::Types::Str(val) => {
            let parsed = val.parse::<Double>();
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
        crate::prelude::Types::Int(val) => s.main_stack.push(Types::Byte(val as Byte)),
        crate::prelude::Types::Long(val) => s.main_stack.push(Types::Byte(val as Byte)),
        crate::prelude::Types::Float(val) => s.main_stack.push(Types::Byte(val as Byte)),
        crate::prelude::Types::Double(val) => s.main_stack.push(Types::Byte(val as Byte)),
        crate::prelude::Types::Byte(val) => s.main_stack.push(Types::Byte(val as Byte)),
        crate::prelude::Types::Str(val) => {
            let parsed = Byte::from_str_radix(val.as_str(), 16);
            if parsed.is_ok() {
                s.main_stack.push(Types::Byte(parsed.unwrap()));
            } else {
                return Err(TYPE_CONVERSION_FAILURE_ERROR.to_string());
            }
        }
    }

    Ok("".to_string())
}
