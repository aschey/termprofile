use std::{env, io::IsTerminal};

use crate::ColorSupport;

impl ColorSupport {
    pub fn detect<T>(output: T) -> Self
    where
        T: IsTerminal,
    {
        if let Some(env) = Self::detect_no_color()
            .or_else(Self::detect_force_color)
            .or_else(Self::detect_special_cases)
        {
            return env;
        }
        if !output.is_terminal() {
            return Self::None;
        }
        if let Some(env) = Self::detect_term_vars() {
            return env;
        }
        #[cfg(all(windows, feature = "windows-version"))]
        if let Some(env) = Self::detect_windows_version() {
            return env;
        }
        ColorSupport::None
    }

    fn detect_no_color() -> Option<Self> {
        if is_env_var_truthy("NO_COLOR") {
            Some(Self::None)
        } else {
            None
        }
    }

    fn detect_force_color() -> Option<Self> {
        let force_color =
            env_var_normalized("FORCE_COLOR").or_else(|| env_var_normalized("CLICOLOR_FORCE"))?;

        if is_value_truthy(&force_color) {
            if let Some(term) = Self::detect_term_vars() {
                if matches!(term, Self::Ansi256 | Self::TrueColor) {
                    // If the terminal reports it has better color support, don't force it to use
                    // basic ANSI.
                    return Some(term);
                }
            }
            return Some(Self::Ansi16);
        }

        let level: u32 = force_color.parse().ok()?;
        match level {
            0 => Some(Self::None),
            1 => Some(Self::Ansi16),
            2 => Some(Self::Ansi256),
            3 => Some(Self::TrueColor),
            _ => None,
        }
    }

    fn detect_special_cases() -> Option<Self> {
        const TRUECOLOR_PLATFORMS: [&str; 4] = [
            "GOOGLE_CLOUD_SHELL",
            "GITHUB_ACTIONS",
            "GITEA_ACTIONS",
            "CIRCLECI",
        ];
        const ANSI_PLATFORMS: [&str; 6] = [
            "TRAVIS",
            "APPVEYOR",
            "GITLAB_CI",
            "BUILDKITE",
            "DRONE",
            "TEAMCITY_VERSION",
        ];

        if TRUECOLOR_PLATFORMS.iter().any(|p| is_env_var_non_empty(p)) {
            return Some(Self::TrueColor);
        }

        if ANSI_PLATFORMS.iter().any(|p| is_env_var_non_empty(p)) {
            return Some(Self::Ansi16);
        }

        // Azure pipelines
        if is_env_var_non_empty("TF_BUILD") && is_env_var_non_empty("AGENT_NAME") {
            return Some(Self::Ansi16);
        }
        if env_var_normalized("CI_NAME").as_deref() == Some("codeship") {
            return Some(Self::Ansi16);
        }
        if env_var_normalized("ConEmuANSI").as_deref() == Some("ON") {
            return Some(Self::TrueColor);
        }

        None
    }

    fn detect_term_vars() -> Option<Self> {
        let colorterm = env_var_normalized("COLORTERM").unwrap_or_default();
        let term: String = env_var_normalized("TERM").unwrap_or_default();
        let term_program = env_var_normalized("TERM_PROGRAM").unwrap_or_default();

        if matches!(colorterm.as_str(), "24bit" | "truecolor") {
            // New versions of screen do support truecolor, but it must be enabled explicitly and
            // there doesn't appear to be an easy way to detect this.
            if term.starts_with("screen") && term_program != "tmux" {
                return Some(Self::Ansi256);
            }
            return Some(Self::TrueColor);
        }

        if is_value_truthy(&colorterm) {
            return Some(Self::Ansi256);
        }

        match term_program.as_str() {
            "mintty" => {
                // Supported as of 2015: https://github.com/mintty/mintty/commit/8e1f4c260b5e1b3311caf10e826d87c85b3c9433
                return Some(Self::TrueColor);
            }
            "iterm.app" => {
                let term_program_version = env_var_normalized("TERM_PROGRAM_VERSION")
                    .unwrap_or_default()
                    .split(".")
                    .next()
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(0);
                if term_program_version >= 3 {
                    return Some(Self::TrueColor);
                } else {
                    return Some(Self::Ansi256);
                }
            }
            "apple_terminal" => return Some(Self::Ansi256),
            _ => {}
        }

        match term.as_str() {
            "alacritty" | "contour" | "rio" | "wezterm" | "xterm-ghostty" | "xterm-kitty"
            | "foot" => {
                return Some(Self::TrueColor);
            }
            "linux" | "xterm" => {
                return Some(Self::Ansi16);
            }
            "dumb" => {
                return Some(Self::None);
            }
            _ => {}
        }
        if term.contains("256color") {
            return Some(Self::Ansi256);
        }
        if term.contains("color") || term.contains("ansi") {
            return Some(Self::Ansi16);
        }

        None
    }

    #[cfg(all(windows, feature = "windows-version"))]
    fn detect_windows_version() -> Option<Self> {
        use os_info::Version;
        let info = os_info::get();
        let windows_version = info.version();
        if let Version::Semantic(os_version, _, build_number) = windows_version {
            if *build_number < 10586 || *os_version < 10 {
                if is_env_var_non_empty("ANSICON") {
                    let ansicon_version = env_var_normalized("ANSICON_VER")
                        .unwrap_or_default()
                        .parse::<u32>();
                    if ansicon_version.map(|v| v >= 181).unwrap_or(false) {
                        return Some(Self::Ansi256);
                    } else {
                        return Some(Self::Ansi16);
                    }
                }

                return Some(Self::None);
            }

            if *build_number < 14931 {
                return Some(Self::Ansi256);
            }

            Some(Self::TrueColor)
        } else {
            None
        }
    }
}

fn env_var_normalized(var: &str) -> Option<String> {
    Some(env::var(var).ok()?.trim_ascii().to_lowercase())
}

fn is_env_var_truthy(var: &str) -> bool {
    env_var_normalized(var)
        .map(|v| is_value_truthy(&v))
        .unwrap_or(false)
}

fn is_env_var_non_empty(var: &str) -> bool {
    env_var_normalized(var)
        .map(|v| !v.is_empty())
        .unwrap_or(false)
}

fn is_value_truthy(val: &str) -> bool {
    val == "1" || val == "true" || val == "yes"
}
