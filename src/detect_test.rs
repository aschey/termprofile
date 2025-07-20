use rstest::rstest;

use super::{IsTerminal, TermVar, TermVars};
use crate::TermProfile;

#[test]
fn default_terminal() {
    let vars = TermVars::default();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ascii, support);
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
    vars.meta.colorterm = TermVar::new("24bit");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = TermVar::new("24bit");
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
    vars.meta.term = TermVar::new("xterm-256color");
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn no_color() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ascii, support);
}

#[test]
fn no_color_precedence() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    vars.overrides.force_color = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ascii, support);
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
    vars.meta.term = TermVar::new("xterm-256color");
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
fn force_color_0() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("0");
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
    vars.overrides.force_color = TermVar::new("ansi");
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi256() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("ansi256");
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn force_color_truecolor() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("truecolor");
    let support = TermProfile::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn force_color_extended_override() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new("ansi256");
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
    vars.meta.term = TermVar::new("xterm-256color");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("alacritty")]
#[case("wezterm")]
#[case("xterm-kitty")]
fn truecolor_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[rstest]
#[case("xterm-256color")]
#[case("screen.xterm-256color")]
fn ansi256_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("linux")]
#[case("xterm")]
fn ansi16_term(#[case] term: &str) {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(term);
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn screen() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("screen.xterm-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("tmux-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term_program() {
    let mut vars = TermVars::default();
    vars.meta.term_program = TermVar::new("tmux");
    vars.meta.term = TermVar::new("xterm-256color");
    vars.meta.colorterm = TermVar::new("truecolor");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_truecolor() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new("tmux-256color");
    vars.tmux.tmux_info = "Tc: (flag) true".to_string();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn apple_terminal() {
    let mut vars = TermVars::default();
    vars.meta.term_program = TermVar::new("apple_terminal");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn iterm() {
    let mut vars = TermVars::default();
    vars.meta.term_program = TermVar::new("iterm.app");
    vars.meta.term_program_version = TermVar::new("3.0");
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
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
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_ansi() {
    let mut vars = TermVars::default();
    vars.special.travis = truthy_var();
    let support = TermProfile::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_con_emu() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.special.con_emu_ansi = TermVar::new("ON");
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
    assert_eq!(TermProfile::Ascii, support);
}

#[test]
fn windows_version_old_ansicon() {
    let mut vars = TermVars::default();
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    vars.windows.ansicon = truthy_var();
    vars.windows.ansicon_ver = TermVar::new("181");
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
    vars.windows.ansicon_ver = TermVar::new("180");
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

fn truthy_var() -> TermVar {
    TermVar::new("1")
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
