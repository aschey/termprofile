use std::collections::{BTreeMap, HashMap};
use std::env;
use std::io::{self, Read};
use std::process::{Command, Stdio};

use crate::TermProfile;

// Rules
// https://bixense.com/clicolors/
// https://no-color.org/

/// Custom trait for determining if something is a terminal. This mirrors the trait from
/// [`std::io`], but that one is sealed and not able to be implemented on custom types.
pub trait IsTerminal {
    /// Returns true if the current object is a terminal.
    fn is_terminal(&self) -> bool;
}

impl<T> IsTerminal for T
where
    T: io::IsTerminal,
{
    fn is_terminal(&self) -> bool {
        self.is_terminal()
    }
}

/// Trait for implementing custom environment variable sources. This is useful if you want to
/// source environment variables from somewhere other than [`std::env::var`].
pub trait EnvVarSource {
    /// Look up the variable in the source.
    fn var(&self, key: &str) -> Option<String>;
}

/// Source that pulls environment variables from [`std::env::var`].
pub struct Env;

impl EnvVarSource for Env {
    fn var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
}

impl EnvVarSource for HashMap<String, String> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).cloned()
    }
}

impl EnvVarSource for HashMap<&str, &str> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).map(ToString::to_string)
    }
}

impl EnvVarSource for BTreeMap<String, String> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).cloned()
    }
}

impl EnvVarSource for BTreeMap<&str, &str> {
    fn var(&self, key: &str) -> Option<String> {
        self.get(key).map(ToString::to_string)
    }
}

/// Collection of variables used to determine color support.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TermVars {
    /// Variables for overriding terminal behavior.
    pub overrides: OverrideVars,
    /// Metadata about the terminal itself.
    pub meta: TermMetaVars,
    /// Special cases for specific platforms.
    pub special: SpecialVars,
    /// tmux-specific variables.
    pub tmux: TmuxVars,
    /// Windows information.
    pub windows: WindowsVars,
    /// Information sourced from terminfo.
    pub terminfo: TerminfoVars,
}

/// Variables for overriding terminal behavior.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct OverrideVars {
    /// `FORCE_COLOR` environment variable - forces color support.
    pub force_color: TermVar,
    /// `CLICOLOR_FORCE` environment variable - forces color support.
    pub clicolor_force: TermVar,
    /// `CLICOLOR` environment variable - enables color support if the output is a terminal.
    pub clicolor: TermVar,
    /// `NO_COLOR` environment variable - disables color support.
    pub no_color: TermVar,
    /// `TTY_FORCE` environment variable - forces the output to behave like a TTY.
    pub tty_force: TermVar,
}

/// Metadata about the terminal itself.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TermMetaVars {
    /// Whether the current environment is a terminal.
    pub is_terminal: bool,
    /// `TERM` environment variable - current terminal name.;
    pub term: TermVar,
    /// `COLORTERM` environment variable - enables true color support.
    pub colorterm: TermVar,
    /// `TERM_PROGRAM` environment variable - current terminal program name.
    pub term_program: TermVar,
    /// `TERM_PROGRAM_VERSION` environment variable - current terminal program version.
    pub term_program_version: TermVar,
    /// Whether the DCS query for true color support returned true.
    pub dcs_response: bool,
}

/// Windows information.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct WindowsVars {
    /// `ANSICON` environment variable - set by the ANSICON program, if enabled.
    pub ansicon: TermVar,
    /// `ANSICON_VER` environment variable - set by the ANSICON program, if enabled.
    pub ansicon_ver: TermVar,
    /// Windows build number.
    pub build_number: u64,
    /// Windows OS version.
    pub os_version: u64,
    /// True if the current system is Windows.
    pub is_windows: bool,
    // Note: Windows terminal developers recommend against using WT_SESSION
    // https://github.com/Textualize/rich/issues/140
}

