use crate::{
    errors::{INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR},
    prelude::{Double, Engine, Float, Int, Long, Types},
};

pub fn equal(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a == b,
        (Types::Int(a), Types::Long(b)) => a as Long == b,
        (Types::Int(a), Types::Float(b)) => a as Float == b,
        (Types::Int(a), Types::Double(b)) => a as Double == b,
        (Types::Int(a), Types::Byte(b)) => a == b as Int,

        (Types::Long(a), Types::Int(b)) => a == b as Long,
        (Types::Long(a), Types::Long(b)) => a == b,
        (Types::Long(a), Types::Float(b)) => a as Float == b,
        (Types::Long(a), Types::Double(b)) => a as Double == b,
        (Types::Long(a), Types::Byte(b)) => a == b as Long,

        (Types::Float(a), Types::Int(b)) => a == b as Float,
        (Types::Float(a), Types::Long(b)) => a == b as Float,
        (Types::Float(a), Types::Float(b)) => a == b,
        (Types::Float(a), Types::Double(b)) => a as Double == b,
        (Types::Float(a), Types::Byte(b)) => a == b as Float,

        (Types::Double(a), Types::Int(b)) => a == b as Double,
        (Types::Double(a), Types::Long(b)) => a == b as Double,
        (Types::Double(a), Types::Float(b)) => a == b as Double,
        (Types::Double(a), Types::Double(b)) => a == b,
        (Types::Double(a), Types::Byte(b)) => a == b as Double,

        (Types::Byte(a), Types::Int(b)) => a as Int == b,
        (Types::Byte(a), Types::Long(b)) => a as Long == b,
        (Types::Byte(a), Types::Float(b)) => a as Float == b,
        (Types::Byte(a), Types::Double(b)) => a as Double == b,
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

pub fn not_equal(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a != b,
        (Types::Int(a), Types::Long(b)) => a as Long != b,
        (Types::Int(a), Types::Float(b)) => a as Float != b,
        (Types::Int(a), Types::Double(b)) => a as Double != b,
        (Types::Int(a), Types::Byte(b)) => a != b as Int,

        (Types::Long(a), Types::Int(b)) => a != b as Long,
        (Types::Long(a), Types::Long(b)) => a != b,
        (Types::Long(a), Types::Float(b)) => a as Float != b,
        (Types::Long(a), Types::Double(b)) => a as Double != b,
        (Types::Long(a), Types::Byte(b)) => a != b as Long,

        (Types::Float(a), Types::Int(b)) => a != b as Float,
        (Types::Float(a), Types::Long(b)) => a != b as Float,
        (Types::Float(a), Types::Float(b)) => a != b,
        (Types::Float(a), Types::Double(b)) => a as Double != b,
        (Types::Float(a), Types::Byte(b)) => a != b as Float,

        (Types::Double(a), Types::Int(b)) => a != b as Double,
        (Types::Double(a), Types::Long(b)) => a != b as Double,
        (Types::Double(a), Types::Float(b)) => a != b as Double,
        (Types::Double(a), Types::Double(b)) => a != b,
        (Types::Double(a), Types::Byte(b)) => a != b as Double,

        (Types::Byte(a), Types::Int(b)) => a as Int != b,
        (Types::Byte(a), Types::Long(b)) => a as Long != b,
        (Types::Byte(a), Types::Float(b)) => a as Float != b,
        (Types::Byte(a), Types::Double(b)) => a as Double != b,
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

pub fn and(s: &mut Engine) -> Result<String, String> {
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

pub fn or(s: &mut Engine) -> Result<String, String> {
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

pub fn not(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();

    if a.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match a.unwrap() {
        Types::Int(a) => {
            if a == -1 {
                false
            } else {
                true
            }
        }

        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    };

    if result {
        s.main_stack.push(Types::Int(-1));
    } else {
        s.main_stack.push(Types::Int(0));
    }

    Ok("".to_string())
}

pub fn grater_than(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (a.unwrap(), b.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a > b,
        (Types::Int(a), Types::Long(b)) => a as Long > b,
        (Types::Int(a), Types::Float(b)) => a as Float > b,
        (Types::Int(a), Types::Double(b)) => a as Double > b,
        (Types::Int(a), Types::Byte(b)) => a > b as Int,

        (Types::Long(a), Types::Int(b)) => a > b as Long,
        (Types::Long(a), Types::Long(b)) => a > b,
        (Types::Long(a), Types::Float(b)) => a as Float > b,
        (Types::Long(a), Types::Double(b)) => a as Double > b,
        (Types::Long(a), Types::Byte(b)) => a > b as Long,

        (Types::Float(a), Types::Int(b)) => a > b as Float,
        (Types::Float(a), Types::Long(b)) => a > b as Float,
        (Types::Float(a), Types::Float(b)) => a > b,
        (Types::Float(a), Types::Double(b)) => a as Double > b,
        (Types::Float(a), Types::Byte(b)) => a > b as Float,

        (Types::Double(a), Types::Int(b)) => a > b as Double,
        (Types::Double(a), Types::Long(b)) => a > b as Double,
        (Types::Double(a), Types::Float(b)) => a > b as Double,
        (Types::Double(a), Types::Double(b)) => a > b,
        (Types::Double(a), Types::Byte(b)) => a > b as Double,

        (Types::Byte(a), Types::Int(b)) => a as Int > b,
        (Types::Byte(a), Types::Long(b)) => a as Long > b,
        (Types::Byte(a), Types::Float(b)) => a as Float > b,
        (Types::Byte(a), Types::Double(b)) => a as Double > b,
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

pub fn less_than(s: &mut Engine) -> Result<String, String> {
    let a = s.main_stack.pop();
    let b = s.main_stack.pop();

    if a.is_none() || b.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    let result: bool = match (b.unwrap(), a.unwrap()) {
        (Types::Int(a), Types::Int(b)) => a > b,
        (Types::Int(a), Types::Long(b)) => a as Long > b,
        (Types::Int(a), Types::Float(b)) => a as Float > b,
        (Types::Int(a), Types::Double(b)) => a as Double > b,
        (Types::Int(a), Types::Byte(b)) => a > b as Int,

        (Types::Long(a), Types::Int(b)) => a > b as Long,
        (Types::Long(a), Types::Long(b)) => a > b,
        (Types::Long(a), Types::Float(b)) => a as Float > b,
        (Types::Long(a), Types::Double(b)) => a as Double > b,
        (Types::Long(a), Types::Byte(b)) => a > b as Long,

        (Types::Float(a), Types::Int(b)) => a > b as Float,
        (Types::Float(a), Types::Long(b)) => a > b as Float,
        (Types::Float(a), Types::Float(b)) => a > b,
        (Types::Float(a), Types::Double(b)) => a as Double > b,
        (Types::Float(a), Types::Byte(b)) => a > b as Float,

        (Types::Double(a), Types::Int(b)) => a > b as Double,
        (Types::Double(a), Types::Long(b)) => a > b as Double,
        (Types::Double(a), Types::Float(b)) => a > b as Double,
        (Types::Double(a), Types::Double(b)) => a > b,
        (Types::Double(a), Types::Byte(b)) => a > b as Double,

        (Types::Byte(a), Types::Int(b)) => a as Int > b,
        (Types::Byte(a), Types::Long(b)) => a as Long > b,
        (Types::Byte(a), Types::Float(b)) => a as Float > b,
        (Types::Byte(a), Types::Double(b)) => a as Double > b,
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
