use std::io::stdout;

use termprofile::{DetectorSettings, TermProfile, TermVars};

fn main() {
    let mut vars = TermVars::from_env(&stdout(), DetectorSettings::with_dcs().unwrap());
    vars.overrides.force_color = "1".into();
    let profile = TermProfile::detect_with_vars(vars);
    println!("Profile with override: {profile:?}");
}
