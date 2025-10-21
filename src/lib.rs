use anyhow::{Context, Result};
use clap::Parser;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use kurbo::{CubicBez, ParamCurve, Point};
use rand::prelude::*;
use std::{thread, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BusyArgs {
    #[arg(short, long, default_value_t = 5, help = "Update interval in seconds")]
    pub update_interval: u64,

    #[arg(
        short,
        long,
        default_value_t = 75,
        value_parser = clap::value_parser!(u8).range(1..=100),
        help = "Mouse speed from 1 to 100 (1 = slowest, 100 = fastest)"
    )]
    pub mouse_speed: u8,
}

fn get_rand_point(width: i32, height: i32) -> Point {
    let mut rng = rand::rng();
    Point::new(
        rng.random_range(0.0..width.into()),
        rng.random_range(0.0..height.into()),
    )
}

fn create_bez_curve(start: Point, end: Point, width: i32, height: i32) -> CubicBez {
    let ctrl1 = get_rand_point(width, height);
    let ctrl2 = get_rand_point(width, height);

    CubicBez::new(start, ctrl1, ctrl2, end)
}

fn move_along_curve(enigo: &mut Enigo, bez_curve: &CubicBez, steps: usize) -> Result<()> {
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let point = bez_curve.eval(t);

        enigo.move_mouse(point.x as i32, point.y as i32, Coordinate::Abs)?;
        thread::sleep(Duration::from_millis(10)); // Small delay for smoother movement
    }
    Ok(())
}

fn get_mouse_position(enigo: &mut Enigo) -> Result<Point> {
    let mouse_pos = enigo.location().context("Failed to get mouse location")?;

    Ok(Point::new(mouse_pos.0.into(), mouse_pos.1.into()))
}

fn calculate_step_size(mouse_speed: u8) -> usize {
    let min_step = 256;
    let max_step = 32;

    min_step - ((usize::from(mouse_speed) - 1) * (min_step - max_step) / 99)
}

pub fn run_busy_loop(args: BusyArgs) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    let (width, height) = enigo
        .main_display()
        .context("Failed to get main display size")?;
    let interval = Duration::from_secs(args.update_interval);
    let max_iterations = 2; // TODO: Going to make this event driven later
    let steps = calculate_step_size(args.mouse_speed);
    let mut start = get_mouse_position(&mut enigo)?;
    let mut end = get_rand_point(width, height);

    for _ in 0..max_iterations {
        let bez_curve = create_bez_curve(start, end, width, height);

        move_along_curve(&mut enigo, &bez_curve, steps)?;

        start = end;
        end = get_rand_point(width, height);

        thread::sleep(interval);
    }

    Ok(())
}
