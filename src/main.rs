use anyhow::Result;
use busy::BusyArgs;
use clap::Parser;

fn main() -> Result<()> {
    let args = BusyArgs::parse();

    if args.enable_logging {
        env_logger::init();
    }

    busy::run_busy_loop(args)?;

    Ok(())
}
