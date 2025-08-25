use std::io::stdout;

use termprofile::{TermProfile, TermVar, TermVars};

fn main() {
    let mut vars = TermVars::from_env();
    vars.overrides.force_color = TermVar::new("1");
    let profile = TermProfile::detect_with_vars(&stdout(), vars);
    println!("Profile with override: {profile:?}");
}
