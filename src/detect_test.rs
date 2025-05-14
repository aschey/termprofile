use crate::ColorSupport;

use super::{IsTerminal, TermVar, TermVars};

#[test]
fn default_terminal() {
    let vars = TermVars::default();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn default_no_terminal() {
    let vars = TermVars::default();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn truecolor() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = TermVar::new(Some("24bit".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = TermVar::new(Some("24bit".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn truecolor_truthy() {
    let mut vars = TermVars::default();
    vars.meta.colorterm = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

#[test]
fn ansi256_term() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(Some("xterm-256color".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn ansi256_no_term() {
    let mut vars = TermVars::default();
    vars.meta.term = TermVar::new(Some("xterm-256color".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn no_color() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn no_color_precedence() {
    let mut vars = TermVars::default();
    vars.overrides.no_color = truthy_var();
    vars.overrides.force_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceTerminal, vars);
    assert_eq!(ColorSupport::None, support);
}

#[test]
fn force_color() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi16, support);
}

#[test]
fn clicolor_force() {
    let mut vars = TermVars::default();
    vars.overrides.clicolor_force = truthy_var();
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi16, support);
}

#[test]
fn force_color_level_2() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new(Some("2".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::Ansi256, support);
}

#[test]
fn force_color_level_3() {
    let mut vars = TermVars::default();
    vars.overrides.force_color = TermVar::new(Some("3".to_string()));
    let support = ColorSupport::detect_with_vars(&ForceNoTerminal, vars);
    assert_eq!(ColorSupport::TrueColor, support);
}

fn truthy_var() -> TermVar {
    TermVar::new(Some("1".to_string()))
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
