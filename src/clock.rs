use chrono::prelude::*;
use std::io::{Stdout, Write};
use std::thread;
use std::time::Duration;
use terminal_size::{Height, Width, terminal_size};

use crate::display;
use crate::font::{self};

pub struct Time {
    pub h: usize,
    pub m: usize,
    pub s: usize,
}

impl Time {
    pub fn new(time: NaiveTime) -> Self {
        Time {
            h: time.hour() as usize,
            m: time.minute() as usize,
            s: time.second() as usize,
        }
    }

    pub fn now() -> Self {
        let now = chrono::Local::now().naive_local();
        Time::new(now.time())
    }
}

pub fn print_clock(mut config: display::Config, out: &mut Stdout) -> std::io::Result<()> {
    let term_size = terminal_size();
    let Some((Width(w), Height(h))) = term_size else {
        panic!("Error while getting the terminal's size");
    };
    // let config = display::Config {
    //     height : 1,
    //     width : 2,
    //     x : w / 2 - ((font::WIDTH * 2 + 2) * 8) / 2,
    //     y : h / 2 - font::HEIGHT / 2,
    //     color : display::Color8(11),
    // };

    config.x = config.x + w / 2 - ((font::WIDTH + 1) * config.width) * 8 / 2;
    config.y = config.y + h / 2 - font::HEIGHT * config.height / 2;
    let mut d = display::Draw::new(config);

    loop {
        d.show_time(Time::now(), out)?;
        out.flush()?;
        thread::sleep(Duration::from_secs(1));
    }
}

/*
pub fn test(out : &mut Stdout) -> std::io::Result<()> {
    let config = display::Config {
        height : 1,
        width : 2,
        x : 0,
        y : 0,
        color : display::Color8(2),
    };
    let mut d = display::Draw::new(config);

    // write!(stdout, "{}{}", reset, CLEAR_ALL)?;
    // d.show_digit(&font::DIGIT[0], 5, 7)?;
    // write!(stdout, "{}{}", reset, CLEAR_ALL)?;
    // d.show_digit(&font::DIGIT[1], 5, 7)?;

    for i in 0..=9 {
        // write!(stdout, "{}{}", reset, CLEAR_ALL)?;
        d.show_digit(&font::DIGIT[i], font::WIDTH, font::HEIGHT, out)?;
        out.flush()?;
        // println!("");
        thread::sleep(time::Duration::from_millis(1000));
        // io::stdin()
        //     .read_line(&mut s)
        //     .expect("Reading error");
    }

    Ok(())
}
*/
