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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    NoTty,
    NoColor,
    Ansi16,
    Ansi256,
    TrueColor,
}
