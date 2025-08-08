mod ansi_256_to_16;
mod ansi_256_to_rgb;
mod color;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, RwLock};

use ansi_256_to_16::ANSI_256_TO_16;
use ansi_256_to_rgb::ANSI_256_TO_RGB;
use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor, Style};
pub use color::*;
use palette::Srgb;

use crate::TermProfile;

impl TermProfile {
    pub fn adapt_color<C>(&self, color: C) -> Option<Color>
    where
        C: Into<Color>,
    {
        let color: Color = color.into();
        if *self < Self::Ansi16 {
            return None;
        }
        match color {
            Color::Ansi(_) => Some(color),
            Color::Ansi256(Ansi256Color(index)) => {
                if *self >= Self::Ansi256 {
                    Some(color)
                } else {
                    Some(ansi256_to_ansi(index).into())
                }
            }
            Color::Rgb(rgb_color) => {
                if *self == Self::TrueColor {
                    Some(color)
                } else {
                    let ansi256_index = rgb_to_ansi256(rgb_color);
                    if *self == Self::Ansi256 {
                        Some(ansi256_index.into())
                    } else {
                        Some(ansi256_to_ansi(ansi256_index).into())
                    }
                }
            }
        }
    }

    pub fn adapt_style<S>(&self, style: S) -> Style
    where
        S: Into<Style>,
    {
        if *self == Self::NoTty {
            return Style::new();
        }
        let mut style: Style = style.into();

        if let Some(color) = style.get_fg_color() {
            style = style.fg_color(self.adapt_color(color));
        }
        if let Some(color) = style.get_bg_color() {
            style = style.bg_color(self.adapt_color(color));
        }
        if let Some(color) = style.get_underline_color() {
            style = style.underline_color(self.adapt_color(color));
        }
        style
    }
}

pub fn ansi256_to_ansi(ansi256_index: u8) -> AnsiColor {
    match ANSI_256_TO_16[&ansi256_index] {
        0 => AnsiColor::Black,
        1 => AnsiColor::Red,
        2 => AnsiColor::Green,
        3 => AnsiColor::Yellow,
        4 => AnsiColor::Blue,
        5 => AnsiColor::Magenta,
        6 => AnsiColor::Cyan,
        7 => AnsiColor::White,
        8 => AnsiColor::BrightBlack,
        9 => AnsiColor::BrightRed,
        10 => AnsiColor::BrightGreen,
        11 => AnsiColor::BrightYellow,
        12 => AnsiColor::BrightBlue,
        13 => AnsiColor::BrightMagenta,
        14 => AnsiColor::BrightCyan,
        15 => AnsiColor::BrightWhite,
        _ => unreachable!(),
    }
}

fn get_color_index<const N: usize>(val: u8, breakpoints: [u8; N]) -> usize {
    breakpoints.iter().position(|p| val < *p).unwrap_or(N)
}

fn red_color_index(val: u8) -> usize {
    get_color_index(val, [49, 116, 156, 196, 236])
}

fn green_color_index(val: u8) -> usize {
    get_color_index(val, [48, 116, 156, 196, 236])
}

fn blue_color_index(val: u8) -> usize {
    get_color_index(val, [48, 116, 156, 196, 236])
}

const COLOR_INTERVALS: [u8; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

static COLOR_CACHE: LazyLock<RwLock<HashMap<RgbColor, u8>>> = LazyLock::new(Default::default);

static CACHE_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_color_cache_enabled(enabled: bool) {
    CACHE_ENABLED.store(enabled, Ordering::SeqCst);
}

pub fn rgb_to_ansi256(color: RgbColor) -> u8 {
    let cache_enabled = CACHE_ENABLED.load(Ordering::Relaxed);
    if cache_enabled && let Some(cached) = COLOR_CACHE.read().unwrap().get(&color) {
        return *cached;
    }
    let converted = rgb_to_ansi256_inner(color);
    if cache_enabled {
        COLOR_CACHE.write().unwrap().insert(color, converted);
    }
    converted
}

fn rgb_to_ansi256_inner(color: RgbColor) -> u8 {
    let srgb = Srgb::new(color.r(), color.g(), color.b());

    let qr = red_color_index(srgb.red);
    let qg = green_color_index(srgb.green);
    let qb = blue_color_index(srgb.blue);
    let cr = COLOR_INTERVALS[qr];
    let cg = COLOR_INTERVALS[qg];
    let cb = COLOR_INTERVALS[qb];
    let color_index = (36 * qr + 6 * qg + qb + 16) as u8;

    if cr == srgb.red && cg == srgb.green && cb == srgb.blue {
        COLOR_CACHE.write().unwrap().insert(color, color_index);
        return color_index;
    }
    let average = ((srgb.red as u32 + srgb.green as u32 + srgb.blue as u32) / 3) as u8;
    let gray_index = if average > 238 {
        23
    } else {
        (average - 3) / 10
    };
    let gray_value = 8 + 10 * gray_index;

    let color2 = Srgb::new(cr, cg, cb);
    let gray2 = Srgb::new(gray_value, gray_value, gray_value);

    let color_distance = distance_squared(srgb, color2);
    let gray_distance = distance_squared(srgb, gray2);
    let index = if color_distance <= gray_distance {
        color_index
    } else {
        232 + gray_index
    };
    index
}

pub fn ansi256_to_rgb(ansi: Ansi256Color) -> RgbColor {
    ANSI_256_TO_RGB[ansi.0 as usize]
}

// after trying a bunch of methods, this seems to get the best results on average - https://stackoverflow.com/a/9085524
fn distance_squared(rgb1: Srgb<u8>, rgb2: Srgb<u8>) -> u32 {
    let r_mean = (rgb1.red as i32 + rgb2.red as i32) / 2;
    let r = (rgb1.red as i32) - (rgb2.red as i32);
    let g = (rgb1.green as i32) - (rgb2.green as i32);
    let b = (rgb1.blue as i32) - (rgb2.blue as i32);
    ((((512 + r_mean) * r * r) >> 8) + 4 * g * g + (((767 - r_mean) * b * b) >> 8)) as u32
}

#[cfg(test)]
#[path = "./convert_test.rs"]
mod convert_test;