/// Special cases for specific platforms.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SpecialVars {
    /// `GOOGLE_CLOUD_SHELL` environment variable.
    pub google_cloud_shell: TermVar,
    /// `GITHUB_ACTIONS` environment variable.
    pub github_actions: TermVar,
    /// `GITEA_ACTIONS` environment variable.
    pub gitea_actions: TermVar,
    /// `CIRCLECI` environment variable.
    pub circleci: TermVar,
    /// `TRAVIS` environment variable.
    pub travis: TermVar,
    /// `APPVEYOR` environment variable.
    pub appveyor: TermVar,
    /// `GITLAB_CI` environment variable.
    pub gitlab_ci: TermVar,
    /// `BUILDKITE` environment variable.
    pub buildkite: TermVar,
    /// `DRONE` environment variable.
    pub drone: TermVar,
    /// `TEAMCITY_VERSION` environment variable.
    pub teamcity_version: TermVar,
    /// `TF_BUILD` environment variable.
    pub tf_build: TermVar,
    /// `AGENT_NAME` environment variable.
    pub agent_name: TermVar,
    /// `CIRRUS_CI` environment variable.
    pub cirrus_ci: TermVar,
    /// `CI_NAME` environment variable.
    pub ci_name: TermVar,
    /// `ConEmuANSI` environment variable.
    pub con_emu_ansi: TermVar,
    /// `CI` environment variable.
    pub ci: TermVar,
}

/// tmux-specific variables.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TmuxVars {
    /// Output from the `tmux info` command.
    pub tmux_info: String,
    /// `TMUX` environment variable - set if running in tmux.
    pub tmux: TermVar,
}

/// Information sourced from terminfo.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct TerminfoVars {
    /// Max colors from the terminfo entry.
    pub max_colors: Option<i32>,
    /// Truecolor terminfo extension, this is non-standard.
    pub truecolor: Option<bool>,
}

pub(crate) const TERM: &str = "TERM";
pub(crate) const TERM_PROGRAM: &str = "TERM_PROGRAM";
pub(crate) const TERM_PROGRAM_VERSION: &str = "TERM_PROGRAM_VERSION";
pub(crate) const COLORTERM: &str = "COLORTERM";
pub(crate) const CLICOLOR_FORCE: &str = "CLICOLOR_FORCE";
pub(crate) const CLICOLOR: &str = "CLICOLOR";
pub(crate) const FORCE_COLOR: &str = "FORCE_COLOR";
pub(crate) const NO_COLOR: &str = "NO_COLOR";
pub(crate) const TTY_FORCE: &str = "TTY_FORCE";

pub(crate) const SCREEN: &str = "screen";
pub(crate) const TMUX: &str = "tmux";
pub(crate) const DUMB: &str = "dumb";
pub(crate) const TC: &str = "Tc";
pub(crate) const RGB: &str = "RGB";

#[cfg(feature = "terminfo")]
fn get_ext_bool(info: &termini::TermInfo, name: &str) -> Option<bool> {
    info.extended_cap(name).map(|c| c == termini::Value::True)
}

impl TerminfoVars {
    #[cfg(feature = "terminfo")]
    fn from_env<S, Q>(source: &S, settings: &DetectorSettings<Q>) -> Self
    where
        S: EnvVarSource,
        Q: QueryTerminal,
    {
        let term = source.var(TERM).unwrap_or_default();
        if settings.enable_terminfo
            && let Ok(info) = termini::TermInfo::from_name(&term)
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
    fn from_env<S, Q>(_source: &S, _settings: &DetectorSettings<Q>) -> Self
    where
        S: EnvVarSource,
        Q: QueryTerminal,
    {
        Self::default()
    }
}

impl TermVars {
    /// Load the terminal variables from the current environment.
    pub fn from_env<Q, T>(out: &T, settings: DetectorSettings<Q>) -> Self
    where
        T: IsTerminal,
        Q: QueryTerminal,
    {
        Self::from_source(&Env, out, settings)
    }

