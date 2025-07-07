use std::io::stdout;

use termprofile::TermProfile;

fn main() {
    let profile = TermProfile::detect(&stdout());
    println!("{profile:?}");
}
