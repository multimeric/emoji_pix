use std::str::FromStr;

use clap::{AppSettings, Clap};
use image::imageops::FilterType;
use image::{open, Rgb};
use num_traits::cast::ToPrimitive;
use scarlet::prelude::*;
use unicode_names2::name;

struct Filter(FilterType);

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

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Input image file to convert to emoji art
    input: String,

    /// Optional: the width of the output image in characters
    #[clap(short, long)]
    width: Option<u32>,

    /// Optional: the height of the output image in characters
    #[clap(short, long)]
    height: Option<u32>,

    /// Optional: if width or height are provided, the algorithm to use for resizing
    /// one of: CatmullRom, Gaussian, Lanczos3, Nearest, or Triangle
    #[clap(default_value = "Gaussian", short, long)]
    resize_filter: Filter,
}

/// Struct for storing colours in a colour palette, which links a physical colour to a unicode
/// character.
#[derive(Debug)]
struct TwemojiColour {
    name: String,
    colour: scarlet::color::RGBColor,
    char: char,
}

impl TwemojiColour {
    /// Create a new palette entry, using the unicode character name to name this entry
    fn new(char: char, colour: &str) -> TwemojiColour {
        TwemojiColour {
            name: name(char).unwrap().collect(),
            colour: RGBColor::from_hex_code(colour).unwrap(),
            char: char,
        }
    }
}

/// Utility function for converting between the image crate's Rgb type, and the scarlet crate's
/// RGBColor type
fn rgb_to_rgb(colour: &Rgb<u8>) -> RGBColor
{
    RGBColor::from((colour.0[0], colour.0[1], colour.0[2]))
    // RGBColor {
    //     r: colour.0[0].to_f64().unwrap(),
    //     g: colour.0[1].to_f64().unwrap(),
    //     b: colour.0[2].to_f64().unwrap(),
    // }
}

fn emojify(opts: Opts) -> String{
    // Associate emoji with hex colours
    let colours = [
        TwemojiColour::new('🟥', "DD2E44"),
        TwemojiColour::new('🟧', "F4900C"),
        TwemojiColour::new('🟨', "FDCB58"),
        TwemojiColour::new('🟩', "78B159"),
        TwemojiColour::new('🟦', "55ACEE"),
        TwemojiColour::new('🟪', "AA8ED6"),
        TwemojiColour::new('🟫', "C1694F"),
        TwemojiColour::new('⬛', "31373D"),
        TwemojiColour::new('⬜', "E6E7E8"),
    ];
    dbg!(&colours);
    let img = open(opts.input).unwrap();

    match (opts.width, opts.height) {
        // Handle the optional resizing
        (Some(w), Some(h)) => img.resize(w, h, opts.resize_filter.0),
        _ => img,
    }
        .to_rgb8()
        .rows()
        .flat_map(|img_row| {
            // For each row
            img_row
                .map(|img_pix| {
                    // For each cell, find the twemoji that is the closest in perceptual distance
                    // to this pixel
                    colours
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

fn main() {
    let opts: Opts = Opts::parse();
    let converted = emojify(opts);
    print!("{}", &converted);
}
