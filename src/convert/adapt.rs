use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor, Style};

/// Represents a color that can be converted to each type of color level.
pub trait AdaptableColor {
    /// Returns the color as an [`RgbColor`] if a compatible representation exists.
    fn as_rgb(&self) -> Option<RgbColor>;
    /// Returns the color as an [`Ansi256Color`] if a compatible representation exists.
    fn as_ansi_256(&self) -> Option<Ansi256Color>;
    /// Returns the color as an [`AnsiColor`] if a compatible representation exists.
    fn as_ansi_16(&self) -> Option<AnsiColor>;
    /// Creates a new instance from an [`Ansi256Color`].
    fn from_ansi_256(color: Ansi256Color) -> Self;
    /// Creates a new instance from an [`AnsiColor`].
    fn from_ansi_16(color: AnsiColor) -> Self;
}

/// Represents a style that can get and set its color properties.
pub trait AdaptableStyle: Default {
    /// The color type used for the style properties.
    type Color: AdaptableColor;

    /// Returns the foreground color, if set.
    fn get_fg_color(&self) -> Option<Self::Color>;
    /// Returns the background color, if set.
    fn get_bg_color(&self) -> Option<Self::Color>;
    /// Returns the underline color, if set.
    fn get_underline_color(&self) -> Option<Self::Color>;
    /// Sets the foreground color.
    fn fg_color(self, color: Option<Self::Color>) -> Self;
    /// Sets the background color.
    fn bg_color(self, color: Option<Self::Color>) -> Self;
    /// Sets the underline color.
    fn underline_color(self, color: Option<Self::Color>) -> Self;
}

impl AdaptableColor for Color {
    fn as_rgb(&self) -> Option<RgbColor> {
        if let Self::Rgb(color) = self {
            Some(*color)
        } else {
            None
        }
    }

    fn as_ansi_256(&self) -> Option<Ansi256Color> {
        if let Self::Ansi256(color) = self {
            Some(*color)
        } else {
            None
        }
    }

    fn as_ansi_16(&self) -> Option<AnsiColor> {
        if let Self::Ansi(color) = self {
            Some(*color)
        } else {
            None
        }
    }

    fn from_ansi_256(color: Ansi256Color) -> Self {
        color.into()
    }

    fn from_ansi_16(color: AnsiColor) -> Self {
        color.into()
    }
}

impl AdaptableStyle for Style {
    type Color = Color;

    fn get_fg_color(&self) -> Option<Self::Color> {
        (*self).get_fg_color()
    }

    fn get_bg_color(&self) -> Option<Self::Color> {
        (*self).get_bg_color()
    }

    fn get_underline_color(&self) -> Option<Self::Color> {
        (*self).get_underline_color()
    }

    fn fg_color(self, color: Option<Self::Color>) -> Self {
        self.fg_color(color)
    }

    fn bg_color(self, color: Option<Self::Color>) -> Self {
        self.bg_color(color)
    }

    fn underline_color(self, color: Option<Self::Color>) -> Self {
        self.underline_color(color)
    }
}
