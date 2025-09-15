use std::collections::{BTreeMap, HashMap};
use std::env;
use std::io::{self, Read};
use std::process::{Command, Stdio};

use crate::TermProfile;

// Rules
// https://bixense.com/clicolors/
// https://no-color.org/

pub trait IsTerminal {
    fn is_terminal(&self) -> bool;
}

impl<T> IsTerminal for T
where
    T: std::io::IsTerminal,
{
    fn is_terminal(&self) -> bool {
        self.is_terminal()
    }
}

pub trait VariableSource {
    fn var(&self, key: &str) -> Option<String>;
}

pub struct Env;

impl VariableSource for Env {
    fn var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
}

impl VariableSource for HashMap<String, String> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).cloned()
    }
}

impl VariableSource for HashMap<&str, &str> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.to_string())
    }
}

impl VariableSource for BTreeMap<String, String> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).cloned()
    }
}

impl VariableSource for BTreeMap<&str, &str> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.to_string())
    }
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TermVars {
    pub overrides: OverrideVars,
    pub meta: TermMetaVars,
    pub special: SpecialVars,
    pub tmux: TmuxVars,
    pub windows: WindowsVars,
    pub terminfo: TerminfoVars,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct OverrideVars {
    pub force_color: TermVar,
    pub clicolor_force: TermVar,
    pub clicolor: TermVar,
    pub no_color: TermVar,
    pub tty_force: TermVar,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TermMetaVars {
    pub term: TermVar,
    pub colorterm: TermVar,
    pub term_program: TermVar,
    pub term_program_version: TermVar,
    pub dcs_response: bool,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WindowsVars {
    pub ansicon: TermVar,
    pub ansicon_ver: TermVar,
    pub build_number: u64,
    pub os_version: u64,
    pub is_windows: bool,
    // Note: Windows terminal developers recommend against using WT_SESSION
    // https://github.com/Textualize/rich/issues/140
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SpecialVars {
    pub google_cloud_shell: TermVar,
    pub github_actions: TermVar,
    pub gitea_actions: TermVar,
    pub circleci: TermVar,
    pub travis: TermVar,
    pub appveyor: TermVar,
    pub gitlab_ci: TermVar,
    pub buildkite: TermVar,
    pub drone: TermVar,
    pub teamcity_version: TermVar,
    pub tf_build: TermVar,
    pub agent_name: TermVar,
    pub cirrus_ci: TermVar,
    pub ci_name: TermVar,
    pub con_emu_ansi: TermVar,
    pub ci: TermVar,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TmuxVars {
    pub tmux_info: String,
    pub tmux: TermVar,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TerminfoVars {
    pub max_colors: Option<i32>,
    pub truecolor: Option<bool>,
}

const TERM: &str = "TERM";
const TERM_PROGRAM: &str = "TERM_PROGRAM";
const SCREEN: &str = "screen";
const TMUX: &str = "tmux";
const DUMB: &str = "dumb";
const TC: &str = "Tc";
const RGB: &str = "RGB";

#[cfg(feature = "terminfo")]
fn get_ext_bool(info: &termini::TermInfo, name: &str) -> Option<bool> {
    info.extended_cap(name).map(|c| c == termini::Value::True)
}

impl TerminfoVars {
    #[cfg(feature = "terminfo")]
    fn from_env(settings: &DetectorSettings) -> Self {
        if settings.enable_terminfo
            && let Ok(info) = termini::TermInfo::from_env()
        {
            Self {
                // Tc/RGB are newer terminfo extensions that seem to be sparsely documented, but
                // some newer terminals support it since the max colors property has
                // some compatibility issues
                truecolor: get_ext_bool(&info, TC).or_else(|| get_ext_bool(&info, RGB)),
                max_colors: info.number_cap(termini::NumberCapability::MaxColors),
            }
        } else {
            Self {
                truecolor: None,
                max_colors: None,
            }
        }
    }

    #[cfg(not(feature = "terminfo"))]
    fn from_env(_settings: &DetectorSettings) -> Self {
        Self::default()
    }
}

impl TermVars {
    pub fn from_env(settings: DetectorSettings) -> Self {
        Self::from_source(&Env, settings)
    }

    pub fn from_source<S>(source: &S, settings: DetectorSettings) -> Self
    where
        S: VariableSource,
    {
        Self {
            meta: TermMetaVars::from_source(source, &settings),
            overrides: OverrideVars::from_source(source),
            special: SpecialVars::from_source(source),
            tmux: TmuxVars::from_source(source, &settings),
            terminfo: TerminfoVars::from_env(&settings),
            windows: WindowsVars::from_source(source),
        }
    }
}

impl TermMetaVars {
    pub fn from_source<S>(source: &S, #[allow(unused)] settings: &DetectorSettings) -> Self
    where
        S: VariableSource,
    {
        let term = TermVar::from_source(source, TERM);
        #[cfg(feature = "dcs-detect")]
        let dcs_response = if settings.enable_dcs {
            dcs_detect(source, term.0.as_deref().unwrap_or_default()).unwrap_or(false)
        } else {
            false
        };
        #[cfg(not(feature = "dcs-detect"))]
        let dcs_response = false;
        Self {
            term,
            colorterm: TermVar::from_source(source, "COLORTERM"),
            term_program: TermVar::from_source(source, TERM_PROGRAM),
            term_program_version: TermVar::from_source(source, "TERM_PROGRAM_VERSION"),
            dcs_response,
        }
    }

    fn is_dumb(&self) -> bool {
        self.term.0.as_deref() == Some(DUMB)
    }
}

fn prefix_or_equal(var: &str, compare: &str) -> bool {
    var == compare
        || var.starts_with(&format!("{compare}-"))
        || var.starts_with(&format!("{compare}."))
}

#[cfg(feature = "dcs-detect")]
fn dcs_detect<S>(source: &S, term: &str) -> io::Result<bool>
where
    S: VariableSource,
{
    use std::io::{Write, stdout};
    use std::time::Duration;

    use termina::escape::csi::{Csi, Device, Sgr};
    use termina::escape::dcs::{Dcs, DcsRequest, DcsResponse};
    use termina::style::RgbColor;
    use termina::{Event, PlatformTerminal, Terminal};
    const TEST_COLOR: RgbColor = RgbColor::new(150, 150, 150);

    // Screen and tmux don't support this sequence
    if !stdout().is_terminal()
        || term == DUMB
        || prefix_or_equal(term, TMUX)
        || !TermVar::from_source(source, &TMUX.to_ascii_uppercase()).is_empty()
        || prefix_or_equal(term, SCREEN)
    {
        return Ok(false);
    }

    let mut terminal = PlatformTerminal::new()?;
    terminal.enter_raw_mode()?;
    write!(
        terminal,
        "{}{}{}{}",
        Csi::Sgr(Sgr::Background(TEST_COLOR.into())),
        Dcs::Request(DcsRequest::GraphicRendition),
        Csi::Sgr(Sgr::Reset),
        Csi::Device(Device::RequestPrimaryDeviceAttributes),
    )?;
    terminal.flush()?;

    let mut true_color = false;
    loop {
        if !terminal.poll(Event::is_escape, Duration::from_millis(100).into())? {
            return Ok(false);
        }
        let event = terminal.read(Event::is_escape)?;

        match event {
            Event::Dcs(Dcs::Response {
                value: DcsResponse::GraphicRendition(sgrs),
                ..
            }) => {
                true_color = sgrs.contains(&Sgr::Background(TEST_COLOR.into()));
            }
            Event::Csi(Csi::Device(Device::DeviceAttributes(()))) => {
                break;
            }
            _ => {}
        }
    }
    Ok(true_color)
}

impl OverrideVars {
    pub fn from_source<S>(source: &S) -> Self
    where
        S: VariableSource,
    {
        Self {
            no_color: TermVar::from_source(source, "NO_COLOR"),
            force_color: TermVar::from_source(source, "FORCE_COLOR"),
            clicolor: TermVar::from_source(source, "CLICOLOR"),
            clicolor_force: TermVar::from_source(source, "CLICOLOR_FORCE"),
            tty_force: TermVar::from_source(source, "TTY_FORCE"),
        }
    }
}

impl SpecialVars {
    pub fn from_source<S>(source: &S) -> Self
    where
        S: VariableSource,
    {
        Self {
            github_actions: TermVar::from_source(source, "GITHUB_ACTIONS"),
            gitea_actions: TermVar::from_source(source, "GITEA_ACTIONS"),
            circleci: TermVar::from_source(source, "CIRCLECI"),
            gitlab_ci: TermVar::from_source(source, "GITLAB_CI"),
            drone: TermVar::from_source(source, "DRONE"),
            ci_name: TermVar::from_source(source, "CI_NAME"),
            google_cloud_shell: TermVar::from_source(source, "GOOGLE_CLOUD_SHELL"),
            appveyor: TermVar::from_source(source, "APPVEYOR"),
            travis: TermVar::from_source(source, "TRAVIS"),
            buildkite: TermVar::from_source(source, "BUILDKITE"),
            agent_name: TermVar::from_source(source, "AGENT_NAME"),
            teamcity_version: TermVar::from_source(source, "TEAMCITY_VERSION"),
            tf_build: TermVar::from_source(source, "TF_BUILD"),
            cirrus_ci: TermVar::from_source(source, "CIRRUS_CI"),
            con_emu_ansi: TermVar::from_source(source, "ConEmuANSI"),
            ci: TermVar::from_source(source, "CI"),
        }
    }
}

impl TmuxVars {
    pub fn from_source<S>(source: &S, settings: &DetectorSettings) -> Self
    where
        S: VariableSource,
    {
        Self::try_from_source(source, settings).unwrap_or_default()
    }

    pub fn try_from_source<S>(source: &S, settings: &DetectorSettings) -> Result<Self, io::Error>
    where
        S: VariableSource,
    {
        let tmux = TermVar::from_source(source, &TMUX.to_ascii_uppercase());
        let term = TermVar::from_source(source, TERM).value();
        let term_program = TermVar::from_source(source, TERM_PROGRAM).value();
        let is_tmux = !tmux.is_empty()
            || prefix_or_equal(&term, TMUX)
            || prefix_or_equal(&term_program, TMUX);

        // tmux var may be missing if using over ssh
        let tmux_info = if settings.enable_tmux_info && is_tmux {
            let mut cmd = Command::new(TMUX)
                .arg("info")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            cmd.wait()?;
            let mut out = String::new();
            cmd.stdout
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "stdout missing"))?
                .read_to_string(&mut out)?;
            out
        } else {
            String::new()
        };

        Ok(Self { tmux_info, tmux })
    }
}

impl WindowsVars {
    #[cfg(all(windows, feature = "windows-version"))]
    pub fn from_source<S>(source: &S) -> Self
    where
        S: VariableSource,
    {
        use os_info::Version;
        let info = os_info::get();
        let windows_version = info.version();

        let (os_version, build_number) =
            if let Version::Semantic(os_version, _, build_number) = windows_version {
                (*os_version, *build_number)
            } else {
                (0, 0)
            };
        Self {
            ansicon: TermVar::from_source(source, "ANSICON"),
            ansicon_ver: TermVar::from_source(source, "ANSICON_VER"),
            os_version,
            build_number,
            is_windows: true,
        }
    }

    #[cfg(not(all(windows, feature = "windows-version")))]
    fn from_source<S>(source: &S) -> Self
    where
        S: VariableSource,
    {
        Self {
            ansicon: TermVar::from_source(source, "ANSICON"),
            ansicon_ver: TermVar::from_source(source, "ANSICON_VER"),
            os_version: 0,
            build_number: 0,
            is_windows: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectorSettings {
    enable_dcs: bool,
    enable_terminfo: bool,
    enable_tmux_info: bool,
}

impl Default for DetectorSettings {
    fn default() -> Self {
        Self {
            enable_dcs: true,
            enable_terminfo: true,
            enable_tmux_info: true,
        }
    }
}

impl DetectorSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_dcs(mut self, enable_dcs: bool) -> Self {
        self.enable_dcs = enable_dcs;
        self
    }

    pub fn enable_terminfo(mut self, enable_terminfo: bool) -> Self {
        self.enable_terminfo = enable_terminfo;
        self
    }

    pub fn enable_tmux_info(mut self, enable_tmux_info: bool) -> Self {
        self.enable_tmux_info = enable_tmux_info;
        self
    }
}

impl TermProfile {
    pub fn detect<T>(output: &T, settings: DetectorSettings) -> Self
    where
        T: IsTerminal,
    {
        Self::detect_with_vars(output, TermVars::from_env(settings))
    }

    pub fn detect_with_vars<T>(output: &T, vars: TermVars) -> Self
    where
        T: IsTerminal,
    {
        let detector = Detector { vars };
        let profile = detector.detect_tty(output);
        if let Some(env) = detector.detect_no_color()
            && profile > TermProfile::NoTty
        {
            return env;
        }
        if let Some(env) = detector.detect_force_color() {
            return env;
        }
        if detector.vars.meta.dcs_response {
            return TermProfile::TrueColor;
        }
        if let Some(env) = detector.detect_special_cases() {
            return env;
        }
        if profile == TermProfile::NoTty {
            return profile;
        }

        detector.detect_term_vars()
    }
}

struct Detector {
    vars: TermVars,
}

impl Detector {
    fn detect_tty<T>(&self, output: &T) -> TermProfile
    where
        T: IsTerminal,
    {
        if (!self.vars.overrides.tty_force.is_truthy() && !output.is_terminal())
            || self.vars.meta.is_dumb()
        {
            TermProfile::NoTty
        } else {
            TermProfile::NoColor
        }
    }
    fn detect_no_color(&self) -> Option<TermProfile> {
        if self.vars.overrides.no_color.is_truthy() {
            Some(TermProfile::NoColor)
        } else {
            None
        }
    }

    fn detect_force_color(&self) -> Option<TermProfile> {
        let mut profile = if self.vars.overrides.clicolor.is_truthy() {
            Some(TermProfile::Ansi16)
        } else {
            None
        };
        let force_color = self
            .vars
            .overrides
            .clicolor_force
            .or(&self.vars.overrides.force_color);

        if force_color.is_truthy() {
            profile = profile.max(Some(TermProfile::Ansi16));
        }

        match force_color.value().as_str() {
            "no_color" => return Some(TermProfile::NoColor),
            "ansi" | "ansi16" => return Some(TermProfile::Ansi16),
            "ansi256" => return Some(TermProfile::Ansi256),
            "truecolor" => return Some(TermProfile::TrueColor),
            _ => {}
        };

        if self.vars.overrides.clicolor.is_truthy() {
            profile = profile.max(Some(TermProfile::Ansi16));
        }

        profile.map(|p| p.max(self.detect_term_vars()))
    }

    fn detect_special_cases(&self) -> Option<TermProfile> {
        let special = &self.vars.special;
        let truecolor_platforms: [&TermVar; 5] = [
            &special.google_cloud_shell,
            &special.github_actions,
            &special.gitea_actions,
            &special.circleci,
            &special.cirrus_ci,
        ];
        let ansi_platforms: [&TermVar; 6] = [
            &special.travis,
            &special.appveyor,
            &special.gitlab_ci,
            &special.buildkite,
            &special.drone,
            &special.teamcity_version,
        ];

        if truecolor_platforms.iter().any(|p| !p.is_empty()) {
            return Some(TermProfile::TrueColor);
        }

        if ansi_platforms.iter().any(|p| !p.is_empty()) {
            return Some(TermProfile::Ansi16);
        }

        // Azure pipelines
        if !special.tf_build.is_empty() && !self.vars.special.agent_name.is_empty() {
            return Some(TermProfile::Ansi16);
        }

        if special.ci_name.value() == "codeship" {
            return Some(TermProfile::Ansi16);
        }

        if special.ci.is_truthy() {
            return Some(TermProfile::Ansi16);
        }

        None
    }

    fn detect_term_vars(&self) -> TermProfile {
        let colorterm = self.vars.meta.colorterm.value();
        let mut term = self.vars.meta.term.value();
        let term_program = self.vars.meta.term_program.value();

        let mut profile = TermProfile::NoColor;

        if term.is_empty() {
            if let Some(win_profile) = self.detect_windows() {
                profile = win_profile;
            }
        } else {
            profile = TermProfile::Ansi16;
        }

        match term_program.as_str() {
            "mintty" => {
                // Supported as of 2015: https://github.com/mintty/mintty/commit/8e1f4c260b5e1b3311caf10e826d87c85b3c9433
                return TermProfile::TrueColor;
            }
            "iterm.app" => {
                let term_program_version = self
                    .vars
                    .meta
                    .term_program_version
                    .value()
                    .split(".")
                    .next()
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(0);
                if term_program_version >= 3 {
                    return TermProfile::TrueColor;
                } else {
                    return TermProfile::Ansi256;
                }
            }
            "apple_terminal" => return TermProfile::Ansi256,
            _ => {}
        }

        let mut is_screen = false;
        if prefix_or_equal(&term, SCREEN) {
            term = term.replacen("screen.", "", 1);
            is_screen = true;
            profile = profile.max(TermProfile::Ansi256);
        }
        let term_last = term.split("-").last().unwrap_or_default();
        match term_last {
            "alacritty" | "contour" | "rio" | "wezterm" | "ghostty" | "kitty" | "foot" | "st"
            | "direct" => {
                return TermProfile::TrueColor;
            }
            "256color" => {
                profile = profile.max(TermProfile::Ansi256);
            }
            "linux" | "xterm" => {
                profile = profile.max(TermProfile::Ansi16);
            }
            _ => {}
        }

        // tmux changes the TERM variable which could make this report 256 color or truecolor
        // incorrectly
        if let Some(tmux_profile) = self.detect_tmux() {
            profile = profile.max(tmux_profile);
        }

        // New versions of screen do support truecolor, but it must be enabled explicitly and
        // there doesn't appear to be an easy way to detect this.
        if (matches!(colorterm.as_str(), "24bit" | "truecolor")
            || self.vars.meta.colorterm.is_truthy())
            && !is_screen
            && !self.is_tmux()
        {
            return TermProfile::TrueColor;
        }

        if term.contains("color") || term.contains("ansi") {
            profile = profile.max(TermProfile::Ansi16);
        }

        if self.vars.terminfo.truecolor == Some(true) {
            return TermProfile::TrueColor;
        }

        const TERMINFO_MAX_COLORS: i32 = 256i32.pow(3);
        let terminfo_colors = self.vars.terminfo.max_colors.unwrap_or(0);
        if terminfo_colors >= TERMINFO_MAX_COLORS {
            return TermProfile::TrueColor;
        }
        if terminfo_colors >= 256 {
            profile = profile.max(TermProfile::Ansi256);
        }

        profile
    }

    fn is_tmux(&self) -> bool {
        !self.vars.tmux.tmux.is_empty()
            || prefix_or_equal(&self.vars.meta.term.value(), TMUX)
            || prefix_or_equal(&self.vars.meta.term_program.value(), TMUX)
    }

    fn detect_tmux(&self) -> Option<TermProfile> {
        if !self.is_tmux() {
            return None;
        }

        let tmux_info = self.vars.tmux.tmux_info.split("\n");
        for line in tmux_info {
            if (line.contains(TC) || line.contains(RGB)) && line.contains("true") {
                return Some(TermProfile::TrueColor);
            }
        }
        Some(TermProfile::Ansi256)
    }

    fn detect_windows(&self) -> Option<TermProfile> {
        if !self.vars.windows.is_windows {
            return None;
        }
        if self.vars.special.con_emu_ansi.value() == "on" {
            return Some(TermProfile::TrueColor);
        }
        if let Some(env) = self.detect_windows_version() {
            return Some(env);
        }
        None
    }

    fn detect_windows_version(&self) -> Option<TermProfile> {
        if self.vars.windows.os_version == 0 {
            return None;
        }

        if self.vars.windows.build_number < 10586 || self.vars.windows.os_version < 10 {
            if self.vars.windows.ansicon.is_empty() {
                return Some(TermProfile::NoColor);
            } else {
                let ansicon_version = self.vars.windows.ansicon_ver.value().parse::<u32>();
                if ansicon_version.map(|v| v >= 181).unwrap_or(false) {
                    return Some(TermProfile::Ansi256);
                } else {
                    return Some(TermProfile::Ansi16);
                }
            }
        }

        if self.vars.windows.build_number < 14931 {
            return Some(TermProfile::Ansi256);
        }

        Some(TermProfile::TrueColor)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TermVar(Option<String>);

impl TermVar {
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_internal(Some(value.into()))
    }

    fn new_internal(value: Option<String>) -> Self {
        Self(value.map(|v| v.trim_ascii().to_lowercase()))
    }

    fn from_source<S>(source: &S, var: &str) -> Self
    where
        S: VariableSource,
    {
        Self(source.var(var).map(|v| v.trim_ascii().to_lowercase()))
    }

    fn is_truthy(&self) -> bool {
        self.0
            .as_deref()
            .map(|v| v == "1" || v == "true" || v == "yes" || v == "on")
            .unwrap_or(false)
    }

    fn is_empty(&self) -> bool {
        self.0.as_deref().map(|v| v.is_empty()).unwrap_or(true)
    }

    fn or(&self, other: &TermVar) -> Self {
        Self(self.0.clone().or_else(|| other.0.clone()))
    }

    fn value(&self) -> String {
        self.0.clone().unwrap_or_default()
    }
}

impl<T> From<T> for TermVar
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
#[path = "./detect_test.rs"]
mod detect_test;
