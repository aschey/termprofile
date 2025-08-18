use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor, Style};

pub trait AdaptableColor {
    fn as_rgb(&self) -> Option<RgbColor>;
    fn as_ansi_256(&self) -> Option<Ansi256Color>;
    fn as_ansi_16(&self) -> Option<AnsiColor>;

    fn from_ansi_256(color: Ansi256Color) -> Self;
    fn from_ansi_16(color: AnsiColor) -> Self;
}

pub trait AdaptableStyle {
    type Color: AdaptableColor;

    fn empty() -> Self;

    fn get_fg_color(&self) -> Option<Self::Color>;
    fn get_bg_color(&self) -> Option<Self::Color>;
    fn get_underline_color(&self) -> Option<Self::Color>;

    fn fg_color(self, color: Option<Self::Color>) -> Self;
    fn bg_color(self, color: Option<Self::Color>) -> Self;
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

    fn empty() -> Self {
        Self::default()
    }

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
