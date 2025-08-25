#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "convert")]
mod convert;
mod detect;

#[cfg(feature = "convert")]
pub use anstyle;
#[cfg(feature = "convert")]
pub use convert::*;
pub use detect::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    NoTty,
    NoColor,
    Ansi16,
    Ansi256,
    TrueColor,
}
