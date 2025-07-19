use anyhow::Result;
use clap::Parser;
use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

mod clock;
mod config;
mod countdown;
mod display;
mod font;

fn main() -> Result<()> {
    let config = config::Config::parse();

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
