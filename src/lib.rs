#![doc = include_str!("../README.md")]
use std::str::FromStr;

use clap::{AppSettings, Clap, crate_authors, crate_version};
use image::imageops::FilterType;
use image::{open, Rgb, GenericImageView};
use lazy_static::lazy_static;
use scarlet::prelude::*;
use unicode_names2::name;


/// A simple wrapper for the `image` crate's `FilterType`
#[derive(Debug)]
pub struct Filter(FilterType);

impl Default for Filter {
    fn default() -> Self {
        Filter(FilterType::Gaussian)
    }
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CatmullRom" => Ok(Filter(FilterType::CatmullRom)),
            "Gaussian" => Ok(Filter(FilterType::Gaussian)),
            "Lanczos3" => Ok(Filter(FilterType::Lanczos3)),
            "Nearest" => Ok(Filter(FilterType::Nearest)),
            "Triangle" => Ok(Filter(FilterType::Triangle)),
            _ => Err(String::from("Invalid filter type")),
        }
    }
}

/// Convert an image file into emoji pixel art
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp, version = crate_version!(), author = crate_authors!("\n"))]
#[derive(Debug, Default)]
pub struct Opts {
    /// Path to the image file to convert to emoji art. The emoji will be printed to stdout.
    pub input: String,

    /// Optional: the width of the output image in characters.
    /// Note: it only makes sense to specify height or width, as the aspect ratio is always conserved, and the other dimension is calculated as a proportion of this one.
    #[clap(short, long)]
    pub width: Option<u32>,

    /// Optional: the height of the output image in characters.
    /// Note: it only makes sense to specify height or width, as the aspect ratio is always conserved, and the other dimension is calculated as a proportion of this one.
    #[clap(short, long)]
    pub height: Option<u32>,

    /// Optional: if width or height are provided, the algorithm to use for resizing.
    /// One of: CatmullRom, Gaussian, Lanczos3, Nearest, or Triangle
    #[clap(default_value = "Gaussian", short, long)]
    pub resize_filter: Filter,
}

/// Struct for storing colours in a colour palette, which links a physical colour to a unicode
/// character.
#[derive(Debug)]
pub struct EmojiColour {
    /// A human-readable name for this colour
    pub name: String,
    /// A representation of the colour in RGB
    pub colour: RGBColor,
    /// The unicode character that can represent the associated colour
    pub char: char,
}

impl EmojiColour {
    /// Create a new palette entry, using the unicode character name to name this entry
    fn new(char: char, colour: &str) -> EmojiColour {
        EmojiColour {
            name: name(char).unwrap().collect(),
            colour: RGBColor::from_hex_code(colour).unwrap(),
            char: char,
        }
    }
}

/// Utility function for converting between the image crate's Rgb type, and the scarlet crate's
/// RGBColor type
fn rgb_to_rgb(colour: &Rgb<u8>) -> RGBColor {
    RGBColor::from((colour.0[0], colour.0[1], colour.0[2]))
}

lazy_static! {
    /// The colour palette available when using Twemoji squares as pixels
    pub static ref TWEMOJI: [EmojiColour; 9] = {[
        EmojiColour::new('🟥', "DD2E44"),
        EmojiColour::new('🟧', "F4900C"),
        EmojiColour::new('🟨', "FDCB58"),
        EmojiColour::new('🟩', "78B159"),
        EmojiColour::new('🟦', "55ACEE"),
        EmojiColour::new('🟪', "AA8ED6"),
        EmojiColour::new('🟫', "C1694F"),
        EmojiColour::new('⬛', "31373D"),
        EmojiColour::new('⬜', "E6E7E8"),
    ]};
}

/// Convert an image into a string of colour pixel emoji
/// # Examples
/// ```
/// use emoji_pix::{emojify, Opts};
/// emojify(Opts {
///     // This was downloaded from https://rustacean.net/assets/rustacean-flat-noshadow.png
///     input: String::from("ferris.png"),
///     height: Some(30),
///     ..Opts::default()
/// });
/// ```
/// This will return a string which looks like this:
///
///⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫⬛🟫🟫⬛🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
///
///⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛
///
///⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫⬛⬛⬛⬛⬛⬛⬛
///
///⬛⬛⬛⬛⬛⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛
///
///⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛
///
///⬛⬛⬛⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛
///
///⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛
///
///⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛
///
///⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
///
///⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
///
///⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬜⬛🟫🟫🟫⬜⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
///
///⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛🟫🟫🟫⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛
///
///🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫🟫🟫🟫🟫🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫
///
///🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫🟫🟫
///
///⬛🟫🟫🟥🟥🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛🟥🟫🟫⬛
///
///⬛⬛🟫🟫⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟥🟥🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛🟫🟥⬛
///
///⬛⬛⬛🟫🟫⬛⬛⬛🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫⬛⬛⬛⬛🟫🟫⬛⬛
///
///⬛⬛⬛⬛🟫⬛⬛⬛⬛🟫🟫🟫🟫🟫⬛⬛🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛🟫⬛⬛⬛
///
///⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫⬛⬛🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
///
pub fn emojify(opts: Opts) -> String {
    let img = open(opts.input).unwrap();
    let height = opts.height.unwrap_or(img.height());
    let width = opts.width.unwrap_or(img.width());

    match (opts.width, opts.height) {
        // If neither height nor width was provided, skip resizing entirely
        (None, None) => img,
        _ => img.resize(width, height, opts.resize_filter.0),
    }
    .to_rgb8()
    .rows()
    .flat_map(|img_row| {
        // For each row
        img_row
            .map(|img_pix| {
                // For each cell, find the twemoji that is the closest in perceptual distance
                // to this pixel
                TWEMOJI
                    .iter()
                    .map(|emoj| (emoj, emoj.colour.distance(&rgb_to_rgb(img_pix))))
                    .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
                    .unwrap()
                    .0
                    .char
            })
            // Append the newline after each row
            .chain(['\n'])
        // Find the index of the closest Twemoji colour
    })
    .collect()
}
