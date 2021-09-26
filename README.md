# Pixel Art Emoji

This is a simple command-line utility (and Rust crate!) for converting from a conventional image file (e.g. a PNG file) into a pixel-art version constructed with emoji.
One good use for this might be making pixel art in Discord, in posts or in your own profile.

## Installation

I haven't yet set up binary builds or published to cargo, so for the moment you'll have to:

1. Make sure [Rust is installed](https://www.rust-lang.org/tools/install), as well as `git`
2. Clone the repo using `git clone https://github.com/multimeric/PixelArtEmoji.git`

## CLI Usage
To run the binary, use `cargo run [options]`. For example, `cargo run ferris.png --width 30 --height 30`.

The full CLI options are obtained using `cargo run -- --help`:
```
USAGE:
    pixel_art_emoji [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Path to the image file to convert to emoji art

FLAGS:
        --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -h, --height <HEIGHT>
            Optional: the height of the output image in characters

    -r, --resize-filter <RESIZE_FILTER>
            Optional: if width or height are provided, the algorithm to use for resizing one of:
            CatmullRom, Gaussian, Lanczos3, Nearest, or Triangle [default: Gaussian]

    -w, --width <WIDTH>
            Optional: the width of the output image in characters
```

## Crate Usage

1. Install the crate by adding `pixel_art_emoji = { git = "https://github.com/multimeric/PixelArtEmoji.git" }` to your `Cargo.toml`
2. Here is some example usage:
```rust
use PixelArtEmoji::{emojify, Opts};

emojify(Opts {
    input: String::from("ferris.png"),
    height: Some(30),
    width: Some(30),
    ..Opts::default()
});
```