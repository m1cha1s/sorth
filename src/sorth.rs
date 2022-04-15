pub struct Sorth {
    pub running: bool,
    compile_mode: bool,
    int_stack: Vec<i32>,
    new_compiled_word: String,
    compiled_words: Vec<String>,
}

impl Sorth {
    pub fn new() -> Sorth {
        Sorth {
            running: true,
            compile_mode: false,
            int_stack: Vec::new(),
            new_compiled_word: String::new(),
            compiled_words: Vec::new(),
        }
    }

    pub fn eval(&mut self, line: String, compiled: bool) -> Result<String, String> {
        let mut out_buffer = String::new();

        let split = line.split_whitespace();
        for word in split {
            if self.compile_mode {
                if word == ";" {
                    self.end_compile();
                    continue;
                }
                self.compile(word);
            } else {
                if self.number(word) {
                    continue;
                }
                match word {
                    "+" => out_buffer.push_str(self.add().as_str()),
                    "-" => out_buffer.push_str(self.subtract().as_str()),
                    "*" => out_buffer.push_str(self.multiply().as_str()),
                    "/" => out_buffer.push_str(self.divide().as_str()),

                    "bye" => {
                        if compiled {
                            return Ok(out_buffer);
                        }
                        self.running = false
                    }

                    "dup" => out_buffer.push_str(self.dup().as_str()),
                    "." => out_buffer.push_str(self.pop().as_str()),

                    "peek" => out_buffer.push_str(self.dot().as_str()),

                    ":" => out_buffer.push_str(self.start_compile().as_str()),

                    _ => {
                        if self
                            .compiled_words
                            .iter()
                            .any(|compiled| compiled.starts_with(word))
                        {
                            match self.run_compiled(word) {
                                Ok(ok) => out_buffer.push_str(ok.as_str()),
                                Err(err) => return Err(err),
                            }
                        } else {
                            return Err("Word nor found!!!".to_string());
                        }
                    }
                }
            }
            if !out_buffer.ends_with(" ") {
                out_buffer.push_str(" ");
            }
        }

        if !compiled {
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

    fn add(&mut self) -> String {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x + y;
        self.int_stack.push(result);
        "".to_string()
    }

    fn subtract(&mut self) -> String {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x - y;
        self.int_stack.push(result);
        "".to_string()
    }

    fn multiply(&mut self) -> String {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x * y;
        self.int_stack.push(result);
        "".to_string()
    }

    fn divide(&mut self) -> String {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(1);
        let result = x / y;
        self.int_stack.push(result);
        "".to_string()
    }

    fn dup(&mut self) -> String {
        let head = self.int_stack.pop().unwrap_or(0);
        self.int_stack.push(head);
        self.int_stack.push(head);
        "".to_string()
    }

    fn pop(&mut self) -> String {
        let head = self.int_stack.pop().unwrap_or(0);
        head.to_string()
    }

    fn dot(&mut self) -> String {
        self.int_stack.last().unwrap().to_string()
    }

    fn start_compile(&mut self) -> String {
        self.compile_mode = true;
        "".to_string()
    }

    fn end_compile(&mut self) -> String {
        self.new_compiled_word = self.new_compiled_word.trim().to_string();
        let new_word_name = self
            .new_compiled_word
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();

        let possibly_existing_word_position = self
            .compiled_words
            .iter()
            .position(|compiled| compiled.starts_with(new_word_name.as_str()));
        if possibly_existing_word_position.is_some() {
            self.compiled_words[possibly_existing_word_position.unwrap()] =
                self.new_compiled_word.clone();
        }

        self.compiled_words.push(self.new_compiled_word.clone());
        self.new_compiled_word.clear();
        self.compile_mode = false;
        "".to_string()
    }

    fn compile(&mut self, word: &str) -> String {
        self.new_compiled_word += " ";
        self.new_compiled_word += word;
        "".to_string()
    }

    fn run_compiled(&mut self, word: &str) -> Result<String, String> {
        let compiled_word = self
            .compiled_words
            .iter()
            .find(|compiled| compiled.starts_with(word))
            .unwrap();

        let mut word_copy = compiled_word.clone();

        for _ in 0..word.len() {
            word_copy.remove(0);
        }

        return self.eval(word_copy, true);
    }
}
