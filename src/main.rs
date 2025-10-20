use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct BusyArgs {
    #[arg(short, long, help = "Enable logging")]
    enable_logging: bool,
}

fn main() -> Result<()> {
    let args = BusyArgs::parse();
    if args.enable_logging {
        env_logger::init();
    }
    Ok(())
}
