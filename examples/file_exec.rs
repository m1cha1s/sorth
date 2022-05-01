use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use sorth::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_name = "./examples/example_progs/sorth_test_prog.txt";
    if args.len() > 1 {
        file_name = match args[1].as_str() {
            "fibonacci" => "./examples/example_progs/fibonacci.txt",
            name => name,
        }
    }

    let mut engine = Engine::new();

    let std_words = Standard::new();

    engine.import_word_list(std_words);

    if let Ok(lines) = read_lines(file_name) {
        for line_opt in lines {
            if let Ok(line) = line_opt {
                match engine.eval(line) {
                    Ok(ok) => print!("{}", ok.trim_start()),
                    Err(err) => print!("Error: {}", err.trim_start()),
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
