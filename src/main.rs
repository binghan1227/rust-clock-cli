use clap::Parser;
use std::io;

mod clock;
mod display;
mod font;

fn main() -> io::Result<()> {
    let config = display::Config::parse();

    let mut stdout = io::stdout();

    let mut clock = clock::Clock::new(config, &mut stdout)?;
    clock.print_clock()?;

    Ok(())
}
