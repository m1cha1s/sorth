mod sorth;

use sorth::prelude::*;

fn main() {
    let mut engine = Engine::new();

    let mut std_words = Standard::new();

    engine.import_word_list(std_words);

    while engine.running {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match engine.eval(line) {
            Ok(ok) => println!("{}", ok.trim()),
            Err(err) => println!("Error: {}", err.trim()),
        }
    }
}
