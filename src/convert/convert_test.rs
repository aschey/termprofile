use anstyle::{Ansi256Color, AnsiColor, Color, Effects, RgbColor, Style};
use rstest::rstest;

use super::ProfileColor;
use crate::TermProfile;

#[rstest]
#[case(RgbColor(220, 90, 90), Ansi256Color(167))]
#[case(RgbColor(20, 73, 18), Ansi256Color(22))]
#[case(RgbColor(255, 0, 0), Ansi256Color(196))]
#[case(RgbColor(255, 255, 255), Ansi256Color(231))]
#[case(RgbColor(250, 250, 250), Ansi256Color(231))]
#[case(RgbColor(0, 0, 0), Ansi256Color(16))]
fn rgb_to_ansi256(#[case] in_color: RgbColor, #[case] out_color: Ansi256Color) {
    let res: Color = TermProfile::Ansi256.adapt_color(in_color.into()).unwrap();
    assert_eq!(res, Color::Ansi256(out_color));

    let res = TermProfile::Ansi256.adapt_style(
        Style::new()
            .fg_color(Some(in_color.into()))
            .bg_color(Some(in_color.into()))
            .effects(Effects::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg_color(Some(out_color.into()))
            .bg_color(Some(out_color.into()))
            .effects(Effects::BOLD),
    );
}

#[rstest]
#[case(RgbColor(220, 90, 90), AnsiColor::Yellow)]
#[case(RgbColor(20, 73, 18), AnsiColor::Green)]
#[case(RgbColor(255, 0, 0), AnsiColor::BrightRed)]
#[case(RgbColor(255, 255, 255), AnsiColor::BrightWhite)]
#[case(RgbColor(0, 0, 0), AnsiColor::Black)]
fn rgb_to_ansi16(#[case] in_color: RgbColor, #[case] out_color: AnsiColor) {
    let res: Color = TermProfile::Ansi16.adapt_color(in_color.into()).unwrap();
    assert_eq!(res, Color::Ansi(out_color));

    let res = TermProfile::Ansi16.adapt_style(
        Style::new()
            .fg_color(Some(in_color.into()))
            .bg_color(Some(in_color.into()))
            .effects(Effects::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg_color(Some(out_color.into()))
            .bg_color(Some(out_color.into()))
            .effects(Effects::BOLD),
    );
}

#[rstest]
#[case(Ansi256Color(167), AnsiColor::Yellow)]
#[case(Ansi256Color(0), AnsiColor::Black)]
fn ansi256_to_ansi(#[case] in_color: Ansi256Color, #[case] out_color: AnsiColor) {
    let res: Color = TermProfile::Ansi16.adapt_color(in_color.into()).unwrap();
    assert_eq!(res, Color::Ansi(out_color));

    let res = TermProfile::Ansi16.adapt_style(
        Style::new()
            .fg_color(Some(in_color.into()))
            .bg_color(Some(in_color.into()))
            .effects(Effects::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg_color(Some(out_color.into()))
            .bg_color(Some(out_color.into()))
            .effects(Effects::BOLD),
    );
}

#[test]
fn ratatui_reset() {
    let res = TermProfile::Ansi16
        .adapt_color(ratatui::style::Color::Reset)
        .unwrap();
    assert_eq!(res, ratatui::style::Color::Reset);
}

#[test]
fn ascii() {
    let color = Color::Rgb(RgbColor(0, 0, 0));
    let res = TermProfile::NoColor.adapt_color(color);
    assert!(res.is_none());

    let res =
        TermProfile::NoColor.adapt_style(Style::new().fg_color(Some(color)).effects(Effects::BOLD));
    assert_eq!(res, Style::new().effects(Effects::BOLD));
}

#[test]
fn no_tty() {
    let color = Color::Rgb(RgbColor(0, 0, 0));
    let res = TermProfile::NoTty.adapt_color(color);
    assert!(res.is_none());

    let res =
        TermProfile::NoTty.adapt_style(Style::new().fg_color(Some(color)).effects(Effects::BOLD));
    assert_eq!(res, Style::new());
}

#[rstest]
#[case(TermProfile::TrueColor, Color::Rgb(RgbColor(0, 0, 0)))]
#[case(TermProfile::Ansi256, Color::Ansi256(Ansi256Color(0)))]
#[case(TermProfile::Ansi16, Color::Ansi(AnsiColor::Black))]
fn no_change(#[case] profile: TermProfile, #[case] color: Color) {
    let res = profile.adapt_color(color).unwrap();
    assert_eq!(res, color);
}

#[test]
fn profile_color_no_change() {
    let color: ProfileColor<Color> = ProfileColor::new(RgbColor(0, 0, 0), TermProfile::TrueColor);
    assert_eq!(color.adapt(), Some(RgbColor(0, 0, 0).into()));
}

#[test]
fn profile_color_adapt() {
    let color: ProfileColor<Color> = ProfileColor::new(RgbColor(0, 0, 0), TermProfile::Ansi256);
    assert_eq!(color.adapt(), Some(Ansi256Color(16).into()));
}

#[test]
fn profile_color_256_override() {
    let color: ProfileColor<Color> =
        ProfileColor::new(RgbColor(0, 0, 0), TermProfile::Ansi256).ansi_256(0);
    assert_eq!(color.adapt(), Some(Ansi256Color(0).into()));
}

#[test]
fn profile_color_16_override() {
    let color: ProfileColor<Color> =
        ProfileColor::new(RgbColor(0, 0, 0), TermProfile::Ansi16).ansi_16(AnsiColor::BrightBlack);
    assert_eq!(color.adapt(), Some(AnsiColor::BrightBlack.into()));
}

#[test]
fn profile_color_downsample_priority() {
    let color: ProfileColor<Color> =
        ProfileColor::new(RgbColor(0, 0, 0), TermProfile::Ansi16).ansi_256(8);
    assert_eq!(color.adapt(), Some(AnsiColor::BrightBlack.into()));
}
