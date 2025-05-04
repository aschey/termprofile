use anstyle::{Ansi256Color, Color};
use palette::{FromColor, Lab, Srgb, color_difference::EuclideanDistance};

use crate::ColorSupport;

impl ColorSupport {
    pub fn adapt<C>(&self, color: C) -> Option<Color>
    where
        C: Into<Color>,
    {
        let color = color.into();
        if *self == Self::None {
            return None;
        }
        match color {
            Color::Ansi(_) => Some(color),
            Color::Ansi256(Ansi256Color(index)) => {
                if *self == Self::Ansi16 && index > 15 {
                    Some(ansi256_to_ansi(index))
                } else {
                    Some(color)
                }
            }
            Color::Rgb(rgb_color) => {
                if *self == Self::TrueColor {
                    Some(color)
                } else {
                    let ansi256_index = rgb_to_ansi256(rgb_color.r(), rgb_color.g(), rgb_color.g());
                    if *self == Self::Ansi256 {
                        Some(Color::Ansi256(ansi256_index.into()))
                    } else {
                        Some(ansi256_to_ansi(ansi256_index))
                    }
                }
            }
        }
    }
}

fn ansi256_to_lab(ansi256_index: u8) -> Lab {
    let srgb: Srgb<u8> = ANSI_HEX[ansi256_index as usize].parse().unwrap();
    Lab::from_color(srgb.into_linear())
}

fn ansi256_to_ansi(ansi256_index: u8) -> Color {
    let reference_lab = ansi256_to_lab(ansi256_index);
    let mut min_distance = f32::MAX;
    let mut closest_ansi = 0u8;
    for i in 0..16u8 {
        let lab = ansi256_to_lab(i);
        let distance = reference_lab.distance(lab);
        if distance < min_distance {
            closest_ansi = i;
            min_distance = distance;
        }
    }
    Color::Ansi256(closest_ansi.into())
}

fn value_to_color_index(value: u8) -> usize {
    if value < 48 {
        0
    } else if value < 115 {
        1
    } else {
        ((value - 35) / 40) as usize
    }
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let color = Srgb::new(r, g, b);
    let r = value_to_color_index(r);
    let g = value_to_color_index(g);
    let b = value_to_color_index(b);

    let color_index = (36 * r + 6 * g + b) as u8;
    let index_to_color_value: [u8; 5] = [0x0, 0x5f, 0x87, 0xaf, 0xff];

    let cr = index_to_color_value[r];
    let cg = index_to_color_value[g];
    let cb = index_to_color_value[b];

    let average = (r + g + b) / 3;
    let gray_index = if average > 238 {
        23
    } else {
        (average - 3) / 10
    };
    let gray_value = 8 + 10 * gray_index as u8;

    let color2 = Srgb::new(cr, cg, cb);
    let gray2 = Srgb::new(gray_value, gray_value, gray_value);

    let lab_color: Lab = Lab::from_color(color.into_linear());
    let color_distance = lab_color.distance(Lab::from_color(color2.into_linear()));
    let gray_distance = lab_color.distance(Lab::from_color(gray2.into_linear()));
    if color_distance <= gray_distance {
        16 + color_index
    } else {
        232 + gray_index as u8
    }
}

