mod composition;
mod edit_distance;
mod suggestion;
mod symspell;
mod wordmaps;

pub use composition::Composition;
pub use suggestion::Suggestion;
pub use symspell::{SymSpell, Verbosity, edits_prefix};
pub use wordmaps::WordRepr;
