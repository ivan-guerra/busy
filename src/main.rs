use anyhow::{Context, Result};
use clap::Parser;
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc;
use std::time::Duration;

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

fn handle_esc_key(event: Event, tx: mpsc::Sender<()>) {
    if let EventType::KeyPress(Key::Escape) = event.event_type {
        tx.send(()).expect("Failed to send ESC key event");
    }
}

fn main() -> Result<()> {
    let args = BusyArgs::parse();
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || -> Result<()> {
        listen(move |event| handle_esc_key(event, tx.clone()))
            .map_err(|e| anyhow::anyhow!("Error: {:?}", e))?;
        Ok(())
    });

    let busy_handle = std::thread::spawn(move || -> Result<()> {
        let mut enigo = Enigo::new(&Settings::default())?;
        let (width, height) = enigo
            .main_display()
            .context("Failed to get main display size")?;
        let interval = Duration::from_secs(args.update_interval);
        let mut start = enigo.location().context("Failed to get mouse location")?;
        let mut end = (width / 2, height / 2);

        while rx.try_recv().is_err() {
            enigo
                .move_mouse(end.0, end.1, Coordinate::Abs)
                .context("Failed to move mouse")?;
            if args.click {
                enigo
                    .button(Button::Left, Direction::Click)
                    .context("Failed to click mouse")?;
            }
            std::mem::swap(&mut start, &mut end);

            // Check if ESC was pressed or if the sender was dropped (listen thread errored)
            match rx.recv_timeout(interval) {
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
            }
        }
        Ok(())
    });

    busy_handle
        .join()
        .map_err(|e| anyhow::anyhow!("busy loop encountered error: {:?}", e))??;

    // The rdev crate used to listen for keyboard events does not provide a way to stop listening:
    // https://github.com/Narsil/rdev/issues/72. As a workaround, we simply exit the program when
    // the busy thread is done.

    Ok(())
}
