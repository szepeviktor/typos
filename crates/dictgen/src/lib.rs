#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "codegen")]
mod gen;
#[cfg(feature = "map")]
mod map;
mod table;
mod trie;

#[cfg(feature = "codegen")]
pub use gen::*;
#[cfg(feature = "map")]
pub use map::*;
pub use table::*;
pub use trie::*;
