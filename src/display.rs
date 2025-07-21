use anyhow::Result;
use std::fmt::Write as _;
use std::io::{Stdout, Write};
use std::str::FromStr;

use crate::font;

// #[derive(Clone, Copy)]
// pub struct ColorRGB {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

#[derive(Clone, Copy)]
pub struct Color8(pub u8);

impl FromStr for Color8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<u8>() {
            Ok(Color8(c))
        } else {
            Err(format!("Invalid color {}", s))
        }
    }
}

pub enum Color {
    C8(Color8),
    Reset,
}

pub struct Paint {
    pub color: Color,
    pub background: u8,
}

impl std::fmt::Display for Paint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bg = self.background;
        match &self.color {
            // Color::RGB(c) => write!(f, "\x1B[{};2;{};{};{}m", bg, c.r, c.g, c.b),
            Color::C8(c) => write!(f, "\x1B[{};5;{}m", bg, c.0),
            Color::Reset => write!(f, "\x1B[{}m", bg + 1),
        }
    }
}

struct Move {
    pub x: u16,
    pub y: u16,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[{};{}H", self.y + 1, self.x + 1)
    }
}

pub struct DrawConfig {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub font: usize,
    pub color: Color8,
}

impl DrawConfig {
    pub fn new(width: u16, height: u16, x: u16, y: u16, font: usize, color: Color8) -> Self {
        Self {
            width,
            height,
            x,
            y,
            font,
            color,
        }
    }
}

pub struct Draw {
    buffer: String,
}

impl Draw {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn show_time(&mut self, d: &Vec<usize>, config: &DrawConfig, out: &Stdout) -> Result<()> {
        let f = font::FONTS[config.font];
        for y in 0..f.height() {
            self.buffer.clear();
            for digit in d {
                let mut mask: u64 = 1 << (f.width() * (y + 1));
                for _ in 0..f.width() {
                    mask >>= 1;
                    self.write_buffer(
                        config.width,
                        Paint {
                            color: if mask & f.digit(*digit) > 0 {
                                Color::C8(config.color)
                            } else {
                                Color::Reset
                            },
                            background: 48,
                        },
                    )?;
                }
                self.write_buffer(
                    config.width,
                    Paint {
                        color: Color::Reset,
                        background: 48,
                    },
                )?;
            }
            self.render_buffer(config.x, config.y + y * config.height, config.height, out)?;
            // println!("{}", buffer);
        }
        Ok(())
    }

    fn write_buffer(&mut self, width: u16, color: Paint) -> Result<()> {
        write!(&mut self.buffer, "{}{:2$}", color, " ", width as usize)?;
        Ok(())
    }

    fn render_buffer(&self, x: u16, y: u16, height: u16, mut out: &Stdout) -> Result<()> {
        for i in 0..height {
            write!(out, "{}{}", Move { x, y: y + i }, self.buffer)?;
        }
        Ok(())
    }
}
