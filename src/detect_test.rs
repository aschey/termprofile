use std::collections::{HashMap, VecDeque};
use std::io;

use rstest::rstest;

use super::{IsTerminal, TermVar, TermVars};
use crate::{DetectorSettings, Event, QueryTerminal, Rgb, TermProfile, WindowsVars};

#[test]
fn default_terminal() {
    let vars = make_vars(&ForceTerminal, &[]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn default_no_terminal() {
    let vars = make_vars(&ForceNoTerminal, &[]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn truecolor() {
    let vars = make_vars(&ForceTerminal, &[("COLORTERM", "24bit")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn truecolor_no_term() {
    let vars = make_vars(&ForceNoTerminal, &[("COLORTERM", "24bit")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn truecolor_truthy() {
    let vars = make_vars(&ForceTerminal, &[("COLORTERM", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn ansi256_no_term() {
    let vars = make_vars(&ForceNoTerminal, &[("TERM", "xterm-256color")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn no_color() {
    let vars = make_vars(&ForceTerminal, &[("NO_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn no_color_precedence() {
    let vars = make_vars(
        &ForceTerminal,
        &[("NO_COLOR", "1"), ("CLICOLOR_FORCE", "1")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_override() {
    let vars = make_vars(
        &ForceTerminal,
        &[("FORCE_COLOR", "1"), ("TERM", "xterm-256color")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor_force() {
    let vars = make_vars(&ForceNoTerminal, &[("CLICOLOR_FORCE", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_disabled() {
    let vars = make_vars(
        &ForceTerminal,
        &[("FORCE_COLOR", "no_color"), ("COLORTERM", "truecolor")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn force_color_disabled_no_tty() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "0")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn force_color_level_truthy() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi_basic() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "ansi")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn force_color_level_ansi256() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "ansi256")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn force_color_truecolor() {
    let vars = make_vars(&ForceNoTerminal, &[("FORCE_COLOR", "truecolor")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn force_color_extended_override() {
    let vars = make_vars(
        &ForceNoTerminal,
        &[("FORCE_COLOR", "ansi256"), ("COLORTERM", "1")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn clicolor() {
    let vars = make_vars(&ForceNoTerminal, &[("CLICOLOR", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn clicolor_override() {
    let vars = make_vars(
        &ForceTerminal,
        &[("CLICOLOR", "1"), ("TERM", "xterm-256color")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("alacritty")]
#[case("wezterm")]
#[case("xterm-kitty")]
fn truecolor_term(#[case] term: &str) {
    let vars = make_vars(&ForceTerminal, &[("TERM", term)]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[rstest]
#[case("xterm-256color")]
#[case("screen.xterm-256color")]
#[case("screen")]
fn ansi256_term(#[case] term: &str) {
    let vars = make_vars(&ForceTerminal, &[("TERM", term)]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[rstest]
#[case("linux")]
#[case("xterm")]
fn ansi16_term(#[case] term: &str) {
    let vars = make_vars(&ForceTerminal, &[("TERM", term)]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn screen() {
    let vars = make_vars(
        &ForceTerminal,
        &[
            ("TERM", "screen.xterm-256color"),
            ("COLORTERM", "truecolor"),
        ],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term() {
    let vars = make_vars(
        &ForceTerminal,
        &[("TERM", "tmux-256color"), ("COLORTERM", "truecolor")],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_term_program() {
    let vars = make_vars(
        &ForceTerminal,
        &[
            ("TERM_PROGRAM", "tmux"),
            ("TERM", "xterm-256color"),
            ("COLORTERM", "truecolor"),
        ],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn tmux_truecolor() {
    let mut vars = make_vars(&ForceTerminal, &[("TERM", "tmux-256color")]);
    vars.tmux.tmux_info = "Tc: (flag) true".to_string();
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn apple_terminal() {
    let vars = make_vars(&ForceTerminal, &[("TERM_PROGRAM", "apple_terminal")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn mintty() {
    let vars = make_vars(&ForceTerminal, &[("TERM_PROGRAM", "mintty")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm() {
    let vars = make_vars(
        &ForceTerminal,
        &[
            ("TERM_PROGRAM", "iterm.app"),
            ("TERM_PROGRAM_VERSION", "3.0"),
        ],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn iterm_old() {
    let vars = make_vars(
        &ForceTerminal,
        &[
            ("TERM_PROGRAM", "iterm.app"),
            ("TERM_PROGRAM_VERSION", "2.0"),
        ],
    );
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn terminfo_truecolor() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.terminfo.truecolor = Some(true);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn terminfo_256color() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.terminfo.max_colors = Some(256);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn terminfo_max_colors() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.terminfo.max_colors = Some(16777216);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_truecolor() {
    let vars = make_vars(&ForceNoTerminal, &[("GOOGLE_CLOUD_SHELL", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn special_var_ansi() {
    let vars = make_vars(&ForceNoTerminal, &[("TRAVIS", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn special_var_ci() {
    let vars = make_vars(&ForceNoTerminal, &[("CI", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_con_emu() {
    let mut vars = make_vars(&ForceTerminal, &[("ConEmuANSI", "ON")]);
    vars.windows.is_windows = true;
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn windows_version_old() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

#[test]
fn windows_version_old_ansicon() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    vars.windows.ansicon = truthy_var();
    vars.windows.ansicon_ver = "181".into();
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn windows_version_old_ansicon_old() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.windows.is_windows = true;
    vars.windows.build_number = 10585;
    vars.windows.os_version = 10;
    vars.windows.ansicon = truthy_var();
    vars.windows.ansicon_ver = "180".into();
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

#[test]
fn windows_version_new_build_number_old() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.windows.is_windows = true;
    vars.windows.build_number = 14930;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi256, support);
}

#[test]
fn windows_version_new() {
    let mut vars = make_vars(&ForceTerminal, &[]);
    vars.windows.is_windows = true;
    vars.windows.build_number = 14931;
    vars.windows.os_version = 10;
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn dumb_term() {
    let vars = make_vars(&ForceTerminal, &[("TERM", "dumb")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoTty, support);
}

#[test]
fn dumb_term_force_color() {
    let vars = make_vars(&ForceTerminal, &[("TERM", "dumb"), ("CLICOLOR_FORCE", "1")]);
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::Ansi16, support);
}

struct FakeTerminal {
    events: VecDeque<Event>,
}

impl QueryTerminal for FakeTerminal {
    fn setup(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn read_event(&mut self) -> std::io::Result<Event> {
        Ok(self.events.pop_front().unwrap())
    }
}

impl io::Write for FakeTerminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn dsc_detect() {
    let mut vars = TermVars::from_source(
        &HashMap::<&str, &str>::default(),
        &ForceTerminal,
        DetectorSettings::new()
            .enable_terminfo(false)
            .enable_tmux_info(false)
            .query_terminal(FakeTerminal {
                events: VecDeque::from_iter([
                    Event::BackgroundColor(Rgb {
                        red: 150,
                        green: 150,
                        blue: 150,
                    }),
                    Event::DeviceAttributes,
                ]),
            }),
    );
    // force reset windows vars to prevent inconsistencies
    vars.windows = WindowsVars::default();

    vars.meta.dcs_response = true;
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::TrueColor, support);
}

#[test]
fn dsc_detect_no_color() {
    let mut vars = TermVars::from_source(
        &HashMap::from_iter([("NO_COLOR", "1")]),
        &ForceTerminal,
        DetectorSettings::new()
            .enable_terminfo(false)
            .enable_tmux_info(false)
            .query_terminal(FakeTerminal {
                events: VecDeque::from_iter([
                    Event::BackgroundColor(Rgb {
                        red: 150,
                        green: 150,
                        blue: 150,
                    }),
                    Event::DeviceAttributes,
                ]),
            }),
    );
    // force reset windows vars to prevent inconsistencies
    vars.windows = WindowsVars::default();
    let support = TermProfile::detect_with_vars(vars);
    assert_eq!(TermProfile::NoColor, support);
}

fn make_vars<T>(out: &T, vars: &[(&str, &str)]) -> TermVars
where
    T: IsTerminal,
{
    let mut vars = TermVars::from_source(
        &HashMap::from_iter(vars.iter().copied()),
        out,
        DetectorSettings::new()
            .enable_terminfo(false)
            .enable_tmux_info(false),
    );
    // force reset windows vars to prevent inconsistencies
    vars.windows = WindowsVars::default();
    vars
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
