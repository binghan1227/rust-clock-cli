use anyhow::Result;
use chrono::prelude::*;
use std::io::{Error, Stdout, Write};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;
use terminal_size::{Height, Width, terminal_size};

use crate::config;
use crate::countdown;
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
    config: config::Config,
    countdown: Option<countdown::CD>,
    out: &'a mut Stdout,
}

const CLEAR_ALL: &str = "\x1B[2J";
const HIDE: &str = "\x1B[?25l";
const SHOW: &str = "\x1B[?25h";
const COLON: usize = 10;
const SPACE: usize = 11;
const A: usize = 12;
const P: usize = 13;
const M: usize = 14;

impl<'a> Clock<'a> {
    pub fn new(config: config::Config, out: &'a mut Stdout) -> Result<Self, Error> {
        write!(out, "{}{}", CLEAR_ALL, HIDE)?;
        let countdown = if let Some(countdown::CountdownCommands::Countdown(countdown_args)) =
            config.countdown
        {
            match (countdown_args.duration, countdown_args.target) {
                (Some(duration), None) => Some(countdown::CD::new_duration(duration)),
                (None, Some(target)) => {
                    let target_utc = target.with_timezone(&Utc);
                    Some(countdown::CD::new_target(target_utc))
                }
                _ => None,
            }
        } else {
            None
        };
        Ok(Clock {
            config,
            countdown,
            out,
        })
    }

    pub fn print_clock(&mut self, stop: &Arc<AtomicBool>) -> Result<()> {
        let term_size = terminal_size();
        let Some((Width(w), Height(h))) = term_size else {
            panic!("Error while getting the terminal's size");
        };

        let f = font::FONTS[self.config.font];
        self.config.x = self.config.x + w / 2
            - ((f.width() + 1) * self.config.width)
                * if self.config.use12_hour { 11 } else { 8 }
                / 2;
        self.config.y =
            self.config.y + h / 2 - f.height() * self.config.height / 2;
        let mut d = display::Draw::new();
        let config = display::DrawConfig::new(
            self.config.width,
            self.config.height,
            self.config.x,
            self.config.y,
            self.config.font,
            self.config.color,
        );
        let mut last_len: usize = 0;

        while !stop.load(Ordering::SeqCst) {
            if self.countdown.is_some() {
                let remaining = self.countdown.unwrap().remaining();
                if remaining.is_empty() {
                    self.countdown = None;
                    write!(self.out, "{}", CLEAR_ALL)?;
                }
                if remaining.len() != last_len {
                    last_len = remaining.len();
                    write!(self.out, "{}", CLEAR_ALL)?;
                }
                if let Some(countdown::CountdownCommands::Countdown(countdown_args)) =
                    self.config.countdown
                {
                    let cf = font::FONTS[countdown_args.font];
                    let countdown_config = display::DrawConfig::new(
                        countdown_args.width,
                        countdown_args.height,
                        w / 2
                            - ((cf.width() + 1) * countdown_args.width)
                                * remaining.len() as u16
                                / 2,
                        h / 2
                            - cf.height() * countdown_args.height
                            - cf.height() * countdown_args.height / 2
                            - 2,
                        countdown_args.font,
                        countdown_args.color,
                    );
                    d.show_time(&remaining, &countdown_config, self.out)?;
                }
            }
            let formatted_time = self.time_formatter(Time::now());
            d.show_time(&formatted_time, &config, self.out)?;
            self.out.flush()?;
            thread::sleep(Duration::from_nanos(
                1_000_000_000 - chrono::Local::now().nanosecond() as u64,
            ));
        }

        Ok(())
    }

    pub fn time_formatter(&mut self, time: Time) -> Vec<usize> {
        if !self.config.use12_hour {
            vec![
                time.h / 10,
                time.h % 10,
                COLON,
                time.m / 10,
                time.m % 10,
                COLON,
                time.s / 10,
                time.s % 10,
            ]
        } else {
            vec![
                time.pm_h.1 / 10,
                time.pm_h.1 % 10,
                COLON,
                time.m / 10,
                time.m % 10,
                COLON,
                time.s / 10,
                time.s % 10,
                SPACE,
                if time.pm_h.0 { P } else { A },
                M,
            ]
        }
    }
}

impl<'a> Drop for Clock<'a> {
    fn drop(&mut self) {
        write!(self.out, "{}{}", CLEAR_ALL, SHOW).expect("Error while dropping");
    }
}
