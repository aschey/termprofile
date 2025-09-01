use rstest::rstest;

use super::{IsTerminal, TermVar, TermVars};
use crate::TermProfile;

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
    let mut vars = TermVars::default();
    vars.meta.colorterm = "24bit".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = "24bit".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn truecolor_truthy() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn ansi256_no_term() {
    let mut vars = TermVars::default();
    vars.meta.term = "xterm-256color".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn no_color() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn no_color_precedence() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    vars.overrides.force_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_override() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = truthy_var();
    vars.meta.term = "xterm-256color".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor_force() {
    let mut vars = TermVars::default();
    vars.overrides.clicolor_force = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_disabled() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "no_color".into();
    vars.meta.colorterm = "truecolor".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color_disabled_no_tty() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "0".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn force_color_level_truthy() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi_basic() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "ansi".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi256() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "ansi256".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn force_color_truecolor() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "truecolor".into();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn force_color_extended_override() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = "ansi256".into();
    vars.meta.colorterm = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor() {
    let mut vars = TermVars::default();
    vars.overrides.clicolor = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn clicolor_override() {
    let mut vars = TermVars::default();
    vars.overrides.clicolor = truthy_var();
    vars.meta.term = "xterm-256color".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("alacritty")]
#[case("wezterm")]
#[case("xterm-kitty")]
fn truecolor_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = term.into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[rstest]
#[case("xterm-256color")]
#[case("screen.xterm-256color")]
fn ansi256_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = term.into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("linux")]
#[case("xterm")]
fn ansi16_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = term.into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn screen() {
    let mut vars = TermVars::default();
    vars.meta.term = "screen.xterm-256color".into();
    vars.meta.colorterm = "truecolor".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term() {
    let mut vars = TermVars::default();
    vars.meta.term = "tmux-256color".into();
    vars.meta.colorterm = "truecolor".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term_program() {
    let mut vars = TermVars::default();
    vars.meta.term_program = "tmux".into();
    vars.meta.term = "xterm-256color".into();
    vars.meta.colorterm = "truecolor".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_truecolor() {
    let mut vars = TermVars::default();
    vars.meta.term = "tmux-256color".into();
    vars.tmux.tmux_info = "Tc: (flag) true".to_string();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn apple_terminal() {
    let mut vars = TermVars::default();
    vars.meta.term_program = "apple_terminal".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn mintty() {
    let mut vars = TermVars::default();
    vars.meta.term_program = "mintty".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm() {
    let mut vars = TermVars::default();
    vars.meta.term_program = "iterm.app".into();
    vars.meta.term_program_version = "3.0".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm_old() {
    let mut vars = TermVars::default();
    vars.meta.term_program = "iterm.app".into();
    vars.meta.term_program_version = "2.0".into();
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
    let mut vars = TermVars::default();
    vars.special.google_cloud_shell = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_ansi() {
    let mut vars = TermVars::default();
    vars.special.travis = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn special_var_ci() {
    let mut vars = TermVars::default();
    vars.special.ci = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_con_emu() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.special.con_emu_ansi = "ON".into();
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
    let mut vars = TermVars::default();
    vars.meta.term = "dumb".into();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn dumb_term_force_color() {
    let mut vars = TermVars::default();
    vars.meta.term = "dumb".into();
    vars.overrides.clicolor_force = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn osc_detect() {
    let mut vars = TermVars::default();
    vars.meta.osc_response = true;
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn osc_detect_no_color() {
    let mut vars = TermVars::default();
    vars.meta.osc_response = true;
    vars.overrides.no_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::NoColor, support);
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