const ANSI_HEX: [&str; 256] = [
    "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080", "#c0c0c0",
    "#808080", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff",
    "#000000", "#00005f", "#000087", "#0000af", "#0000d7", "#0000ff", "#005f00", "#005f5f",
    "#005f87", "#005faf", "#005fd7", "#005fff", "#008700", "#00875f", "#008787", "#0087af",
    "#0087d7", "#0087ff", "#00af00", "#00af5f", "#00af87", "#00afaf", "#00afd7", "#00afff",
    "#00d700", "#00d75f", "#00d787", "#00d7af", "#00d7d7", "#00d7ff", "#00ff00", "#00ff5f",
    "#00ff87", "#00ffaf", "#00ffd7", "#00ffff", "#5f0000", "#5f005f", "#5f0087", "#5f00af",
    "#5f00d7", "#5f00ff", "#5f5f00", "#5f5f5f", "#5f5f87", "#5f5faf", "#5f5fd7", "#5f5fff",
    "#5f8700", "#5f875f", "#5f8787", "#5f87af", "#5f87d7", "#5f87ff", "#5faf00", "#5faf5f",
    "#5faf87", "#5fafaf", "#5fafd7", "#5fafff", "#5fd700", "#5fd75f", "#5fd787", "#5fd7af",
    "#5fd7d7", "#5fd7ff", "#5fff00", "#5fff5f", "#5fff87", "#5fffaf", "#5fffd7", "#5fffff",
    "#870000", "#87005f", "#870087", "#8700af", "#8700d7", "#8700ff", "#875f00", "#875f5f",
    "#875f87", "#875faf", "#875fd7", "#875fff", "#878700", "#87875f", "#878787", "#8787af",
    "#8787d7", "#8787ff", "#87af00", "#87af5f", "#87af87", "#87afaf", "#87afd7", "#87afff",
    "#87d700", "#87d75f", "#87d787", "#87d7af", "#87d7d7", "#87d7ff", "#87ff00", "#87ff5f",
    "#87ff87", "#87ffaf", "#87ffd7", "#87ffff", "#af0000", "#af005f", "#af0087", "#af00af",
    "#af00d7", "#af00ff", "#af5f00", "#af5f5f", "#af5f87", "#af5faf", "#af5fd7", "#af5fff",
    "#af8700", "#af875f", "#af8787", "#af87af", "#af87d7", "#af87ff", "#afaf00", "#afaf5f",
    "#afaf87", "#afafaf", "#afafd7", "#afafff", "#afd700", "#afd75f", "#afd787", "#afd7af",
    "#afd7d7", "#afd7ff", "#afff00", "#afff5f", "#afff87", "#afffaf", "#afffd7", "#afffff",
    "#d70000", "#d7005f", "#d70087", "#d700af", "#d700d7", "#d700ff", "#d75f00", "#d75f5f",
    "#d75f87", "#d75faf", "#d75fd7", "#d75fff", "#d78700", "#d7875f", "#d78787", "#d787af",
    "#d787d7", "#d787ff", "#d7af00", "#d7af5f", "#d7af87", "#d7afaf", "#d7afd7", "#d7afff",
    "#d7d700", "#d7d75f", "#d7d787", "#d7d7af", "#d7d7d7", "#d7d7ff", "#d7ff00", "#d7ff5f",
    "#d7ff87", "#d7ffaf", "#d7ffd7", "#d7ffff", "#ff0000", "#ff005f", "#ff0087", "#ff00af",
    "#ff00d7", "#ff00ff", "#ff5f00", "#ff5f5f", "#ff5f87", "#ff5faf", "#ff5fd7", "#ff5fff",
    "#ff8700", "#ff875f", "#ff8787", "#ff87af", "#ff87d7", "#ff87ff", "#ffaf00", "#ffaf5f",
    "#ffaf87", "#ffafaf", "#ffafd7", "#ffafff", "#ffd700", "#ffd75f", "#ffd787", "#ffd7af",
    "#ffd7d7", "#ffd7ff", "#ffff00", "#ffff5f", "#ffff87", "#ffffaf", "#ffffd7", "#ffffff",
    "#080808", "#121212", "#1c1c1c", "#262626", "#303030", "#3a3a3a", "#444444", "#4e4e4e",
    "#585858", "#626262", "#6c6c6c", "#767676", "#808080", "#8a8a8a", "#949494", "#9e9e9e",
    "#a8a8a8", "#b2b2b2", "#bcbcbc", "#c6c6c6", "#d0d0d0", "#dadada", "#e4e4e4", "#eeeeee",
];
