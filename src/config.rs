use crate::countdown;
use crate::display;
use crate::font;
use clap::Parser;

#[derive(Clone, Copy, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The height of each tile.
    #[arg(short = 'H', long, default_value_t = 1)]
    pub height: u16,

    /// The width of each tile.
    #[arg(short = 'W', long, default_value_t = 2)]
    pub width: u16,

    /// The offset of x
    #[arg(short, default_value_t = 0)]
    pub x: u16,

    /// The offset of y
    #[arg(short, default_value_t = 0)]
    pub y: u16,

    /// Change to 12-hour clock
    #[arg(long = "12")]
    pub use12_hour: bool,

    /// Choose the digit's font (0: 5x7, 1: 3x5)
    #[arg(short, long, default_value_t = 0, value_parser = font::font_in_range)]
    pub font: usize,

    /// The tile's color
    #[arg(short, long, default_value = "3")]
    pub color: display::Color8,

    #[command(subcommand)]
    pub countdown: Option<countdown::CountdownCommands>,
}
