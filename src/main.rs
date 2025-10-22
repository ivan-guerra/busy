//! A simple application that moves the mouse cursor periodically to prevent
//! the system from going idle or appearing inactive.
//!
//! The application moves the mouse between two points at configurable intervals
//! and can optionally click at each position. Press ESC to stop the application.
use anyhow::{Context, Result};
use clap::Parser;
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc;
use std::time::Duration;

/// Command-line arguments for the busy application.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BusyArgs {
    /// Time interval in seconds between mouse movements.
    #[arg(short, long, default_value_t = 5, help = "Update interval in seconds")]
    pub update_interval: u64,

    /// Whether to perform a left mouse click at the end of each movement.
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Click at the end of the movement"
    )]
    pub click: bool,
}

/// Handles keyboard events and sends a signal when ESC is pressed.
///
/// # Arguments
///
/// * `event` - The keyboard event to process
/// * `tx` - Channel sender to signal ESC key press
fn handle_esc_key(event: Event, tx: mpsc::Sender<()>) {
    if let EventType::KeyPress(Key::Escape) = event.event_type {
        tx.send(()).expect("Failed to send ESC key event");
    }
}

/// Main entry point for the busy application.
///
/// Spawns two threads:
/// 1. A keyboard listener thread that monitors for ESC key presses
/// 2. A busy loop thread that moves the mouse cursor periodically
///
/// The application continues until ESC is pressed or an error occurs.
fn main() -> Result<()> {
    let args = BusyArgs::parse();
    let (tx, rx) = mpsc::channel();

    // Spawn keyboard listener thread to detect ESC key press
    std::thread::spawn(move || -> Result<()> {
        listen(move |event| handle_esc_key(event, tx.clone()))
            .map_err(|e| anyhow::anyhow!("Error: {:?}", e))?;
        Ok(())
    });

    // Spawn main busy loop thread that moves the mouse
    let busy_handle = std::thread::spawn(move || -> Result<()> {
        let mut enigo = Enigo::new(&Settings::default())?;
        let (width, height) = enigo
            .main_display()
            .context("Failed to get main display size")?;
        let interval = Duration::from_secs(args.update_interval);
        let mut start = enigo.location().context("Failed to get mouse location")?;
        let mut end = (width / 2, height / 2);

        // Continue moving mouse until ESC is pressed
        while rx.try_recv().is_err() {
            enigo
                .move_mouse(end.0, end.1, Coordinate::Abs)
                .context("Failed to move mouse")?;
            if args.click {
                enigo
                    .button(Button::Left, Direction::Click)
                    .context("Failed to click mouse")?;
            }
            // Swap start and end positions for next iteration
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
