use crate::{
    errors::INVALID_TYPE_ERROR,
    prelude::{Engine, Types},
};

pub fn while_word(_s: &mut Engine) -> Result<String, String> {
    Ok("".to_string())
}

pub fn do_word(s: &mut Engine) -> Result<String, String> {
    let cond = s.main_stack.pop();
    if cond.is_none() {
        return Err("Error Int stack underflow!".to_string());
    }

    match cond.unwrap() {
        Types::Int(x) => {
            if x == 0 {
                let mut controll = 1;
                while controll != 0 {
                    *s.curr_word_idx.last_mut().unwrap() += 1;

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

pub fn again_word(s: &mut Engine) -> Result<String, String> {
    let mut controll = 1;
    while controll > 0 {
        *s.curr_word_idx.last_mut().unwrap() -= 1;

        match s.get_curr_word().as_str() {
            "while" => controll -= 1,
            "again" => controll += 1,
            _ => {}
        }
    }

    Ok("".to_string())
}

pub fn for_word(s: &mut Engine) -> Result<String, String> {
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

pub fn next_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.pop();
    if curr_loop.is_none() {
        return Err("Error Loop control stack underflow!".to_string());
    }

    let mut curr_loop_contents = curr_loop.unwrap();
    curr_loop_contents.1 += 1;

    if curr_loop_contents.1 < curr_loop_contents.0 {
        let mut cntr = 1;
        while cntr != 0 {
            *s.curr_word_idx.last_mut().unwrap() -= 1;

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

pub fn bynext_word(s: &mut Engine) -> Result<String, String> {
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
            *s.curr_word_idx.last_mut().unwrap() -= 1;

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

pub fn i_word(s: &mut Engine) -> Result<String, String> {
    let curr_loop = s.loop_stack.last();

    match curr_loop {
        Some(_) => {}
        None => return Err("Error Loop control stack underflow!".to_string()),
    }

    s.main_stack.push(Types::Int(curr_loop.unwrap().1));

    Ok("".to_string())
}
