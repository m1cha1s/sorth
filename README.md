# Sorth
This is a language interpreter inspired by FORTH. It's main features are that it's easily extensible and that it is no_std friendly only requiering global allocator.

## ToDo:
- [x] Generic math ops
- [x] Generic logic ops
- [x] Generic stack ops
- [x] Runtime word definition
- [x] Conditional logic
- [x] Loops
- [x] String support
- [x] Int, Long and Byte support
- [x] Float and Double support
- [x] Variables support
- [ ] Extended math ops
- [ ] File access wordset
- [ ] Sorth user input interface
- [x] Rework sorth into library crate
- [ ] Share on crates.io

## Getting started
Use this to get started.
```
use sorth::prelude::*;
```
Next you will need to initialize your sorth execution engine. Start by creating your engine with 
```
let mut engine = Engine::new();
```
Then you will need to include standard word set which provides basic functionality.
```
let std_words = Standard::new();

engine.import_word_list(std_words);
```
This is the bare minimum required to get started with sorth language. The only function that you will use to interact with the engine is the ```eval``` function. This function takes a ```String``` as its input and executes it.Here is a example on how to take input from stdin and interpret it.
```
while engine.running {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match engine.eval(line) {
        Ok(ok) => println!("{}", ok.trim()),
        Err(err) => println!("Error: {}", err.trim()),
    }
}
```
This example as a whole can be found in examples folder or executed by running
```
cargo run --examples terminal
```

## Extending the wordset

If you would like to extend the wordset with additional functionality from your code it is very possible. It is done in exactly the same manner as including the standard wordset. You will need to create a struct that implements the ```WordList``` trait. This trait will allow you to import the wordset into sorth engine. Here is a example of such struct
```
sturct standard {
    words: Vec<Word>,
}
```
The word type is essentialy a tuple with two main elements. Element no. 1 is a function that tell the engine when will the word be executed. Whereas the second element is a function that tells the engine what to do exactly. Here is the definition of the word and a example of usage.
```
// Taken from word.rs
pub type WordSymbol = fn(s: &Engine) -> bool;
pub type WordDefinition = fn(s: &mut Engine) -> Result<String, String>;
pub type Word = (WordSymbol, WordDefinition);

// Taken from standard/mod.rs
(|s| s.get_curr_word() == "let" && s.mode_normal(), let_word),
//        Remember to specify this ^^^^^^^^^^^^^^^ Important!!!
```
The normal mode is the only mode that you want to do enything in if you don't want to break the standard wordset!!!

## no_std environment

If you want to use sorth in the ```no_std``` ecosystem, then the only that you need to provide is a ```global allocator``` [here](https://os.phil-opp.com/heap-allocation/) is a example on how to do it.