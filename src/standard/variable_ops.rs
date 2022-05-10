use crate::{
    errors::{
        INVALID_TYPE_ERROR, STACK_UNDERFLOW_ERROR, VARIABLE_INDEX_OUT_OR_RANGE_ERROR,
        VARIABLE_NOT_DEFINED,
    },
    prelude::{Engine, Types},
};

pub fn let_word(s: &mut Engine) -> Result<String, String> {
    *s.curr_word_idx.last_mut().unwrap() += 1;

    let potential_existing = s
        .variable_stack
        .iter()
        .enumerate()
        .find(|&v| (v.1).0 == s.get_curr_word());

    if potential_existing.is_some() {
        let index = potential_existing.unwrap().0;
        s.variable_stack.remove(index);
    }

    s.variable_stack.push((s.get_curr_word(), Vec::new()));

    Ok("".to_string())
}

pub fn get_var_addr_word(s: &mut Engine) -> Result<String, String> {
    let mut word = s.get_curr_word();
    word.remove(0);

    let potential_existing = s
        .variable_stack
        .iter()
        .enumerate()
        .find(|v| (v.1).0 == word);

    if potential_existing.is_some() {
        let index = potential_existing.unwrap().0 as i32;
        s.main_stack.push(Types::Int(index));
    } else {
        return Err(VARIABLE_NOT_DEFINED.to_string());
    }

    Ok("".to_string())
}

pub fn push_word(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();
    let var_index = s.main_stack.pop();

    if val.is_none() || var_index.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (var_index.unwrap(), val.unwrap()) {
        (Types::Int(i), Types::Int(val)) => s.variable_stack[i as usize].1.push(Types::Int(val)),
        (Types::Int(i), Types::Long(val)) => s.variable_stack[i as usize].1.push(Types::Long(val)),
        (Types::Int(i), Types::Float(val)) => {
            s.variable_stack[i as usize].1.push(Types::Float(val))
        }
        (Types::Int(i), Types::Double(val)) => {
            s.variable_stack[i as usize].1.push(Types::Double(val))
        }
        (Types::Int(i), Types::Byte(val)) => s.variable_stack[i as usize].1.push(Types::Byte(val)),
        (Types::Int(i), Types::Str(val)) => s.variable_stack[i as usize].1.push(Types::Str(val)),
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

pub fn pop_word(s: &mut Engine) -> Result<String, String> {
    let var_index = s.main_stack.pop();

    if var_index.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match var_index.unwrap() {
        Types::Int(i) => {
            let poped_val = s.variable_stack[i as usize].1.pop();

            if poped_val.is_none() {
                return Err(STACK_UNDERFLOW_ERROR.to_string());
            }

            s.main_stack.push(poped_val.unwrap());
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

pub fn len_word(s: &mut Engine) -> Result<String, String> {
    let var_index = s.main_stack.pop();

    if var_index.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match var_index.unwrap() {
        Types::Int(i) => {
            let var_len = s.variable_stack[i as usize].1.len();

            s.main_stack.push(Types::Int(var_len as i32));
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}

pub fn get_from_index_word(s: &mut Engine) -> Result<String, String> {
    let index = s.main_stack.pop();
    let var_index = s.main_stack.pop();

    if index.is_none() || var_index.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (var_index.unwrap(), index.unwrap()) {
        (Types::Int(i), Types::Int(index)) => {
            if index as usize >= s.variable_stack[i as usize].1.len() {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }
            s.main_stack
                .push(s.variable_stack[i as usize].1[index as usize].clone());
        }
        (Types::Int(i), Types::Long(index)) => {
            if index as usize >= s.variable_stack[i as usize].1.len() {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }
            s.main_stack
                .push(s.variable_stack[i as usize].1[index as usize].clone());
        }
        (Types::Int(i), Types::Float(index)) => {
            if index as usize >= s.variable_stack[i as usize].1.len() {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }
            s.main_stack
                .push(s.variable_stack[i as usize].1[index as usize].clone());
        }
        (Types::Int(i), Types::Double(index)) => {
            if index as usize >= s.variable_stack[i as usize].1.len() {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }
            s.main_stack
                .push(s.variable_stack[i as usize].1[index as usize].clone());
        }
        (Types::Int(i), Types::Byte(index)) => {
            if index as usize >= s.variable_stack[i as usize].1.len() {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }
            s.main_stack
                .push(s.variable_stack[i as usize].1[index as usize].clone());
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}
pub fn set_in_index_word(s: &mut Engine) -> Result<String, String> {
    let val = s.main_stack.pop();
    let index = s.main_stack.pop();
    let var_index = s.main_stack.pop();

    if index.is_none() || var_index.is_none() {
        return Err(STACK_UNDERFLOW_ERROR.to_string());
    }

    match (var_index.unwrap(), index.unwrap(), val.unwrap()) {
        (Types::Int(var_idx), Types::Int(idx), Types::Int(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Int(a);
        }
        (Types::Int(var_idx), Types::Int(idx), Types::Long(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Long(a);
        }
        (Types::Int(var_idx), Types::Int(idx), Types::Float(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Float(a);
        }
        (Types::Int(var_idx), Types::Int(idx), Types::Double(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Double(a);
        }
        (Types::Int(var_idx), Types::Int(idx), Types::Byte(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Byte(a);
        }
        (Types::Int(var_idx), Types::Int(idx), Types::Str(a)) => {
            if idx + 1 > s.variable_stack[var_idx as usize].1.len() as i32 {
                return Err(VARIABLE_INDEX_OUT_OR_RANGE_ERROR.to_string());
            }

            s.variable_stack[var_idx as usize].1[idx as usize] = Types::Str(a);
        }
        _ => return Err(INVALID_TYPE_ERROR.to_string()),
    }

    Ok("".to_string())
}
