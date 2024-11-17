#![allow(dead_code)]

mod cli;
mod color;
mod geotiff;
mod raster;
mod stats;

use color::*;
use raster::*;
use stats::*;
use image::{Rgb, RgbImage};
use show_image::event::{VirtualKeyCode, WindowEvent::KeyboardInput};
use show_image::{create_window, Color, WindowOptions};
use std::error::Error;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    /* Init and get args */
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    let args = cli::get_args();
    let wd = std::env::current_dir()?;
    let stretch = args.stretch;
    let ramp = args.color;
    let path = wd.join(args.input);

    /* Get image data with Geotiff? */
    let mut image = Raster::new(path, Some(SrcType::F32), stretch, ramp); // Hardcoding data type, but wanted to try to build it in for later
    image.read()?;
    image.calc_stats();
    image.stretch();

    /* Print Image Info */
    println!("Name: {:?}", image.file_name);
    println!("Cols: {}", image.cols);
    println!("Rows: {}", image.rows);
    println!("Band: {}", image.band_count);
    println!("{:#?}", image.stats);

    /* Apply Color Ramp */
    let mut buf = RgbImage::new(image.cols as u32, image.rows as u32);

    for fx in 0..image.cols {
        for fy in 0..image.rows {
            let idx = fy * image.cols + fx;
            let val = image.data[idx] as usize;
            let pixel = match image.ramp {
                Ramp::Elevation => Rgb {0: ELEV_CR[255-val]},
                Ramp::Ryg => Rgb {0: RYG[255-val]},
            };
            buf.put_pixel(fx as u32, fy as u32, pixel);
        }
    }

    /* Set up window options */
    let window_options = WindowOptions {
        preserve_aspect_ratio: true,
        background_color: Color::rgb(0.5, 0.5, 0.5),
        start_hidden: false,
        size: Some([1200, 800]),
        resizable: true,
        borderless: false,
        fullscreen: false,
        overlays_visible: true,
        default_controls: true,
    };

    /* Display the data */
    let window = create_window("Tiffy", window_options)?;
    window.set_image(image.file_name.to_str().unwrap(), buf)?;

    for event in window.event_channel()? {
        if let KeyboardInput(event) = event {
            if !event.is_synthetic
                && event.input.key_code == Some(VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                println!("Closing...");
                break;
            }
        }
    }

    Ok(())
}