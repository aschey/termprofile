use std::io::{IsTerminal, stdout};

use termprofile::{TermProfile, TermVar, TermVars};

fn main() {
    let mut vars = TermVars::from_env();
    let is_terminal = stdout().is_terminal();
    println!("is_terminal {is_terminal}");
    println!("{vars:#?}");
    vars.overrides.force_color = TermVar::new("1");
    let profile = TermProfile::detect_with_vars(&stdout(), vars);
    println!("Detected profile: {profile:?}");
}
