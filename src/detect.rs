use std::{
    env,
    io::{self, Read},
    process::{Command, Stdio},
};

use crate::ColorSupport;

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
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct OverrideVars {
    pub force_color: TermVar,
    pub clicolor_force: TermVar,
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
    pub build_number: u32,
    pub os_version: u32,
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
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TmuxVars {
    pub tmux_info: String,
}

impl TermVars {
    pub fn from_env() -> Self {
        Self {
            meta: TermMetaVars::from_env(),
            overrides: OverrideVars::from_env(),
            special: SpecialVars::from_env(),
            tmux: TmuxVars::from_env(),
            #[cfg(all(windows, feature = "windows-version"))]
            windows: WindowsVars::from_os_info(),
            #[cfg(not(all(windows, feature = "windows-version")))]
            windows: WindowsVars::default(),
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
}

impl OverrideVars {
    pub fn from_env() -> Self {
        Self {
            no_color: TermVar::from_env("NO_COLOR"),
            force_color: TermVar::from_env("FORCE_COLOR"),
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
        }
    }
}

impl TmuxVars {
    pub fn from_env() -> Self {
        Self::try_from_env().unwrap_or_default()
    }

    pub fn try_from_env() -> Result<Self, io::Error> {
        let tmux = TermVar::from_env("TMUX");
        let tmux_info = if !tmux.is_empty() {
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
    pub fn from_os_info() -> Self {
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
        }
    }
}

impl ColorSupport {
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
        if let Some(env) = detector
            .detect_no_color()
            .or_else(|| detector.detect_force_color())
            .or_else(|| detector.detect_special_cases())
        {
            return env;
        }
        if !detector.vars.overrides.tty_force.is_truthy() && !output.is_terminal() {
            return Self::None;
        }
        detector.detect_term_vars()
    }
}

struct Detector {
    vars: TermVars,
}

impl Detector {
    fn detect_no_color(&self) -> Option<ColorSupport> {
        if self.vars.overrides.no_color.is_truthy() {
            Some(ColorSupport::None)
        } else {
            None
        }
    }

    fn detect_force_color(&self) -> Option<ColorSupport> {
        let force_color = self
            .vars
            .overrides
            .clicolor_force
            .or(&self.vars.overrides.force_color);

        if force_color.is_truthy() {
            let term = self.detect_term_vars();
            if matches!(term, ColorSupport::Ansi256 | ColorSupport::TrueColor) {
                // If the terminal reports it has better color support, don't force it to use
                // basic ANSI.
                return Some(term);
            }

            return Some(ColorSupport::Ansi16);
        }

        let level: u32 = force_color.value().parse().ok()?;
        match level {
            0 => Some(ColorSupport::None),
            1 => Some(ColorSupport::Ansi16),
            2 => Some(ColorSupport::Ansi256),
            3 => Some(ColorSupport::TrueColor),
            _ => None,
        }
    }

    fn detect_special_cases(&self) -> Option<ColorSupport> {
        let truecolor_platforms: [&TermVar; 4] = [
            &self.vars.special.google_cloud_shell,
            &self.vars.special.github_actions,
            &self.vars.special.gitea_actions,
            &self.vars.special.circleci,
        ];
        let ansi_platforms: [&TermVar; 6] = [
            &self.vars.special.travis,
            &self.vars.special.appveyor,
            &self.vars.special.gitlab_ci,
            &self.vars.special.buildkite,
            &self.vars.special.drone,
            &self.vars.special.teamcity_version,
        ];

        if truecolor_platforms.iter().any(|p| !p.is_empty()) {
            return Some(ColorSupport::TrueColor);
        }

        if ansi_platforms.iter().any(|p| !p.is_empty()) {
            return Some(ColorSupport::Ansi16);
        }

        // Azure pipelines
        if !self.vars.special.tf_build.is_empty() && !self.vars.special.agent_name.is_empty() {
            return Some(ColorSupport::Ansi16);
        }

        if self.vars.special.ci_name.value() == "codeship" {
            return Some(ColorSupport::Ansi16);
        }

        None
    }

    fn detect_term_vars(&self) -> ColorSupport {
        let colorterm = self.vars.meta.colorterm.value();
        let mut term = self.vars.meta.term.value();
        let term_program = self.vars.meta.term_program.value();

        let mut profile = ColorSupport::None;

        if term.is_empty() || term == "dumb" {
            if cfg!(windows) {
                if let Some(win_profile) = self.detect_windows() {
                    profile = win_profile;
                }
            }
        } else {
            profile = ColorSupport::Ansi16;
        }

        match term_program.as_str() {
            "mintty" => {
                // Supported as of 2015: https://github.com/mintty/mintty/commit/8e1f4c260b5e1b3311caf10e826d87c85b3c9433
                return ColorSupport::TrueColor;
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
                    return ColorSupport::TrueColor;
                } else {
                    return ColorSupport::Ansi256;
                }
            }
            "apple_terminal" => return ColorSupport::Ansi256,
            _ => {}
        }

        let mut is_screen = false;
        if term.starts_with("screen.") {
            term = term.replacen("screen.", "", 1);
            is_screen = true;
            profile = profile.max(ColorSupport::Ansi256);
        }
        let term_last = term.split("-").last().unwrap_or_default();
        match term_last {
            "alacritty" | "contour" | "rio" | "wezterm" | "ghostty" | "kitty" | "foot" | "st"
            | "direct" => {
                return ColorSupport::TrueColor;
            }
            "256color" => {
                profile = profile.max(ColorSupport::Ansi256);
            }
            "linux" | "xterm" => {
                profile = profile.max(ColorSupport::Ansi16);
            }
            _ => {}
        }

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
            return ColorSupport::TrueColor;
        }

        if term.contains("color") || term.contains("ansi") {
            profile = profile.max(ColorSupport::Ansi16);
        }

        profile
    }

    fn detect_tmux(&self) -> Option<ColorSupport> {
        if !self.vars.meta.term.value().starts_with("tmux")
            && self.vars.tmux.tmux_info.is_empty()
            && self.vars.meta.term_program.value() != "tmux"
        {
            return None;
        }

        let tmux_info = self.vars.tmux.tmux_info.split("\n");
        for line in tmux_info {
            if (line.contains("Tc") || line.contains("RGB")) && line.contains("true") {
                return Some(ColorSupport::TrueColor);
            }
        }
        Some(ColorSupport::Ansi256)
    }

    fn detect_windows(&self) -> Option<ColorSupport> {
        if self.vars.special.con_emu_ansi.value() == "ON" {
            return Some(ColorSupport::TrueColor);
        }
        #[cfg(all(windows, feature = "windows-version"))]
        if let Some(env) = detector.detect_windows_version() {
            return env;
        }
        None
    }

    #[cfg(all(windows, feature = "windows-version"))]
    fn detect_windows_version(&self) -> Option<ColorSupport> {
        if self.vars.windows.os_version == 0 {
            return None;
        }

        if self.vars.windows.build_number < 10586 || self.vars.windows.os_version < 10 {
            if self.vars.windows.ansicon.is_empty() {
                return Some(ColorSupport::None);
            } else {
                let ansicon_version = self.vars.windows.ansicon_ver.value().parse::<u32>();
                if ansicon_version.map(|v| v >= 181).unwrap_or(false) {
                    return Some(ColorSupport::Ansi256);
                } else {
                    return Some(ColorSupport::Ansi16);
                }
            }
        }

        if self.vars.windows.build_number < 14931 {
            return Some(ColorSupport::Ansi256);
        }

        Some(ColorSupport::TrueColor)
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
