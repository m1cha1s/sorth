# Sorth

[![Tests](https://github.com/m1cha1s/sorth/actions/workflows/tests.yml/badge.svg)](https://github.com/m1cha1s/sorth/actions/workflows/tests.yml)

This is a **Rust** based language interpreter inspired by FORTH. It's main features are that it's easily extensible and that it is no_std friendly only requiering global allocator.

## Language features

Push value to the stack:
```
12 // int
" hello " // string
12.0 // double
12.0f // float
2l // long 
0x23 // byte

" hello    world " // Note. This will be interpreted as " hello world ". This is the limitation of the interpreters architecture
```

New line:
```
nl // Writes a newline to the output
```

Math funcitions:
```
// Currently available math operations
+ - * /

// Exmaple
12 24 + . // 36 Ok.
```

Logic operators and comparisons:
```
and // logic and
or  // logic or
not // logic not

==  // equal
!=  // not equal
>   // higher
<   // Lower

-1  // true
0   // false
```

Stack manipulation functions:
```
.    // Pops and prints
peek // Just prints
dup  // duplicates top of the stack
2dup // duplicates two highest values on the stack
drop // Drops top of the stack
swap // Swaps two highest values on the stack
rot  // Rotates 3 top values on the stack example [ 3 2 1 ] -> [ 1 3 2 ]
```

Custom words:
```
: // start defining
<name>
... // here put the contents of the word
; // end defining
```

If statements (only in custom words):
```
if ... else ... then
    ^ execute if true
             ^ execute if false
^ pop top of the stack and see if true
```

For statements (only in custom words):
```
<end> <start> for ... next // increment by 1 until >= than <end>
<end> <start> for ... <increment> bynext // increment by <increment> until >= than <end>
```

While statement (only in custom words):
```
while <condition> do ... again (you can ommit "<condition> do" if you want a never ending loop)
```

Variables:
```
let <variable name> // create a variable
@<variable name> // get variable address
<variable addr> <val> push // push <val> to varable
<variable addr> pop // pop value from variable and put it on the main stack
<variable addr> <addr> get // get value from variable table at position <addr>
<variable addr> <addr> <val> set // set value of variable table at position <addr> to <val> if position exists
<varible addr> len // get lenght of the variable internal stack
```

Type converion:
```
// Convert top of the stack to the respectable value
to_int
to_long
to_float
to_double
to_byte
to_str
```

Input:
```
" This message will be displayed as a question/request for the input to the user " input // pushes user input to the stack
```

Silent mode:
```
<state either -1/0 > silent // turn the "Ok." messages on or off
```

Comments:
```
( this is a comment )
```

## Fibonacci example
```
: fib
    0
    for
    dup 
    rot
    +
    peek
    next
;

0l 1l 10 fib
```
This can be executed with:
```
cargo run --example file_exec fibonacci
```

## Usage

Add this to ```Cargo.toml```
```
[dependencies]
sorth = "0.2"
```
## Examples
### Terminal example
```
use sorth::prelude::*;

fn main() {
    let mut engine = Engine::new();

    let std_words = Standard::new();

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
```
Run with:
```
cargo run --examples terminal
```

### Execute from file example
To run the example use:
```
cargo run --example file_exec <Sorth program example>
```
Currently available Sorth program examples:
- ```fibonacci```
- ```turing```

For example
```
cargo run --example file_exec turing
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

## Change log:
[Here](CHANGELOG.md)
## Goals:

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
- [x] Sorth user input interface
- [x] Rework sorth into library crate
- [x] Share on crates.io
- [ ] Create a Book

## License

Sorth is distributed under MIT licence. See [LICENSE](LICENSE) for details.