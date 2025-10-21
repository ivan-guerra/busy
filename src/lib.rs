use anyhow::{Context, Result};
use clap::Parser;
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use std::{thread, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BusyArgs {
    #[arg(short, long, default_value_t = 5, help = "Update interval in seconds")]
    pub update_interval: u64,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Click at the end of the movement"
    )]
    pub click: bool,
}

pub fn run_busy_loop(args: BusyArgs) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    let (width, height) = enigo
        .main_display()
        .context("Failed to get main display size")?;
    let interval = Duration::from_secs(args.update_interval);
    let mut start = enigo.location().context("Failed to get mouse location")?;
    let mut end = (width / 2, height / 2);

    let max_iterations = 3; // TODO: Going to make this event driven later
    for _ in 0..max_iterations {
        enigo
            .move_mouse(end.0, end.1, Coordinate::Abs)
            .context("Failed to move mouse")?;

        if args.click {
            enigo
                .button(Button::Left, Direction::Click)
                .context("Failed to click mouse")?;
        }

        std::mem::swap(&mut start, &mut end);

        thread::sleep(interval);
    }

    Ok(())
}
