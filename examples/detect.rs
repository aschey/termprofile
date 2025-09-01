use std::io::stdout;

use termprofile::{DetectorSettings, TermProfile};

fn main() {
    let profile = TermProfile::detect(&stdout(), DetectorSettings::default());
    println!("Detected profile: {profile:?}");
}
