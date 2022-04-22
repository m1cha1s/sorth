// Execution engine
pub mod engine;

// Keyword sets
pub mod standard;
pub mod word;

pub mod prelude {
    pub use super::engine::*;
    pub use super::standard::*;
    pub use super::word::*;
}

#[cfg(test)]
mod tests {}
