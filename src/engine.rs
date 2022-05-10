use crate::prelude::{Types, Word, WordList, UNKNOWN_WORD_ERROR};

pub struct Engine {
    pub running: bool,
    pub waiting_for_input: bool,
    pub silent: bool,
    pub compiled_exec: Vec<bool>,

    pub mode: EngineMode,

    pub main_stack: Vec<Types>,

    pub variable_stack: Vec<(String, Vec<Types>)>,
    pub conditional_stack: Vec<i8>,
    pub loop_stack: Vec<(i32, i32)>,

    pub curr_line_vec: Vec<Vec<String>>,
    pub curr_word_idx: Vec<i32>,

    pub string_buffer: String,

    pub new_compiled_word: String,
    pub compiled_words: Vec<String>,

    pub words: Vec<Word>,
}

#[derive(PartialEq, Debug)]
pub enum EngineMode {
    NORMAL,
    COMPILE,
    SEE,
    COMMENT,
    STRING,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            running: true,
            mode: EngineMode::NORMAL,
            main_stack: Vec::new(),
            new_compiled_word: String::new(),
            compiled_words: Vec::new(),
            words: Vec::new(),
            compiled_exec: vec![false],
            conditional_stack: Vec::new(),
            loop_stack: Vec::new(),
            curr_line_vec: Vec::new(),
            curr_word_idx: Vec::new(),
            variable_stack: Vec::new(),
            string_buffer: String::new(),
            waiting_for_input: false,
            silent: false,
        }
    }

    pub fn import_word_list(&mut self, word_list: impl WordList) {
        self.words.append(&mut word_list.get_words());
    }

    pub fn eval(&mut self, line: String) -> Result<String, String> {
        if self.waiting_for_input {
            self.main_stack.push(Types::Str(line));
            self.waiting_for_input = false;
            if !self.silent {
                return Ok("\nOk.\n".to_string());
            } else {
                return Ok("\n".to_string());
            }
        }

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

            let word_def = self.words.iter().find(|&w| w.0(self));
            if word_def.is_some() {
                let res = word_def.unwrap().1(self);
                match res {
                    Ok(ok) => {
                        if !ok.starts_with("\n") || !ok.ends_with("\n") {
                            out_buffer += ok.trim();
                        } else {
                            out_buffer += ok.as_str();
                        }
                        continue;
                    }
                    Err(err) => return Err(err + "\n"),
                }
            }

            let err_val = UNKNOWN_WORD_ERROR.to_string()
                + self.curr_line_vec.last().unwrap()
                    [*self.curr_word_idx.last_mut().unwrap() as usize]
                    .as_str();
            self.curr_line_vec.pop();
            self.curr_word_idx.pop();
            return Err(err_val + "\n");
        }

        if !self.compiled_exec.last().unwrap() && self.mode == EngineMode::NORMAL {
            if !self.silent {
                out_buffer.push_str("\nOk.\n");
            }
            // else {
            //     out_buffer.push_str("");
            // }
        }

        self.curr_line_vec.pop();
        self.curr_word_idx.pop();

        out_buffer = out_buffer.trim_matches(|c| c == ' ').to_string();

        Ok(out_buffer)
    }

    pub fn get_compiled_exec(&self) -> bool {
        self.compiled_exec.last().unwrap().clone()
    }

    pub fn get_curr_word(&self) -> String {
        if *self.curr_word_idx.last().unwrap() < 0
            || *self.curr_word_idx.last().unwrap() > self.curr_line_vec.last().unwrap().len() as i32
        {
            return "".to_string();
        }

        self.curr_line_vec.last().unwrap()[*self.curr_word_idx.last().unwrap() as usize].clone()
    }

    pub fn mode_normal(&self) -> bool {
        self.mode == EngineMode::NORMAL
    }
    pub fn mode_compile(&self) -> bool {
        self.mode == EngineMode::COMPILE
    }
    pub fn mode_comment(&self) -> bool {
        self.mode == EngineMode::COMMENT
    }
    pub fn mode_see(&self) -> bool {
        self.mode == EngineMode::SEE
    }
    pub fn mode_string(&self) -> bool {
        self.mode == EngineMode::STRING
    }
}
