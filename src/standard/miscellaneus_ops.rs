use crate::prelude::Engine;

pub fn bye(s: &mut Engine) -> Result<String, String> {
    if !s.compiled_exec {
        s.running = false;
    } else {
        s.compiled_exec = false;
    }
    Ok("".to_string())
}
