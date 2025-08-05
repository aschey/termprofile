use anstyle::{Ansi256Color, AnsiColor, Color, Style};
use anstyle_owo_colors::{to_owo_colors, to_owo_style};
use owo_colors::{DynColors, OwoColorize};
use termprofile::TermProfile;

fn main() {
    for i in 0..=255 {
        let color = Color::Ansi256(Ansi256Color(i));
        let adapted = TermProfile::Ansi16.adapt_color(color).unwrap();

        let Color::Ansi(ansi) = adapted else {
            unreachable!()
        };

        let (fg, bg) = if ansi == AnsiColor::Black {
            (AnsiColor::Black.into(), AnsiColor::White.into())
        } else {
            (AnsiColor::White.into(), AnsiColor::Black.into())
        };

        let text_style = style(fg, bg);
        let i_str = i.to_string();
        let adapt_str = format!("{adapted:?}");

        let s = format!(
            "{}{}{}{}",
            "original: ".style(text_style),
            i_str.style(style(color, bg)),
            " adapted: ".style(text_style),
            adapt_str.style(to_owo_style(
                Style::new().fg_color(Some(adapted)).bg_color(Some(bg))
            ))
        );

        println!("{s}");
    }
}

fn style(fg: Color, bg: Color) -> owo_colors::Style {
    to_owo_style(Style::new().fg_color(Some(fg)).bg_color(Some(bg)))
}
