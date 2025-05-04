use std::env;

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
    pub windows: WindowsVars,
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct OverrideVars {
    pub force_color: TermVar,
    pub clicolor_force: TermVar,
    pub no_color: TermVar,
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

impl TermVars {
    pub fn from_env() -> Self {
        Self {
            meta: TermMetaVars::from_env(),
            overrides: OverrideVars::from_env(),
            special: SpecialVars::from_env(),
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
    pub fn detect<T>(output: T) -> Self
    where
        T: IsTerminal,
    {
        Self::detect_with_vars(output, TermVars::from_env())
    }

    pub fn detect_with_vars<T>(output: T, vars: TermVars) -> Self
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
        if !output.is_terminal() {
            return Self::None;
        }
        if let Some(env) = detector.detect_term_vars() {
            return env;
        }
        #[cfg(windows)]
        if let Some(env) = detector.detect_windows_version() {
            return env;
        }
        ColorSupport::None
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
            .force_color
            .or(&self.vars.overrides.clicolor_force);

        if force_color.is_truthy() {
            if let Some(term) = self.detect_term_vars() {
                if matches!(term, ColorSupport::Ansi256 | ColorSupport::TrueColor) {
                    // If the terminal reports it has better color support, don't force it to use
                    // basic ANSI.
                    return Some(term);
                }
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

        if self.vars.special.con_emu_ansi.value() == "ON" {
            return Some(ColorSupport::TrueColor);
        }

        None
    }

    fn detect_term_vars(&self) -> Option<ColorSupport> {
        let colorterm = self.vars.meta.colorterm.value();
        let term = self.vars.meta.term.value();
        let term_program = self.vars.meta.term_program.value();

        if matches!(colorterm.as_str(), "24bit" | "truecolor") {
            // New versions of screen do support truecolor, but it must be enabled explicitly and
            // there doesn't appear to be an easy way to detect this.
            if term.starts_with("screen") && term_program != "tmux" {
                return Some(ColorSupport::Ansi256);
            }
            return Some(ColorSupport::TrueColor);
        }

        if self.vars.meta.colorterm.is_truthy() {
            return Some(ColorSupport::Ansi256);
        }

        match term_program.as_str() {
            "mintty" => {
                // Supported as of 2015: https://github.com/mintty/mintty/commit/8e1f4c260b5e1b3311caf10e826d87c85b3c9433
                return Some(ColorSupport::TrueColor);
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
                    return Some(ColorSupport::TrueColor);
                } else {
                    return Some(ColorSupport::Ansi256);
                }
            }
            "apple_terminal" => return Some(ColorSupport::Ansi256),
            _ => {}
        }

        match term.as_str() {
            "alacritty" | "contour" | "rio" | "wezterm" | "xterm-ghostty" | "xterm-kitty"
            | "foot" => {
                return Some(ColorSupport::TrueColor);
            }
            "linux" | "xterm" => {
                return Some(ColorSupport::Ansi16);
            }
            "dumb" => {
                return Some(ColorSupport::None);
            }
            _ => {}
        }
        if term.contains("256color") {
            return Some(ColorSupport::Ansi256);
        }
        if term.contains("color") || term.contains("ansi") {
            return Some(ColorSupport::Ansi16);
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
    fn new(value: Option<String>) -> Self {
        Self(value.map(|v| v.trim_ascii().to_lowercase()))
    }

    fn from_env(var: &str) -> Self {
        Self::new(env::var(var).ok())
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
