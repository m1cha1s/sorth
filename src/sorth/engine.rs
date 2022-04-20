use super::prelude::{Word, WordList};

pub struct Engine {
    pub running: bool,
    pub compiled_exec: bool,

    pub mode: EngineMode,

    pub int_stack: Vec<i32>,

    pub conditional_stack: Vec<bool>,

    pub curr_word: String,

    pub new_compiled_word: String,
    pub compiled_words: Vec<String>,

    pub normal_words: Vec<Word>,
    pub compile_words: Vec<Word>,
    pub comment_words: Vec<Word>,
    pub see_words: Vec<Word>,
}

pub enum EngineMode {
    NORMAL,
    COMPILE,
    SEE,
    COMMENT,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            running: true,
            mode: EngineMode::NORMAL,
            int_stack: Vec::new(),
            new_compiled_word: String::new(),
            compiled_words: Vec::new(),
            curr_word: "".to_string(),
            normal_words: Vec::new(),
            compile_words: Vec::new(),
            comment_words: Vec::new(),
            see_words: Vec::new(),
            compiled_exec: false,
            conditional_stack: Vec::new(),
        }
    }

    pub fn import_word_list(&mut self, word_list: impl WordList) {
        self.normal_words.append(&mut word_list.get_normal());
        self.compile_words.append(&mut word_list.get_compile());
        self.comment_words.append(&mut word_list.get_comment());
        self.see_words.append(&mut word_list.get_see());
    }

    pub fn eval(&mut self, line: String) -> Result<String, String> {
        let mut out_buffer = String::new();

        let split = line.split_whitespace();

        for word in split {
            if !out_buffer.ends_with(" ") {
                out_buffer.push_str(" ");
            }

            self.curr_word = word.to_string();
            match self.mode {
                EngineMode::NORMAL => {
                    let word_def = self.normal_words.iter().find(|&w| w.0(self));
                    if word_def.is_some() {
                        let res = word_def.unwrap().1(self);
                        match res {
                            Ok(ok) => {
                                out_buffer += ok.as_str();
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
                EngineMode::COMPILE => {
                    let word_def = self.compile_words.iter().find(|&w| w.0(self));
                    if word_def.is_some() {
                        let res = word_def.unwrap().1(self);
                        match res {
                            Ok(ok) => {
                                out_buffer += ok.as_str();
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
                EngineMode::SEE => {
                    let word_def = self.see_words.iter().find(|&w| w.0(self));
                    if word_def.is_some() {
                        let res = word_def.unwrap().1(self);
                        match res {
                            Ok(ok) => {
                                out_buffer += ok.as_str();
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
                EngineMode::COMMENT => {
                    let word_def = self.comment_words.iter().find(|&w| w.0(self));
                    if word_def.is_some() {
                        let res = word_def.unwrap().1(self);
                        match res {
                            Ok(ok) => {
                                out_buffer += ok.as_str();
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
            }

            return Err("Error unknown word!".to_string());
        }

        if !self.compiled_exec {
            out_buffer.push_str(" Ok.\n");
        }

        Ok(out_buffer)
    }

    fn number(&mut self, word: &str) -> bool {
        let numeric = word.parse::<i32>();
        match numeric {
            Ok(val) => {
                self.int_stack.push(val);
                return true;
            }
            Err(_) => return false,
        }
    }
}
