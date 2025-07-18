use std::{io::{self, Write}};

use crate::clock::{print_clock};

mod display;
mod clock;
mod font;

const CLEAR_ALL: &str = "\x1B[2J";
const HIDE: &str = "\x1B[?25l";


fn main() -> io::Result<()> {
    let mut stdout  = io::stdout();
    write!(stdout, "{}{}", CLEAR_ALL, HIDE)?;

    // println!("Start");
    // let w : u16 = 8;
    // let mut config = display::Config {
    //     height : 1,
    //     width : 2,
    //     x : 0,
    //     y : 0,
    //     color : display::Color8(2),
    // };
    // for i in 0..=9 {
    //     config.x = w * 2 * i;
    //     let mut d = display::Draw::new(config);
    //     d.show_digit(&num[i as usize], 5, 7)?;
    // }

    // test(&mut stdout)?;
    print_clock(&mut stdout)?;


    Ok(())
}

