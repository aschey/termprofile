mod ansi_256_to_16;
#[cfg(feature = "convert")]
mod convert;
mod detect;

pub use detect::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    NoTty,
    Ascii,
    Ansi16,
    Ansi256,
    TrueColor,
}
