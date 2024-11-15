mod color;


use clap::Parser;
use image::GrayImage;
use gdal::Dataset;
use show_image::event::{WindowEvent::KeyboardInput, VirtualKeyCode};
use show_image::create_window;
use std::error::Error;
use std::path::PathBuf;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {

    /* Init and get args */
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
	env_logger::init();
	let args = Args::parse();
    let wd = std::env::current_dir()?;
    let path = wd.join(args.input);
    //let s = args.stretch;

    /* Get image info and data via GDAL */
    let src = Dataset::open(&path)?;
    let num_bands = src.raster_count();
    if num_bands > 1 {
        println!("Currently only handles first band of data (DEM)");
    }
    let band = src.rasterband(1)?;
    let nodata = band.no_data_value().unwrap() as f32;
    let size = band.size();
    let buf = band.read_as::<f32>((0, 0), size, size, None)?;
    let data = buf.data().to_vec();
    src.close()?;

    /* Stretch the data */
    let stats = Stats::new(&data, nodata);
    let lower = stats.max;//stats.median - s * stats.iqr;
    let upper = stats.min; //stats.median + s * stats.iqr;
    let stretched_data: Vec<u8> = data
        .iter()
        .map(|&val| {
            let normalized = ((val - lower) / (upper - lower)).max(0.0).min(1.0);
            255 -((normalized * 255f32) as u8)
        }).collect();

    let image = GrayImage::from_vec(size.0 as u32, size.1 as u32, stretched_data).expect("Could not create image buffer");

    /* Display the data */
    let window = create_window("Tiffy", Default::default())?;
    window.set_image(path.to_str().unwrap(), image)?;

    for event in window.event_channel()? {
        if let KeyboardInput(event) = event {
            if !event.is_synthetic && event.input.key_code == Some(VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                println!("Closing...");
                break;
            }
        }
    }
    
	Ok(())
}

#[derive(Debug)]
struct Stats {
    _count: f32,
    min: f32,
    _q1: f32,
    _median: f32,
    _q3: f32,
    max: f32,
    _iqr: f32,
    _mean: f32,
    _sd: f32,
}

impl Stats {
    pub fn new(data: &[f32], nodata: f32) -> Self {
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted_data.len();
        let count = len as f32;
        let _min = sorted_data[0];
        let q1 = sorted_data[len / 4];
        let median = sorted_data[len / 2];
        let q3 = sorted_data[3 * len / 4];
        let _max = sorted_data[len-1];
        let iqr = q3 - q1;
        let sum = sorted_data.iter().sum::<f32>();
        let mean = sum / len as f32;
        let variance = sorted_data.iter().map(|val| {
            let diff = mean - (*val);
            diff * diff
        }).sum::<f32>() / len as f32;
        let sd = variance.sqrt();

        let mut lowest = std::f32::MAX;
        let mut second_lowest = std::f32::MAX;
        let mut highest = std::f32::MIN;
        let mut second_highest = std::f32::MIN;

        for &value in &sorted_data {
            if value < lowest {
                second_lowest = lowest;
                lowest = value;
            } else if value > lowest && value < second_lowest {
                second_lowest = value;
            } else if value > highest {
                second_highest = highest;
                highest = value;
            } else if value < highest && value > second_highest {
                second_highest = value;
            }
        }


        
        Self { _count: count, min: second_lowest, _q1: q1, _median: median, _q3: q3, max: second_highest, _iqr: iqr, _mean: mean, _sd: sd }
    }
}

#[derive(Parser)]
struct Args {
    /// Input Image path
    input: PathBuf,
    /// Number of Std. Dev. for stretch
    #[arg(short, long, default_value_t=2.0)]
    stretch: f32,
}