    /// Load the terminal variables from the given source.
    pub fn from_source<S, Q, T>(source: &S, out: &T, mut settings: DetectorSettings<Q>) -> Self
    where
        S: EnvVarSource,
        T: IsTerminal,
        Q: QueryTerminal,
    {
        Self {
            meta: TermMetaVars::from_source(source, out, &mut settings),
            overrides: OverrideVars::from_source(source),
            special: SpecialVars::from_source(source),
            tmux: TmuxVars::from_source(source, &settings),
            terminfo: TerminfoVars::from_env(source, &settings),
            windows: WindowsVars::from_source(source),
        }
    }
}

impl TermMetaVars {
    /// Load the variables from the given source.
    pub fn from_source<S, Q, T>(
        source: &S,
        out: &T,
        #[cfg_attr(not(feature = "query-detect"), expect(unused))] settings: &mut DetectorSettings<
            Q,
        >,
    ) -> Self
    where
        S: EnvVarSource,
        T: IsTerminal,
        Q: QueryTerminal,
    {
        let term = TermVar::from_source(source, TERM);
        #[cfg(feature = "query-detect")]
        let dcs_response = if settings.enable_query {
            crate::query_detect(
                source,
                out,
                &mut settings.query_terminal,
                term.0.as_deref().unwrap_or_default(),
            )
            .unwrap_or(false)
        } else {
            false
        };
        #[cfg(not(feature = "query-detect"))]
        let dcs_response = false;
        Self {
            is_terminal: out.is_terminal(),
            term,
            colorterm: TermVar::from_source(source, COLORTERM),
            term_program: TermVar::from_source(source, TERM_PROGRAM),
            term_program_version: TermVar::from_source(source, TERM_PROGRAM_VERSION),
            dcs_response,
        }
    }

    fn is_dumb(&self) -> bool {
        self.term.0.as_deref() == Some(DUMB)
    }
}

pub(crate) fn prefix_or_equal(var: &str, compare: &str) -> bool {
    var == compare
        || var.starts_with(&format!("{compare}-"))
        || var.starts_with(&format!("{compare}."))
}

/// RGB Color.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// Red component.
    pub red: u8,
    /// Green component.
    pub green: u8,
    /// Blue component.
    pub blue: u8,
}

/// Event returned by a DCS query.
pub enum DcsEvent {
    /// Background color queried from the terminal.
    BackgroundColor(Rgb),
    /// Device attributes returned by the terminal - used to signal the end of the query.
    DeviceAttributes,
    /// A miscellaneous event.
    Other,
    /// Timed reading the next event.
    TimedOut,
}

/// Trait for defining a terminal source that can be queried.
pub trait QueryTerminal: io::Write {
    /// Set up the terminal by enabling raw mode.
    fn setup(&mut self) -> io::Result<()>;
    /// Clean up the terminal by disabling raw mode.
    fn cleanup(&mut self) -> io::Result<()>;
    /// Read the next event from the terminal.
    fn read_event(&mut self) -> io::Result<DcsEvent>;
}

/// Default implementation for [`QueryTerminal`] that doesn't query anything.
pub struct NoTerminal;

impl io::Write for NoTerminal {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl IsTerminal for NoTerminal {
    fn is_terminal(&self) -> bool {
        false
    }
}

impl QueryTerminal for NoTerminal {
    fn setup(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn read_event(&mut self) -> io::Result<DcsEvent> {
        Ok(DcsEvent::TimedOut)
    }
}

impl OverrideVars {
    /// Load the variables from the given source.
    pub fn from_source<S>(source: &S) -> Self
    where
        S: EnvVarSource,
    {
        Self {
            no_color: TermVar::from_source(source, NO_COLOR),
            force_color: TermVar::from_source(source, FORCE_COLOR),
            clicolor: TermVar::from_source(source, CLICOLOR),
            clicolor_force: TermVar::from_source(source, CLICOLOR_FORCE),
            tty_force: TermVar::from_source(source, TTY_FORCE),
        }
    }
}

impl SpecialVars {
    /// Load the variables from the given source.
    pub fn from_source<S>(source: &S) -> Self
    where
        S: EnvVarSource,
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
    /// Load the variables from the given source.
    pub fn from_source<S, T>(source: &S, settings: &DetectorSettings<T>) -> Self
    where
        S: EnvVarSource,
        T: QueryTerminal,
    {
        Self::try_from_source(source, settings).unwrap_or_default()
    }

