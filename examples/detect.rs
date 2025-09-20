use std::io::stdout;

use termprofile::{DetectorSettings, TermProfile};

fn main() {
    let profile = TermProfile::detect(&stdout(), DetectorSettings::with_dcs().unwrap());
    println!("Detected profile: {profile:?}");
}
