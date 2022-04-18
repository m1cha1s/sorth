// Execution engine
pub mod engine;

// Keyword sets
pub mod standard;

pub mod prelude {
    pub use crate::sorth::engine::*;
}
