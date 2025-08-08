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
    Ascii,
    Ansi16,
    Ansi256,
    TrueColor,
}
