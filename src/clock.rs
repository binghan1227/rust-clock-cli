use chrono::prelude::*;
use std::io::{Error, Stdout, Write};
use std::thread;
use std::time::Duration;
use terminal_size::{Height, Width, terminal_size};

use crate::display;
use crate::font::{self};

pub struct Time {
    pub h: usize,
    pub m: usize,
    pub s: usize,
    pub pm_h: (bool, usize),
}

impl Time {
    pub fn new(time: NaiveTime) -> Self {
        Time {
            h: time.hour() as usize,
            m: time.minute() as usize,
            s: time.second() as usize,
            pm_h: (time.hour12().0, time.hour12().1 as usize),
        }
    }

    pub fn now() -> Self {
        let now = chrono::Local::now().naive_local();
        Time::new(now.time())
    }
}

pub struct Clock<'a> {
    config: display::Config,
    out: &'a mut Stdout,
}

const CLEAR_ALL: &str = "\x1B[2J";
const HIDE: &str = "\x1B[?25l";
const SHOW: &str = "\x1B[?25h";

impl<'a> Clock<'a> {
    pub fn new(config: display::Config, out: &'a mut Stdout) -> Result<Self, Error> {
        write!(out, "{}{}", CLEAR_ALL, HIDE)?;
        Ok(Clock { config, out })
    }

    pub fn print_clock(&mut self) -> std::io::Result<()> {
        let term_size = terminal_size();
        let Some((Width(w), Height(h))) = term_size else {
            panic!("Error while getting the terminal's size");
        };

        self.config.x = self.config.x + w / 2
            - ((font::WIDTH[self.config.font] + 1) * self.config.width)
                * if self.config.use12_hour { 11 } else { 8 }
                / 2;
        self.config.y =
            self.config.y + h / 2 - font::HEIGHT[self.config.font] * self.config.height / 2;
        let mut d = display::Draw::new(self.config);

        loop {
            d.show_time(Time::now(), self.out)?;
            self.out.flush()?;
            thread::sleep(Duration::from_nanos(
                1_000_000_000 - chrono::Local::now().nanosecond() as u64,
            ));
        }
    }
}

impl<'a> Drop for Clock<'a> {
    fn drop(&mut self) {
        write!(self.out, "{}{}", CLEAR_ALL, SHOW).expect("Error while dropping");
    }
}
