use std::io::stdout;

use term_color_format::ColorSupport;

fn main() {
    let color_support = ColorSupport::detect(stdout());
    println!("{color_support:?}");
}
