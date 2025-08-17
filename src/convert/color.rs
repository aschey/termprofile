use anstyle::{Ansi256Color, AnsiColor, Color};

use crate::TermProfile;

pub struct ProfileColor {
    default: Color,
    ansi_256: Option<Ansi256Color>,
    ansi_16: Option<AnsiColor>,
    profile: TermProfile,
}

impl ProfileColor {
    pub fn new<T>(default_color: T, profile: TermProfile) -> Self
    where
        T: Into<Color>,
    {
        Self {
            default: default_color.into(),
            ansi_256: None,
            ansi_16: None,
            profile,
        }
    }

    pub fn ansi_256<T>(mut self, color: T) -> Self
    where
        T: Into<Ansi256Color>,
    {
        self.ansi_256 = Some(color.into());
        self
    }

    pub fn ansi_16<T>(mut self, color: T) -> Self
    where
        T: Into<AnsiColor>,
    {
        self.ansi_16 = Some(color.into());
        self
    }
}

impl ProfileColor {
    pub fn adapt(&self) -> Option<Color> {
        let mut color = self.default;
        if self.profile <= TermProfile::Ansi256
            && let Some(ansi_256) = self.ansi_256
        {
            color = ansi_256.into();
        }

        if self.profile <= TermProfile::Ansi16
            && let Some(ansi_16) = self.ansi_16
        {
            color = ansi_16.into();
        }
        self.profile.adapt_color(color)
    }
}
