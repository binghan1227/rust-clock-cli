use crate::{display, font};
use anyhow::Result;
use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use clap::{ArgGroup, Parser, Subcommand};

#[derive(Clone, Copy)]
pub struct CD {
    pub end: DateTime<Utc>,
}

const COLON: usize = 10;

impl CD {
    pub fn new_duration(duration: Duration) -> Self {
        Self {
            end: Utc::now() + duration,
        }
    }

    pub fn new_target(target: DateTime<Utc>) -> Self {
        Self { end: target }
    }

    pub fn remaining(&self) -> Vec<usize> {
        let now = Utc::now();
        if now >= self.end {
            vec![]
        } else {
            let secs = (self.end - now + Duration::seconds(1)).num_seconds();
            let h = (secs / 3600) as usize;
            let m = ((secs % 3600) / 60) as usize;
            let s = (secs % 60) as usize;
            let mut digits = Vec::new();
            if h > 0 {
                digits.push(h / 10);
                digits.push(h % 10);
                digits.push(COLON);
            }
            if m > 0 {
                digits.push(m / 10);
                digits.push(m % 10);
                digits.push(COLON);
            }
            digits.push(s / 10);
            digits.push(s % 10);
            digits
        }
    }
}

pub fn parse_target_time(input: &str) -> Result<DateTime<Local>> {
    let input = input.trim();

    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M"))
    {
        return Local
            .from_local_datetime(&dt)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Unknown local time"));
    }

    let naive_time = NaiveTime::parse_from_str(input, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(input, "%H:%M"));
    if let Ok(t) = naive_time {
        let today = Local::now().date_naive();
        let dt_today = Local
            .from_local_datetime(&NaiveDateTime::new(today, t))
            .single()
            .ok_or_else(|| anyhow::anyhow!("Unknown local time"))?;

        return Ok(if dt_today <= Local::now() {
            dt_today + chrono::Duration::days(1)
        } else {
            dt_today
        });
    }

    Err(anyhow::anyhow!("Invalid time format: {input}"))
}

pub fn parse_duration(input: &str) -> Result<Duration> {
    let input = input.trim().to_lowercase();

    if input.ends_with('h') {
        let h: i64 = input[..input.len() - 1].parse()?;
        Ok(Duration::hours(h))
    } else if input.ends_with('m') {
        let m: i64 = input[..input.len() - 1].parse()?;
        Ok(Duration::minutes(m))
    } else if input.ends_with('s') {
        let s: i64 = input[..input.len() - 1].parse()?;
        Ok(Duration::seconds(s))
    } else {
        let s: i64 = input.parse()?;
        Ok(Duration::seconds(s))
    }
}

#[derive(Subcommand, Clone, Copy)]
pub enum CountdownCommands {
    /// Start a countdown
    Countdown(CountdownArgs),
}

#[derive(Clone, Copy, Parser)]
#[command(group(
    ArgGroup::new("countdown")
        .required(true)
        .args(["duration", "target"])
))]
pub struct CountdownArgs {
    /// Countdown specific duration (e.g. 10m, 1h, 30s)
    #[arg(short = 'd', long, value_parser = parse_duration)]
    pub duration: Option<Duration>,

    /// Countdown to target time (e.g. "2025-07-19 12:00")
    #[arg(short = 't', long, value_parser = parse_target_time)]
    pub target: Option<DateTime<Local>>,

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

    /// Font style for digits (0: 5x7, 1: 3x5).
    #[arg(short, long, default_value_t = 0, value_parser = font::font_in_range)]
    pub font: usize,

    /// Color of the digit tiles.
    #[arg(short, long, default_value = "3")]
    pub color: display::Color8,
}
