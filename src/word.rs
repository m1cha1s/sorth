use super::prelude::Engine;

pub type WordSymbol = fn(s: &Engine) -> bool;
pub type WordDefinition = fn(s: &mut Engine) -> Result<String, String>;
pub type Word = (WordSymbol, WordDefinition);

pub trait WordList {
    fn new() -> Self;

    fn get_words(&self) -> Vec<Word>;
}
