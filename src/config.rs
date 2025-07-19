use crate::countdown;
use crate::display;
use crate::font;
use clap::Parser;

#[derive(Clone, Copy, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Height of each digit tile.
    #[arg(short = 'H', long, default_value_t = 1)]
    pub height: u16,

    /// Width of each digit tile.
    #[arg(short = 'W', long, default_value_t = 2)]
    pub width: u16,

    /// Horizontal offset (x position).
    #[arg(short, default_value_t = 0)]
    pub x: u16,

    /// Vertical offset (y position).
    #[arg(short, default_value_t = 0)]
    pub y: u16,

    /// Use 12-hour clock format.
    #[arg(long = "12")]
    pub use12_hour: bool,

    /// Font style for digits (0: 5x7, 1: 3x5).
    #[arg(short, long, default_value_t = 0, value_parser = font::font_in_range)]
    pub font: usize,

    /// Color of the digit tiles.
    #[arg(short, long, default_value = "3")]
    pub color: display::Color8,

    #[command(subcommand)]
    pub countdown: Option<countdown::CountdownCommands>,
}
