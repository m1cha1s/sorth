use crate::prelude::{Engine, Types};

pub fn int_number(s: &mut Engine) -> Result<String, String> {
    let number = s.get_curr_word().parse::<i32>().unwrap();
    s.main_stack.push(Types::Int(number));
    Ok("".to_string())
}

pub fn long_number(s: &mut Engine) -> Result<String, String> {
    let mut word = s.get_curr_word();
    word.pop();
    let number = word.parse::<i64>().unwrap();
    s.main_stack.push(Types::Long(number));
    Ok("".to_string())
}

pub fn float_number(s: &mut Engine) -> Result<String, String> {
    let mut word = s.get_curr_word();
    word.pop();
    let number = word.parse::<f32>().unwrap();
    s.main_stack.push(Types::Float(number));
    Ok("".to_string())
}

pub fn double_number(s: &mut Engine) -> Result<String, String> {
    let number = s.get_curr_word().parse::<f64>().unwrap();
    s.main_stack.push(Types::Double(number));
    Ok("".to_string())
}

pub fn byte_number(s: &mut Engine) -> Result<String, String> {
    let mut word = s.get_curr_word();
    word.remove(0);
    word.remove(0);

    let number = u8::from_str_radix(word.as_str(), 16).unwrap();
    s.main_stack.push(Types::Byte(number));
    Ok("".to_string())
}
