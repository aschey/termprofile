mod ansi_256_to_16;
mod color;
#[cfg(feature = "convert")]
mod convert;
mod detect;

pub use color::*;
pub use detect::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TermProfile {
    NoTty,
    Ascii,
    Ansi16,
    Ansi256,
    TrueColor,
}
