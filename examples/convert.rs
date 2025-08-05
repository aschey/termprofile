use std::io::stdout;

use anstyle::{Color, RgbColor, Style};
use anstyle_owo_colors::to_owo_style;
use owo_colors::OwoColorize;
use termprofile::TermProfile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let color: Color = if args.len() == 4 {
        let rgb = (
            args[1].parse::<u8>().unwrap(),
            args[2].parse::<u8>().unwrap(),
            args[3].parse::<u8>().unwrap(),
        );
        RgbColor(rgb.0, rgb.1, rgb.2).into()
    } else {
        RgbColor(rand_rgb(), rand_rgb(), rand_rgb()).into()
    };
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
    let color = profile.adapt_color(color);
    if let Some(color) = color {
        let style = to_owo_style(Style::new().fg_color(Some(color)));
        println!("{}", color_to_str(&color).style(style));
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
