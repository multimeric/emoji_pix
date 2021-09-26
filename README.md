# Pixel Art Emoji
This is a simple command-line utility (and Rust crate!) for converting from a conventional image file (e.g. a PNG file) into a pixel-art version constructed with emoji.
One good use for this might be making pixel art in Discord, in posts or in your own profile.
For example, if you use `cargo run ferris.png --width 30 --height 30` on the Ferris (Rust's mascot) picture downloaded [from here](https://rustacean.net/assets/rustacean-flat-noshadow.png), you will get:
```
⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫⬛🟫🟫⬛🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫⬛⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬛⬛⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛
⬛⬛⬛⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛
⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛
⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛
⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬜⬛🟫🟫🟫⬜⬛🟥🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛
⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛🟫🟫🟫⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛
🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫🟫🟫🟫🟫🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫
🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟥🟫🟫🟫
⬛🟫🟫🟥🟥🟥🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫⬛🟥🟫🟫⬛
⬛⬛🟫🟫⬛⬛⬛🟫🟫🟫🟫🟫🟫🟫🟥🟥🟫🟫🟫🟫🟫🟫🟫🟫⬛⬛⬛🟫🟥⬛
⬛⬛⬛🟫🟫⬛⬛⬛🟫🟫🟫🟫⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫⬛⬛⬛⬛🟫🟫⬛⬛
⬛⬛⬛⬛🟫⬛⬛⬛⬛🟫🟫🟫🟫🟫⬛⬛🟫🟫🟫🟫🟫⬛⬛⬛⬛⬛🟫⬛⬛⬛
⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛🟫🟫🟫🟫⬛⬛🟫🟫🟫⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛
```
Regardless of where you post the resulting pixel art, I suggest using a monospaced font or `code` formatting when you post this, as it will generally give a more pleasing result.

## CLI Installation and Usage
I haven't yet set up binary builds or published to cargo, so for the moment you'll have to:
1. Make sure [Rust is installed](https://www.rust-lang.org/tools/install), as well as `git`
2. Clone the repo using `git clone https://github.com/multimeric/PixelArtEmoji.git`
3. `cd PixelArtEmoji`
4. To run the binary, use `cargo run [options]`. For example, `cargo run ferris.png --width 30 --height 30`.

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