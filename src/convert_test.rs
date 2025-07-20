use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor};
use rstest::rstest;

use crate::TermProfile;

#[rstest]
#[case(RgbColor(220, 90, 90), Ansi256Color(167))]
#[case(RgbColor(20, 73, 18), Ansi256Color(22))]
#[case(RgbColor(255, 0, 0), Ansi256Color(196))]
#[case(RgbColor(255, 255, 255), Ansi256Color(231))]
#[case(RgbColor(250, 250, 250), Ansi256Color(231))]
#[case(RgbColor(0, 0, 0), Ansi256Color(16))]
fn rgb_to_ansi256(#[case] in_color: RgbColor, #[case] out_color: Ansi256Color) {
    let res = TermProfile::Ansi256
        .adapt_color(Color::Rgb(in_color))
        .unwrap();
    assert_eq!(res, Color::Ansi256(out_color));
}

#[rstest]
#[case(RgbColor(220, 90, 90), AnsiColor::BrightRed)]
#[case(RgbColor(20, 73, 18), AnsiColor::Green)]
#[case(RgbColor(255, 0, 0), AnsiColor::BrightRed)]
#[case(RgbColor(255, 255, 255), AnsiColor::BrightWhite)]
#[case(RgbColor(0, 0, 0), AnsiColor::Black)]
fn rgb_to_ansi16(#[case] in_color: RgbColor, #[case] out_color: AnsiColor) {
    let res = TermProfile::Ansi16
        .adapt_color(Color::Rgb(in_color))
        .unwrap();
    assert_eq!(res, Color::Ansi(out_color));
}

#[rstest]
#[case(Ansi256Color(167), AnsiColor::BrightRed)]
#[case(Ansi256Color(0), AnsiColor::Black)]
fn ansi256_to_ansi(#[case] in_color: Ansi256Color, #[case] out_color: AnsiColor) {
    let res = TermProfile::Ansi16
        .adapt_color(Color::Ansi256(in_color))
        .unwrap();
    assert_eq!(res, Color::Ansi(out_color));
}

#[test]
fn ascii() {
    let res = TermProfile::Ascii.adapt_color(Color::Rgb(RgbColor(0, 0, 0)));
    assert!(res.is_none());
}

#[test]
fn no_tty() {
    let res = TermProfile::NoTty.adapt_color(Color::Rgb(RgbColor(0, 0, 0)));
    assert!(res.is_none());
}

#[test]
fn no_change() {
    let res = TermProfile::TrueColor
        .adapt_color(Color::Rgb(RgbColor(0, 0, 0)))
        .unwrap();
    assert_eq!(res, Color::Rgb(RgbColor(0, 0, 0)));
}
