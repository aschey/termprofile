use std::io::stdout;

use termprofile::{TermProfile, TermVars};

fn main() {
    let mut vars = TermVars::from_env();
    vars.overrides.force_color = "1".into();
    let profile = TermProfile::detect_with_vars(&stdout(), vars);
    println!("Profile with override: {profile:?}");
}
