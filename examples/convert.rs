use std::io::stdout;

use anstyle::{Color, RgbColor, Style};
use anstyle_crossterm::to_crossterm;
use termprofile::TermProfile;

fn main() {
    let color = Color::Rgb(RgbColor(rand_rgb(), rand_rgb(), rand_rgb()));
    let profile = TermProfile::detect(&stdout());
    print!("Detected profile: ");
    print_color(profile, color);
    if profile > TermProfile::Ansi256 {
        print!("ANSI 256: ");
        print_color(TermProfile::Ansi256, color);
    }
    if profile > TermProfile::Ansi16 {
        print!("ANSI 16: ");
        print_color(TermProfile::Ansi16, color);
    }
}

fn print_color(profile: TermProfile, color: Color) {
    let color = profile.adapt(color);
    if let Some(color) = color {
        let style = to_crossterm(Style::new().fg_color(Some(color)));
        println!("{}", style.apply(color_to_str(&color)));
    } else {
        println!("No color");
    }
}

fn rand_rgb() -> u8 {
    rand::random_range(0..256) as u8
}

fn color_to_str(color: &Color) -> String {
    match color {
        Color::Ansi(ansi) => format!("{ansi:?}"),
        Color::Ansi256(color) => color.0.to_string(),
        Color::Rgb(color) => format!("rgb({},{},{})", color.0, color.1, color.2),
    }
}
