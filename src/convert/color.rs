use anstyle::{Ansi256Color, AnsiColor};

use crate::{AdaptableColor, TermProfile};

/// A color that can have different variants on the terminal's color support profile.
#[derive(Debug, Clone)]
pub struct ProfileColor<C> {
    default: C,
    ansi_256: Option<Ansi256Color>,
    ansi_16: Option<AnsiColor>,
    profile: TermProfile,
}

impl<C> ProfileColor<C>
where
    C: AdaptableColor + Clone,
{
    /// Creates a new [`ProfileColor`].
    pub fn new(default_color: C, profile: TermProfile) -> Self {
        Self {
            default: default_color,
            ansi_256: None,
            ansi_16: None,
            profile,
        }
    }

    /// Sets the color variant used when the terminal supports 256 colors.
    pub fn ansi_256<T>(mut self, color: T) -> Self
    where
        T: Into<C>,
    {
        self.ansi_256 = color.into().as_ansi_256();
        self
    }

    /// Sets the color variant when the terminal supports 16 colors.
    pub fn ansi_16<T>(mut self, color: T) -> Self
    where
        T: Into<C>,
    {
        self.ansi_16 = color.into().as_ansi_16();
        self
    }

    /// Returns the adapted color based on the profile.
    pub fn adapt(&self) -> Option<C> {
        let mut color = self.default.clone();
        if self.profile <= TermProfile::Ansi256
            && let Some(ansi_256) = self.ansi_256
        {
            color = C::from_ansi_256(ansi_256);
        }

        if self.profile <= TermProfile::Ansi16
            && let Some(ansi_16) = self.ansi_16
        {
            color = C::from_ansi_16(ansi_16);
        }
        self.profile.adapt_color(color)
    }
}
