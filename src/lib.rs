use anyhow::Result;
use clap::Parser;
use enigo::{Coordinate, Enigo, Mouse, Settings};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BusyArgs {
    #[arg(short, long, help = "Enable logging")]
    pub enable_logging: bool,

    #[arg(short, long, help = "Center mouse on start")]
    pub center_mouse: bool,
}

pub fn run_busy_loop(args: BusyArgs) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    let (width, height) = enigo.main_display()?;

    if args.center_mouse {
        let center_abs = (width / 2, height / 2);
        enigo.move_mouse(center_abs.0, center_abs.1, Coordinate::Abs)?;
    }

    Ok(())
}
