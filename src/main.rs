use std::io::stdout;

use term_color_adapter::ColorSupport;

fn main() {
    let color_support = ColorSupport::detect(&stdout());
    println!("{color_support:?}");
}
