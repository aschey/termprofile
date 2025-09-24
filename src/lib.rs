#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "convert")]
mod convert;
#[cfg(feature = "dcs-detect")]
mod dcs;
mod detect;

#[cfg(feature = "convert")]
pub use anstyle;
#[cfg(feature = "convert")]
pub use convert::*;
#[cfg(feature = "dcs-detect")]
pub use dcs::*;
pub use detect::*;

/// Terminal color profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    /// No terminal is attached. This may happen if the output is piped or if the program was not
    /// run from a TTY.
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
