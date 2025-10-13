#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "convert")]
mod convert;
mod detect;
#[cfg(feature = "query-detect")]
mod query;

#[cfg(feature = "convert")]
pub use anstyle;
#[cfg(feature = "convert")]
pub use convert::*;
pub use detect::*;
#[cfg(feature = "query-detect")]
pub use query::*;

/// Terminal color profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    /// No terminal is attached. This may happen if the output is piped or if the program was not
    /// ran from a TTY.
    NoTty,
    /// Text modifiers may be used, but no colors should be emitted.
    NoColor,
    /// 16 colors are supported.
    Ansi16,
    /// 256 colors are supported.
    Ansi256,
    /// Any RGB color is supported.
    TrueColor,
}
