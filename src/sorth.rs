pub struct Sorth {
    pub running: bool,

    compile_mode: bool,
    comment_mode: bool,
    see_mode: bool,

    int_stack: Vec<i32>,

    new_compiled_word: String,
    compiled_words: Vec<String>,
}

impl Sorth {
    pub fn new() -> Sorth {
        Sorth {
            running: true,
            compile_mode: false,
            see_mode: false,
            comment_mode: false,
            int_stack: Vec::new(),
            new_compiled_word: String::new(),
            compiled_words: Vec::new(),
        }
    }

    pub fn eval(&mut self, line: String, compiled: bool) -> Result<String, String> {
        let mut out_buffer = String::new();

        let split = line.split_whitespace();
        for word in split {
            if self.see_mode {
                let compiled_word_index = self
                    .compiled_words
                    .iter()
                    .position(|compiled| compiled.starts_with(word));

                if compiled_word_index.is_none() {
                    return Err("No such word defined!!!".to_string());
                }

                out_buffer.push_str(
                    self.compiled_words[compiled_word_index.unwrap()]
                        .clone()
                        .as_str(),
                );

                self.see_mode = false;
            } else if self.comment_mode {
                if word == ")" {
                    self.comment_mode = false;
                }
            } else if self.compile_mode {
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
                    // Basic math ops
                    "+" => self.add(),
                    "-" => self.subtract(),
                    "*" => self.multiply(),
                    "/" => self.divide(),

                    // Logic ops
                    "=" => self.equal(),
                    "<>" => self.not_equal(),
                    "and" => self.and(),
                    "or" => self.or(),
                    ">" => self.grate_than(),
                    "<" => self.less_than(),

                    // Exit
                    "bye" => {
                        if compiled {
                            return Ok(out_buffer);
                        }
                        self.running = false
                    }

                    // Stack ops
                    "dup" => self.dup(),
                    "2dup" => self.two_dup(),
                    "pop" => self.drop(),
                    "swap" => self.swap(),
                    "rot" => self.rot(),

                    // Stack display ops
                    "." => out_buffer += self.dot().as_str(),
                    "peek" => out_buffer += self.peek().as_str(),

                    // Mode starting sumbols
                    ":" => self.start_compile(),
                    "(" => self.comment_mode = true,
                    "see" => self.see_mode = true,

                    // Custom words
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

    fn add(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x + y;
        self.int_stack.push(result);
    }

    fn subtract(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x - y;
        self.int_stack.push(result);
    }

    fn multiply(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let result = x * y;
        self.int_stack.push(result);
    }

    fn divide(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(1);
        let result = x / y;
        self.int_stack.push(result);
    }

    fn dup(&mut self) {
        let head = self.int_stack.pop().unwrap_or(0);
        self.int_stack.push(head);
        self.int_stack.push(head);
    }

    fn two_dup(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        self.int_stack.push(y);
        self.int_stack.push(x);
        self.int_stack.push(y);
        self.int_stack.push(x);
    }

    fn drop(&mut self) {
        let _ = self.int_stack.pop().unwrap_or(0);
    }

    fn swap(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        self.int_stack.push(x);
        self.int_stack.push(y);
    }

    fn rot(&mut self) {
        let x = self.int_stack.pop().unwrap_or(0);
        let y = self.int_stack.pop().unwrap_or(0);
        let z = self.int_stack.pop().unwrap_or(0);
        self.int_stack.push(y);
        self.int_stack.push(x);
        self.int_stack.push(z);
    }

    fn dot(&mut self) -> String {
        let head = self.int_stack.pop().unwrap_or(0);
        head.to_string()
    }

    fn peek(&mut self) -> String {
        self.int_stack.last().unwrap_or(&0).to_string()
    }

    fn start_compile(&mut self) {
        self.compile_mode = true;
    }

    fn end_compile(&mut self) {
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
    }

    fn compile(&mut self, word: &str) {
        self.new_compiled_word += " ";
        self.new_compiled_word += word;
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

    fn equal(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if a == b {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }

    fn not_equal(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if a != b {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }

    fn and(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if a == b {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }

    fn or(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if (a == -1) || (b == -1) {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }

    fn grate_than(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if a > b {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }

    fn less_than(&mut self) {
        let a = self.int_stack.pop().unwrap_or(0);
        let b = self.int_stack.pop().unwrap_or(0);

        if a < b {
            self.int_stack.push(-1);
        } else {
            self.int_stack.push(0);
        }
    }
}
