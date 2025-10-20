use anyhow::{Context, Result};
use clap::Parser;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::{thread, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BusyArgs {
    #[arg(short, long, help = "Enable logging")]
    pub enable_logging: bool,

    #[arg(short, long, help = "Center mouse on start")]
    pub center_mouse: bool,

    #[arg(short, long, default_value_t = 5, help = "Update interval in seconds")]
    pub update_interval: u32,
}

pub fn run_busy_loop(args: BusyArgs) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    let (width, height) = enigo
        .main_display()
        .context("Failed to get main display size")?;

    if args.center_mouse {
        let center_abs = (width / 2, height / 2);
        enigo
            .move_mouse(center_abs.0, center_abs.1, Coordinate::Abs)
            .context("Failed to center mouse")?;
    }

    let interval = Duration::from_secs(args.update_interval as u64);
    let max_iterations = 5; // TODO: Going to make this event driven later
    for _ in 0..max_iterations {
        thread::sleep(interval);
    }

    Ok(())
}
