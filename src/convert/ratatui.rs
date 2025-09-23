use ratatui::style::{Color, Style};

use super::{AdaptableColor, AdaptableStyle};

impl AdaptableColor for Color {
    fn as_rgb(&self) -> Option<anstyle::RgbColor> {
        if let Self::Rgb(r, g, b) = *self {
            Some((r, g, b).into())
        } else {
            None
        }
    }

    fn as_ansi_256(&self) -> Option<anstyle::Ansi256Color> {
        if let Self::Indexed(i) = *self {
            Some(i.into())
        } else {
            None
        }
    }

    fn as_ansi_16(&self) -> Option<anstyle::AnsiColor> {
        Some(match self {
            Self::Reset => None?,
            Self::Black => anstyle::AnsiColor::Black,
            Self::Red => anstyle::AnsiColor::Red,
            Self::Green => anstyle::AnsiColor::Green,
            Self::Yellow => anstyle::AnsiColor::Yellow,
            Self::Blue => anstyle::AnsiColor::Blue,
            Self::Magenta => anstyle::AnsiColor::Magenta,
            Self::Cyan => anstyle::AnsiColor::Cyan,
            Self::Gray => anstyle::AnsiColor::White,
            Self::DarkGray => anstyle::AnsiColor::BrightBlack,
            Self::LightRed => anstyle::AnsiColor::BrightRed,
            Self::LightGreen => anstyle::AnsiColor::BrightGreen,
            Self::LightYellow => anstyle::AnsiColor::BrightYellow,
            Self::LightBlue => anstyle::AnsiColor::BrightBlue,
            Self::LightMagenta => anstyle::AnsiColor::BrightMagenta,
            Self::LightCyan => anstyle::AnsiColor::BrightCyan,
            Self::White => anstyle::AnsiColor::BrightWhite,
            Self::Rgb(_, _, _) | Self::Indexed(_) => None?,
        })
    }

    fn from_ansi_256(color: anstyle::Ansi256Color) -> Self {
        Self::Indexed(color.0)
    }

    fn from_ansi_16(color: anstyle::AnsiColor) -> Self {
        match color {
            anstyle::AnsiColor::Black => Self::Black,
            anstyle::AnsiColor::Red => Self::Red,
            anstyle::AnsiColor::Green => Self::Green,
            anstyle::AnsiColor::Yellow => Self::Yellow,
            anstyle::AnsiColor::Blue => Self::Blue,
            anstyle::AnsiColor::Magenta => Self::Magenta,
            anstyle::AnsiColor::Cyan => Self::Cyan,
            anstyle::AnsiColor::White => Self::Gray,
            anstyle::AnsiColor::BrightBlack => Self::DarkGray,
            anstyle::AnsiColor::BrightRed => Self::LightRed,
            anstyle::AnsiColor::BrightGreen => Self::LightGreen,
            anstyle::AnsiColor::BrightYellow => Self::LightYellow,
            anstyle::AnsiColor::BrightBlue => Self::LightBlue,
            anstyle::AnsiColor::BrightMagenta => Self::LightMagenta,
            anstyle::AnsiColor::BrightCyan => Self::LightCyan,
            anstyle::AnsiColor::BrightWhite => Self::White,
        }
    }
}

impl AdaptableStyle for Style {
    type Color = Color;

    fn empty() -> Self {
        Self::default()
    }

    fn get_fg_color(&self) -> Option<Self::Color> {
        self.fg
    }

    fn fg_color(mut self, color: Option<Self::Color>) -> Self {
        if let Some(color) = color {
            self.fg(color)
        } else {
            self.fg = None;
            self
        }
    }

    fn get_bg_color(&self) -> Option<Self::Color> {
        self.bg
    }

    fn bg_color(mut self, color: Option<Self::Color>) -> Self {
        if let Some(color) = color {
            self.bg(color)
        } else {
            self.bg = None;
            self
        }
    }

    #[cfg(feature = "ratatui-underline-color")]
    fn get_underline_color(&self) -> Option<Self::Color> {
        self.underline_color
    }

    #[cfg(not(feature = "ratatui-underline-color"))]
    fn get_underline_color(&self) -> Option<Self::Color> {
        None
    }

    #[cfg(feature = "ratatui-underline-color")]
    fn underline_color(mut self, color: Option<Self::Color>) -> Self {
        if let Some(color) = color {
            self.underline_color(color)
        } else {
            self.underline_color = None;
            self
        }
    }

    #[cfg(not(feature = "ratatui-underline-color"))]
    fn underline_color(mut self, _color: Option<Self::Color>) -> Self {
        self
    }
}

#[cfg(test)]
#[path = "./ratatui_test.rs"]
mod ratatui_test;
