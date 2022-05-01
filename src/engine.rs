use crate::prelude::{Types, Word, WordList, UNKNOWN_WORD_ERROR};

pub struct Engine {
    pub running: bool,
    pub compiled_exec: bool,

    pub mode: EngineMode,

    pub main_stack: Vec<Types>,

    pub variable_stack: Vec<(String, Vec<Types>)>,
    pub conditional_stack: Vec<i8>,
    pub loop_stack: Vec<(i32, i32)>,

    pub curr_line_vec: Vec<Vec<String>>,
    pub curr_word_idx: Vec<i32>,

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
            main_stack: Vec::new(),
            new_compiled_word: String::new(),
            compiled_words: Vec::new(),
            normal_words: Vec::new(),
            compile_words: Vec::new(),
            comment_words: Vec::new(),
            see_words: Vec::new(),
            compiled_exec: false,
            conditional_stack: Vec::new(),
            loop_stack: Vec::new(),
            curr_line_vec: Vec::new(),
            curr_word_idx: Vec::new(),
            variable_stack: Vec::new(),
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

        self.curr_word_idx.push(-1);

        self.curr_line_vec
            .push(line.split_whitespace().map(|w| w.to_string()).collect());

        while *self.curr_word_idx.last_mut().unwrap() + 1
            < self.curr_line_vec.last().unwrap().len() as i32
        {
            if !out_buffer.ends_with(" ") {
                out_buffer.push_str(" ");
            }

            *self.curr_word_idx.last_mut().unwrap() += 1;

            match self.mode {
                EngineMode::NORMAL => {
                    let word_def = self.normal_words.iter().find(|&w| w.0(self));
                    if word_def.is_some() {
                        let res = word_def.unwrap().1(self);
                        match res {
                            Ok(ok) => {
                                out_buffer += ok.as_str().trim();
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
                                out_buffer += ok.as_str().trim();
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
                                out_buffer += ok.as_str().trim();
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
                                out_buffer += ok.as_str().trim();
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
            }
            let err_val = UNKNOWN_WORD_ERROR.to_string()
                + self.curr_line_vec.last().unwrap()
                    [*self.curr_word_idx.last_mut().unwrap() as usize]
                    .as_str();
            self.curr_line_vec.pop();
            self.curr_word_idx.pop();
            return Err(err_val);
        }

        if !self.compiled_exec {
            out_buffer.push_str(" Ok.\n");
        }

        self.curr_line_vec.pop();
        self.curr_word_idx.pop();

        Ok(out_buffer)
    }

    pub fn get_curr_word(&self) -> String {
        if *self.curr_word_idx.last().unwrap() < 0
            || *self.curr_word_idx.last().unwrap() > self.curr_line_vec.last().unwrap().len() as i32
        {
            return "".to_string();
        }

        self.curr_line_vec.last().unwrap()[*self.curr_word_idx.last().unwrap() as usize].clone()
    }
}
