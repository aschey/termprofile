use ratatui::style::Color;

use super::AdaptableColor;

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
            Color::Reset => None?,
            Color::Black => anstyle::AnsiColor::Black,
            Color::Red => anstyle::AnsiColor::Red,
            Color::Green => anstyle::AnsiColor::Green,
            Color::Yellow => anstyle::AnsiColor::Yellow,
            Color::Blue => anstyle::AnsiColor::Blue,
            Color::Magenta => anstyle::AnsiColor::Magenta,
            Color::Cyan => anstyle::AnsiColor::Cyan,
            Color::Gray => anstyle::AnsiColor::White,
            Color::DarkGray => anstyle::AnsiColor::BrightBlack,
            Color::LightRed => anstyle::AnsiColor::BrightRed,
            Color::LightGreen => anstyle::AnsiColor::BrightGreen,
            Color::LightYellow => anstyle::AnsiColor::BrightYellow,
            Color::LightBlue => anstyle::AnsiColor::BrightBlue,
            Color::LightMagenta => anstyle::AnsiColor::BrightMagenta,
            Color::LightCyan => anstyle::AnsiColor::BrightCyan,
            Color::White => anstyle::AnsiColor::BrightWhite,
            Color::Rgb(_, _, _) => None?,
            Color::Indexed(_) => None?,
        })
    }

    fn from_ansi_256(color: anstyle::Ansi256Color) -> Self {
        Color::Indexed(color.0)
    }

    fn from_ansi_16(color: anstyle::AnsiColor) -> Self {
        match color {
            anstyle::AnsiColor::Black => Color::Black,
            anstyle::AnsiColor::Red => Color::Red,
            anstyle::AnsiColor::Green => Color::Green,
            anstyle::AnsiColor::Yellow => Color::Yellow,
            anstyle::AnsiColor::Blue => Color::Blue,
            anstyle::AnsiColor::Magenta => Color::Magenta,
            anstyle::AnsiColor::Cyan => Color::Cyan,
            anstyle::AnsiColor::White => Color::Gray,
            anstyle::AnsiColor::BrightBlack => Color::DarkGray,
            anstyle::AnsiColor::BrightRed => Color::LightRed,
            anstyle::AnsiColor::BrightGreen => Color::LightGreen,
            anstyle::AnsiColor::BrightYellow => Color::LightYellow,
            anstyle::AnsiColor::BrightBlue => Color::LightBlue,
            anstyle::AnsiColor::BrightMagenta => Color::LightMagenta,
            anstyle::AnsiColor::BrightCyan => Color::LightCyan,
            anstyle::AnsiColor::BrightWhite => Color::White,
        }
    }
}
