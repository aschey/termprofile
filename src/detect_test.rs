use std::collections::HashMap;

use rstest::rstest;

use super::{IsTerminal, TermVar, TermVars};
use crate::{DetectorSettings, TermProfile};

#[test]
fn default_terminal() {
    let vars = TermVars::default();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn default_no_terminal() {
    let vars = TermVars::default();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn truecolor() {
    let vars = make_vars(&[("COLORTERM", "24bit")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let vars = make_vars(&[("COLORTERM", "24bit")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn truecolor_truthy() {
    let vars = make_vars(&[("COLORTERM", "1")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn ansi256_no_term() {
    let vars = make_vars(&[("TERM", "xterm-256color")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn no_color() {
    let vars = make_vars(&[("NO_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn no_color_precedence() {
    let vars = make_vars(&[("NO_COLOR", "1"), ("CLICOLOR_FORCE", "1")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color() {
    let vars = make_vars(&[("FORCE_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_override() {
    let vars = make_vars(&[("FORCE_COLOR", "1"), ("TERM", "xterm-256color")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor_force() {
    let vars = make_vars(&[("CLICOLOR_FORCE", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_disabled() {
    let vars = make_vars(&[("FORCE_COLOR", "no_color"), ("COLORTERM", "truecolor")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color_disabled_no_tty() {
    let vars = make_vars(&[("FORCE_COLOR", "0")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn force_color_level_truthy() {
    let vars = make_vars(&[("FORCE_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi_basic() {
    let vars = make_vars(&[("FORCE_COLOR", "ansi")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi256() {
    let vars = make_vars(&[("FORCE_COLOR", "ansi256")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn force_color_truecolor() {
    let vars = make_vars(&[("FORCE_COLOR", "truecolor")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn force_color_extended_override() {
    let vars = make_vars(&[("FORCE_COLOR", "ansi256"), ("COLORTERM", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor() {
    let vars = make_vars(&[("CLICOLOR", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn clicolor_override() {
    let vars = make_vars(&[("CLICOLOR", "1"), ("TERM", "xterm-256color")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("alacritty")]
#[case("wezterm")]
#[case("xterm-kitty")]
fn truecolor_term(#[case] term: &str) {
    let vars = make_vars(&[("TERM", term)]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[rstest]
#[case("xterm-256color")]
#[case("screen.xterm-256color")]
#[case("screen")]
fn ansi256_term(#[case] term: &str) {
    let vars = make_vars(&[("TERM", term)]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("linux")]
#[case("xterm")]
fn ansi16_term(#[case] term: &str) {
    let vars = make_vars(&[("TERM", term)]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn screen() {
    let vars = make_vars(&[
        ("TERM", "screen.xterm-256color"),
        ("COLORTERM", "truecolor"),
    ]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term() {
    let vars = make_vars(&[("TERM", "tmux-256color"), ("COLORTERM", "truecolor")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term_program() {
    let vars = make_vars(&[
        ("TERM_PROGRAM", "tmux"),
        ("TERM", "xterm-256color"),
        ("COLORTERM", "truecolor"),
    ]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_truecolor() {
    let mut vars = make_vars(&[("TERM", "tmux-256color")]);
    vars.tmux.tmux_info = "Tc: (flag) true".to_string();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn apple_terminal() {
    let vars = make_vars(&[("TERM_PROGRAM", "apple_terminal")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn mintty() {
    let vars = make_vars(&[("TERM_PROGRAM", "mintty")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm() {
    let vars = make_vars(&[
        ("TERM_PROGRAM", "iterm.app"),
        ("TERM_PROGRAM_VERSION", "3.0"),
    ]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm_old() {
    let vars = make_vars(&[
        ("TERM_PROGRAM", "iterm.app"),
        ("TERM_PROGRAM_VERSION", "2.0"),
    ]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn terminfo_truecolor() {
    let mut vars = TermVars::default();
    vars.terminfo.truecolor = Some(true);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn terminfo_256color() {
    let mut vars = TermVars::default();
    vars.terminfo.max_colors = Some(256);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn terminfo_max_colors() {
    let mut vars = TermVars::default();
    vars.terminfo.max_colors = Some(16777216);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_truecolor() {
    let vars = make_vars(&[("GOOGLE_CLOUD_SHELL", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_ansi() {
    let vars = make_vars(&[("TRAVIS", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn special_var_ci() {
    let vars = make_vars(&[("CI", "1")]);
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_con_emu() {
    let mut vars = make_vars(&[("ConEmuANSI", "ON")]);
    vars.windows.is_windows = true;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn windows_version_old() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn windows_version_old_ansicon() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    vars.windows.ansicon = truthy_var();
    vars.windows.ansicon_ver = "181".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn windows_version_old_ansicon_old() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    vars.windows.ansicon = truthy_var();
    vars.windows.ansicon_ver = "180".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_version_new_build_number_old() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 14930;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn windows_version_new() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 14931;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn dumb_term() {
    let vars = make_vars(&[("TERM", "dumb")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn dumb_term_force_color() {
    let vars = make_vars(&[("TERM", "dumb"), ("CLICOLOR_FORCE", "1")]);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn osc_detect() {
    let mut vars = TermVars::default();
    vars.meta.dcs_response = true;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn osc_detect_no_color() {
    let mut vars = make_vars(&[("NO_COLOR", "1")]);
    vars.meta.dcs_response = true;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

fn make_vars(vars: &[(&str, &str)]) -> TermVars {
    TermVars::from_source(
        &HashMap::from_iter(vars.iter().map(|(k, v)| (k.to_string(), v.to_string()))),
        DetectorSettings::new()
            .enable_dcs(false)
            .enable_terminfo(false),
    )
}

fn truthy_var() -> TermVar {
    "1".into()
}

struct ForceTerminal;

impl IsTerminal for ForceTerminal {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct ForceNoTerminal;

impl IsTerminal for ForceNoTerminal {
    fn is_terminal(&self) -> bool {
        false
    }
}
