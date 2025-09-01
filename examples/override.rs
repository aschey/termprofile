use std::io::stdout;

use termprofile::{DetectorSettings, TermProfile, TermVars};

fn main() {
    let mut vars = TermVars::from_env(DetectorSettings::default());
    vars.overrides.force_color = "1".into();
    let profile = TermProfile::detect_with_vars(&stdout(), vars);
    println!("Profile with override: {profile:?}");
}
