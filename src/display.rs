use std::io::{self, Write, Stdout};
use std::fmt::Write as _;

use crate::clock;
use crate::font;

// const CLEAR_ALL: &str = "\x1B[2J";
// const HIDE: &str = "\x1B[?25l";

#[derive(Clone, Copy)]
pub struct Config {
    pub height: u16,
    pub width: u16,
    pub x: u16,
    pub y: u16,
    pub color: Color8,
}

// #[derive(Clone, Copy)]
// pub struct ColorRGB {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

#[derive(Clone, Copy)]
pub struct Color8 (pub u8);

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
    pub x : u16,
    pub y : u16,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[{};{}H", self.y + 1, self.x + 1)
    }
}

pub struct Draw {
    config: Config,
    buffer: String,
}

const COLON : usize = 10;

impl Draw {
    pub fn new (config: Config) -> Self {
        Self {
            buffer: String::new(),
            config,
        }
    }

    pub fn show_digit(&mut self, digit : &u64, width : u16, height : u16, out : & Stdout) -> io::Result<()> {
        // let width = 5;
        // let height = 7;

        for y in 0..height {
            self.buffer.clear();
            let mut mask = 1 << (width * (y + 1));
            for _ in 0..width {
                mask >>= 1;
                self.write_buffer(Paint {
                    color : if mask & digit > 0 {Color::C8(self.config.color)} else {Color::Reset},
                    background : 48,
                }).expect("Write Buffer Error");
            }
            self.render_buffer(self.config.x, self.config.y + y * self.config.height, out)?;
            // println!("{}", buffer);
        }
        Ok(()) 
    }

    pub fn show_time(&mut self, time : clock::Time, out : & Stdout) -> io::Result<()> {
        let d = [time.h / 10, time.h % 10, COLON, time.m / 10, time.m % 10, COLON, time.s / 10, time.s % 10];

        for y in 0..font::HEIGHT {
            self.buffer.clear();
            for digit in d {
                let mut mask = 1 << (font::WIDTH * (y + 1));
                for _ in 0..font::WIDTH {
                    mask >>= 1;
                    self.write_buffer(Paint {
                        color : if mask & font::DIGIT[digit] > 0 {Color::C8(self.config.color)} else {Color::Reset},
                        background : 48,
                    }).expect("Write Buffer Error");
                }
                self.write_buffer(Paint {
                    color : Color::Reset,
                    background : 48,
                }).expect("Write Buffer Error");
            }
            self.render_buffer(self.config.x, self.config.y + y * self.config.height, out)?;
            // println!("{}", buffer);
        }
        Ok(()) 
    }

    fn write_buffer (&mut self, color: Paint) -> std::fmt::Result {
        write! (
            &mut self.buffer,
            "{}{:2$}",
            color, " ", self.config.width as usize
        )?;
        Ok(())
    }

    fn render_buffer (& self, x: u16, y: u16, mut out: & Stdout) -> io::Result<()> {
        for i in 0..self.config.height {
            write!(out, "{}{}", Move{x, y: y + i}, self.buffer)?;
        } 
        Ok(())
    }
}
    

