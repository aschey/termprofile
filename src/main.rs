use std::io::stdout;

use termprofile::TermProfile;

fn main() {
    let color_support = TermProfile::detect(&stdout());
    println!("{color_support:?}");
}
