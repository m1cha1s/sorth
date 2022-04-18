mod sorth;

use sorth::prelude::*;

fn main() {
    let mut s = Sorth::new();

    while s.running {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match s.eval(line, false) {
            Ok(ok) => println!("{}", ok.trim()),
            Err(err) => println!("Error: {}", err.trim()),
        }
    }
}
