use crate::prelude::{Engine, EngineMode};

pub fn comment_toggle(s: &mut Engine) -> Result<String, String> {
    if s.mode_normal() {
        s.mode = EngineMode::COMMENT;
    } else if s.mode_comment() {
        s.mode = EngineMode::NORMAL;
    }
    Ok("".to_string())
}
