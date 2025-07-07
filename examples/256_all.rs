use anstyle::{Ansi256Color, Color, Style};
use anstyle_crossterm::to_crossterm;
use crossterm::style::ContentStyle;
use termprofile::TermProfile;

fn main() {
    for i in 0..=255 {
        let color = Color::Ansi256(Ansi256Color(i));
        print!("{} ", get_style(color).apply(format!("original: {i}")));

        let adapted = TermProfile::Ansi16.adapt(color).unwrap();
        let Color::Ansi(ansi) = adapted else {
            unreachable!()
        };
        println!("{}", get_style(adapted).apply(format!("adapted: {ansi:?}")));
    }
}

fn get_style(color: Color) -> ContentStyle {
    to_crossterm(Style::new().fg_color(Some(color)))
}
