use std::sync::{atomic::{ AtomicBool, Ordering }, Arc};
use clap::Parser;
use std::io;

mod clock;
mod display;
mod font;

fn main() -> io::Result<()> {
    let config = display::Config::parse();

    let stop = Arc::new(AtomicBool::new(false));

    {
        let stop_clone = Arc::clone(&stop);
        ctrlc::set_handler(move || stop_clone.store(true, Ordering::SeqCst))
            .expect("error setting Ctrl‑C handler");
    }

    let mut stdout = io::stdout();

    let mut clock = clock::Clock::new(config, &mut stdout)?;
    clock.print_clock(&stop)?;

    Ok(())
}