    /// Try to load the variables from the given source.
    pub fn try_from_source<S, T>(
        source: &S,
        settings: &DetectorSettings<T>,
    ) -> Result<Self, io::Error>
    where
        S: EnvVarSource,
        T: QueryTerminal,
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
    /// Load the variables from the given source.
    #[cfg(all(windows, feature = "windows-version"))]
    pub fn from_source<S>(source: &S) -> Self
    where
        S: EnvVarSource,
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

    /// Load the variables from the given source.
    #[cfg(not(all(windows, feature = "windows-version")))]
    pub fn from_source<S>(source: &S) -> Self
    where
        S: EnvVarSource,
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

/// Settings for enabling extra detector features.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectorSettings<T> {
    pub(crate) enable_query: bool,
    pub(crate) enable_terminfo: bool,
    pub(crate) enable_tmux_info: bool,
    pub(crate) query_terminal: T,
}

impl Default for DetectorSettings<NoTerminal> {
    fn default() -> Self {
        Self {
            enable_query: false,
            enable_terminfo: true,
            enable_tmux_info: true,
            query_terminal: NoTerminal,
        }
    }
}

impl DetectorSettings<NoTerminal> {
    /// Create a new [`DetectorSettings`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> DetectorSettings<T> {
    /// Enable or disable querying the terminfo database.
    #[cfg(feature = "terminfo")]
    pub fn enable_terminfo(mut self, enable_terminfo: bool) -> Self {
        self.enable_terminfo = enable_terminfo;
        self
    }

    /// Enable or disable querying the tmux information if tmux is used.
    pub fn enable_tmux_info(mut self, enable_tmux_info: bool) -> Self {
        self.enable_tmux_info = enable_tmux_info;
        self
    }
}

impl TermProfile {
    /// Detect the output's profile information.
    pub fn detect<T, Q>(output: &T, settings: DetectorSettings<Q>) -> Self
    where
        T: IsTerminal,
        Q: QueryTerminal,
    {
        Self::detect_with_vars(TermVars::from_env(output, settings))
    }

    /// Detect the output's profile information using the given variables as the source.
    pub fn detect_with_vars(vars: TermVars) -> Self {
        let detector = Detector { vars };
        let profile = detector.detect_tty();
        if let Some(env) = detector.detect_no_color()
            && profile > Self::NoTty
        {
            return env;
        }
        if let Some(env) = detector.detect_force_color() {
            return env;
        }
        if detector.vars.meta.dcs_response {
            return Self::TrueColor;
        }
        if let Some(env) = detector.detect_special_cases() {
            return env;
        }
        if profile == Self::NoTty {
            return profile;
        }

        detector.detect_term_vars()
    }
}

struct Detector {
    vars: TermVars,
}

impl Detector {
    fn detect_tty(&self) -> TermProfile {
        if (!self.vars.overrides.tty_force.is_truthy() && !self.vars.meta.is_terminal)
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
        let mut profile = None;
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

        if term.is_empty() && !self.vars.overrides.clicolor.is_truthy() {
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

/// Represents an environment variable.
#[derive(Clone, Debug, Default)]
pub struct TermVar(Option<String>);

impl TermVar {
    /// Create a new [`TermVar`]. This will normalize the supplied string by trimming whitespace
    /// and converting it to lowercase.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_internal(Some(value.into()))
    }

    fn new_internal(value: Option<String>) -> Self {
        Self(value.map(|v| v.trim_ascii().to_lowercase()))
    }

    /// Create a new [`TermVar`] by looking up the key from the given source.
    pub fn from_source<S>(source: &S, var: &str) -> Self
    where
        S: EnvVarSource,
    {
        Self(source.var(var).map(|v| v.trim_ascii().to_lowercase()))
    }

    pub(crate) fn is_truthy(&self) -> bool {
        self.0
            .as_deref()
            .map(|v| v == "1" || v == "true" || v == "yes" || v == "on")
            .unwrap_or(false)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.as_deref().map(str::is_empty).unwrap_or(true)
    }

    fn or(&self, other: &Self) -> Self {
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
