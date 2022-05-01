// Execution engine
pub mod engine;

// Keyword sets
pub mod errors;
pub mod standard;
pub mod types;
pub mod word;

pub mod prelude {
    pub use crate::engine::*;
    pub use crate::errors::*;
    pub use crate::standard::*;
    pub use crate::types::*;
    pub use crate::word::*;
}

#[cfg(test)]
mod tests;
