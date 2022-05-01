use crate::prelude::{Engine, EngineMode, Types};

pub fn string_mode_toggle(s: &mut Engine) -> Result<String, String> {
    if s.mode_string() {
        s.mode = EngineMode::NORMAL;
        _ = s.string_buffer.trim_end();
        s.main_stack.push(Types::Str(s.string_buffer.clone()));
        s.string_buffer.drain(..);
    } else {
        s.mode = EngineMode::STRING;
    }
    Ok("".to_string())
}

pub fn string_creation(s: &mut Engine) -> Result<String, String> {
    s.string_buffer.push_str(s.get_curr_word().as_str());
    s.string_buffer.push_str(" ");
    Ok("".to_string())
}
