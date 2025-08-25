use ratatui::style::{Color, Modifier, Style};
use rstest::rstest;

use crate::{ProfileColor, TermProfile};

#[rstest]
#[case(Color::Rgb(220, 90, 90), Color::Indexed(167))]
#[case(Color::Rgb(20, 73, 18), Color::Indexed(22))]
#[case(Color::Rgb(255, 0, 0), Color::Indexed(196))]
#[case(Color::Rgb(255, 255, 255), Color::Indexed(231))]
#[case(Color::Rgb(250, 250, 250), Color::Indexed(231))]
#[case(Color::Rgb(0, 0, 0), Color::Indexed(16))]
fn rgb_to_ansi256(#[case] in_color: Color, #[case] out_color: Color) {
    let res: Color = TermProfile::Ansi256.adapt_color(in_color).unwrap();
    assert_eq!(res, out_color);

    let res = TermProfile::Ansi256.adapt_style(
        Style::new()
            .fg(in_color)
            .bg(in_color)
            .add_modifier(Modifier::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg(out_color)
            .bg(out_color)
            .add_modifier(Modifier::BOLD),
    );
}

#[rstest]
#[case(Color::Rgb(220, 90, 90), Color::Yellow)]
#[case(Color::Rgb(20, 73, 18), Color::Green)]
#[case(Color::Rgb(255, 0, 0), Color::LightRed)]
#[case(Color::Rgb(255, 255, 255), Color::White)]
#[case(Color::Rgb(0, 0, 0), Color::Black)]
fn rgb_to_ansi16(#[case] in_color: Color, #[case] out_color: Color) {
    let res: Color = TermProfile::Ansi16.adapt_color(in_color).unwrap();
    assert_eq!(res, out_color);

    let res = TermProfile::Ansi16.adapt_style(
        Style::new()
            .fg(in_color)
            .bg(in_color)
            .add_modifier(Modifier::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg(out_color)
            .bg(out_color)
            .add_modifier(Modifier::BOLD),
    );
}

#[rstest]
#[case(Color::Indexed(167), Color::Yellow)]
#[case(Color::Indexed(0), Color::Black)]
fn ansi256_to_ansi(#[case] in_color: Color, #[case] out_color: Color) {
    let res: Color = TermProfile::Ansi16.adapt_color(in_color).unwrap();
    assert_eq!(res, out_color);

    let res = TermProfile::Ansi16.adapt_style(
        Style::new()
            .fg(in_color)
            .bg(in_color)
            .add_modifier(Modifier::BOLD),
    );
    assert_eq!(
        res,
        Style::new()
            .fg(out_color)
            .bg(out_color)
            .add_modifier(Modifier::BOLD),
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
    let color = Color::Rgb(0, 0, 0);
    let res = TermProfile::NoColor.adapt_color(color);
    assert!(res.is_none());

    let res = TermProfile::NoColor.adapt_style(Style::new().fg(color).add_modifier(Modifier::BOLD));
    assert_eq!(res, Style::new().add_modifier(Modifier::BOLD));
}

#[test]
fn no_tty() {
    let color = Color::Rgb(0, 0, 0);
    let res = TermProfile::NoTty.adapt_color(color);
    assert!(res.is_none());

    let res = TermProfile::NoTty.adapt_style(Style::new().fg(color).add_modifier(Modifier::BOLD));
    assert_eq!(res, Style::new());
}

#[rstest]
#[case(TermProfile::TrueColor, Color::Rgb(0, 0, 0))]
#[case(TermProfile::Ansi256, Color::Indexed(0))]
#[case(TermProfile::Ansi16, Color::Black)]
fn no_change(#[case] profile: TermProfile, #[case] color: Color) {
    let res = profile.adapt_color(color).unwrap();
    assert_eq!(res, color);
}

#[test]
fn profile_color_no_change() {
    let color: ProfileColor<Color> = ProfileColor::new(Color::Rgb(0, 0, 0), TermProfile::TrueColor);
    assert_eq!(color.adapt(), Some(Color::Rgb(0, 0, 0)));
}

#[test]
fn profile_color_adapt() {
    let color: ProfileColor<Color> = ProfileColor::new(Color::Rgb(0, 0, 0), TermProfile::Ansi256);
    assert_eq!(color.adapt(), Some(Color::Indexed(16)));
}

#[test]
fn profile_color_256_override() {
    let color: ProfileColor<Color> =
        ProfileColor::new(Color::Rgb(0, 0, 0), TermProfile::Ansi256).ansi_256(Color::Indexed(0));
    assert_eq!(color.adapt(), Some(Color::Indexed(0)));
}

#[test]
fn profile_color_16_override() {
    let color: ProfileColor<Color> =
        ProfileColor::new(Color::Rgb(0, 0, 0), TermProfile::Ansi16).ansi_16(Color::DarkGray);
    assert_eq!(color.adapt(), Some(Color::DarkGray));
}

#[test]
fn profile_color_downsample_priority() {
    let color: ProfileColor<Color> =
        ProfileColor::new(Color::Rgb(0, 0, 0), TermProfile::Ansi16).ansi_256(Color::Indexed(8));
    assert_eq!(color.adapt(), Some(Color::DarkGray));
}
