use anstyle::{Ansi256Color, Color, RgbColor};

use crate::TermProfile;

#[test]
fn rgb_to_256() {
    let res = TermProfile::Ansi256
        .adapt(Color::Rgb(RgbColor(220, 90, 90)))
        .unwrap();
    assert_eq!(res, Color::Ansi256(Ansi256Color(167)));
}
