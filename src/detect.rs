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
    pub ci_name: TermVar,
    pub con_emu_ansi: TermVar,
    pub ci: TermVar,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TmuxVars {
    pub tmux_info: String,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TerminfoVars {
    pub truecolor: Option<bool>,
    pub max_colors: Option<i32>,
}

impl TerminfoVars {
    #[cfg(feature = "terminfo")]
    fn from_env() -> Self {
        if let Ok(info) = terminfo::Database::from_env() {
            Self {
                truecolor: info.get::<terminfo::capability::TrueColor>().map(|t| t.0),
                max_colors: info.get::<terminfo::capability::MaxColors>().map(|c| c.0),
            }
        } else {
            Self {
                truecolor: None,
                max_colors: None,
            }
        }
    }

    #[cfg(not(feature = "terminfo"))]
    fn from_env() -> Self {
        Self::default()
    }
}

impl TermVars {
    pub fn from_env() -> Self {
        Self {
            meta: TermMetaVars::from_env(),
            overrides: OverrideVars::from_env(),
            special: SpecialVars::from_env(),
            tmux: TmuxVars::from_env(),
            terminfo: TerminfoVars::from_env(),
            windows: WindowsVars::from_env(),
        }
    }
}

impl TermMetaVars {
    pub fn from_env() -> Self {
        Self {
            term: TermVar::from_env("TERM"),
            colorterm: TermVar::from_env("COLORTERM"),
            term_program: TermVar::from_env("TERM_PROGRAM"),
            term_program_version: TermVar::from_env("TERM_PROGRAM_VERSION"),
        }
    }

    fn is_dumb(&self) -> bool {
        self.term.0.as_deref() == Some("dumb")
    }
}

impl OverrideVars {
    pub fn from_env() -> Self {
        Self {
            no_color: TermVar::from_env("NO_COLOR"),
            force_color: TermVar::from_env("FORCE_COLOR"),
            clicolor: TermVar::from_env("CLICOLOR"),
            clicolor_force: TermVar::from_env("CLICOLOR_FORCE"),
            tty_force: TermVar::from_env("TTY_FORCE"),
        }
    }
}

impl SpecialVars {
    pub fn from_env() -> Self {
        Self {
            github_actions: TermVar::from_env("GITHUB_ACTIONS"),
            gitea_actions: TermVar::from_env("GITEA_ACTIONS"),
            circleci: TermVar::from_env("CIRCLECI"),
            gitlab_ci: TermVar::from_env("GITLAB_CI"),
            drone: TermVar::from_env("DRONE"),
            ci_name: TermVar::from_env("CI_NAME"),
            google_cloud_shell: TermVar::from_env("GOOGLE_CLOUD_SHELL"),
            appveyor: TermVar::from_env("APPVEYOR"),
            travis: TermVar::from_env("TRAVIS"),
            buildkite: TermVar::from_env("BUILDKITE"),
            agent_name: TermVar::from_env("AGENT_NAME"),
            teamcity_version: TermVar::from_env("TEAMCITY_VERSION"),
            tf_build: TermVar::from_env("TF_BUILD"),
            con_emu_ansi: TermVar::from_env("ConEmuANSI"),
            ci: TermVar::from_env("CI"),
        }
    }
}

impl TmuxVars {
    pub fn from_env() -> Self {
        Self::try_from_env().unwrap_or_default()
    }

    pub fn try_from_env() -> Result<Self, io::Error> {
        let tmux = TermVar::from_env("TMUX");
        let term = TermVar::from_env("TERM");
        // tmux var may be missing if using over ssh
        let tmux_info = if !tmux.is_empty() || term.value().starts_with("tmux") {
            let mut cmd = Command::new("tmux")
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

        Ok(Self { tmux_info })
    }
}

impl WindowsVars {
    #[cfg(all(windows, feature = "windows-version"))]
    pub fn from_env() -> Self {
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
            ansicon: TermVar::from_env("ANSICON"),
            ansicon_ver: TermVar::from_env("ANSICON_VER"),
            os_version,
            build_number,
            is_windows: true,
        }
    }

    #[cfg(not(all(windows, feature = "windows-version")))]
    fn from_env() -> Self {
        Self {
            ansicon: TermVar::from_env("ANSICON"),
            ansicon_ver: TermVar::from_env("ANSICON_VER"),
            os_version: 0,
            build_number: 0,
            is_windows: true,
        }
    }
}

impl TermProfile {
    pub fn detect<T>(output: &T) -> Self
    where
        T: IsTerminal,
    {
        Self::detect_with_vars(output, TermVars::from_env())
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
        let truecolor_platforms: [&TermVar; 4] = [
            &special.google_cloud_shell,
            &special.github_actions,
            &special.gitea_actions,
            &special.circleci,
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
        if term.starts_with("screen.") {
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
            && term_program != "tmux"
            && !term.starts_with("tmux")
        {
            return TermProfile::TrueColor;
        }

        if term.contains("color") || term.contains("ansi") {
            profile = profile.max(TermProfile::Ansi16);
        }

        if self.vars.terminfo.truecolor == Some(true) {
            return TermProfile::TrueColor;
        }

        const TERMINFO_MAX_COLORS: i32 = 16777216;
        let terminfo_colors = self.vars.terminfo.max_colors.unwrap_or(0);
        if terminfo_colors >= TERMINFO_MAX_COLORS {
            return TermProfile::TrueColor;
        }
        if terminfo_colors >= 256 {
            profile = profile.max(TermProfile::Ansi256);
        }

        profile
    }

    fn detect_tmux(&self) -> Option<TermProfile> {
        if !self.vars.meta.term.value().starts_with("tmux")
            && self.vars.tmux.tmux_info.is_empty()
            && self.vars.meta.term_program.value() != "tmux"
        {
            return None;
        }

        let tmux_info = self.vars.tmux.tmux_info.split("\n");
        for line in tmux_info {
            if (line.contains("Tc") || line.contains("RGB")) && line.contains("true") {
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

    fn from_env(var: &str) -> Self {
        Self::new_internal(env::var(var).ok())
    }

    fn is_truthy(&self) -> bool {
        self.0
            .as_deref()
            .map(|v| v == "1" || v == "true" || v == "yes")
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

#[cfg(test)]
#[path = "./detect_test.rs"]
mod detect_test;
