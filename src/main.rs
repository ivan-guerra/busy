use anyhow::Result;
use busy::BusyArgs;
use clap::Parser;

fn main() -> Result<()> {
    let args = BusyArgs::parse();

    busy::run_busy_loop(args)?;

    Ok(())
}
