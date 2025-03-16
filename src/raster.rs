use crate::{geotiff::*, stats::*};
use num_traits::Float;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct Raster {
    pub file_name: PathBuf,
    pub src_type: SrcType,
    pub src_type_str: String,
    pub stretch: StretchType,
    pub ramp: Ramp,
    pub quant: Quant,
    pub cols: usize,
    pub rows: usize,
    pub band_count: usize,
    pub stats: Stats,
    pub src_data: Vec<f32>,
    pub data: Vec<u8>,
}

impl Raster {
    pub fn new(
        file_name: PathBuf,
        src_type: Option<SrcType>,
        stretch: StretchType,
        ramp: Ramp,
        quant: Quant,
    ) -> Self {
        let src_type = src_type.unwrap_or_default();
        Self {
            file_name,
            src_type,
            stretch,
            ramp,
            quant,
            ..Default::default()
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let src = read_geotiff(&self.file_name);
        self.cols = src.raster_width;
        self.rows = src.raster_height;
        self.band_count = src.num_samples;
        self.src_type_str = src.raster_data.type_of();

        self.src_data = match src.raster_data {
            RasterData::F32(v) => v,
            RasterData::F64(v) => v.iter().map(|&i| i as f32).collect(),
            RasterData::I16(v) => v.iter().map(|&i| i as f32).collect(),
            RasterData::U8(v) => v.iter().map(|&i| i as f32).collect(),
            e => panic!("Raster format `{}` not yet supported", e.type_of()),
        };

        Ok(())
    }

    pub fn calc_stats(&mut self) {
        let mut sorted_data: Vec<f64> = self
            .src_data
            .iter()
            .copied()
            .filter(|&x| x.is_finite() && x > -1000f32 && x < 10000f32)
            .map(|x| x as f64)
            .collect();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted_data.len();
        let count = len as f64;
        let q1 = sorted_data[len / 4];
        let median = sorted_data[len / 2];
        let q3 = sorted_data[3 * len / 4];
        let min = *sorted_data.first().unwrap();
        let max = *sorted_data.last().unwrap();
        let sum: f64 = sorted_data.iter().sum::<f64>();

        let mean = sum / count;
        let variance = sorted_data
            .iter()
            .map(|&val| (mean - val).powi(2))
            .sum::<f64>()
            / count;
        let sd = variance.sqrt();
        let unique = match self.quant {
            Quant::Continuous => None,
            Quant::Discrete => Some(filter_unique(sorted_data).len()),
        };

        let stats = Stats {
            count,
            min,
            q1,
            median,
            q3,
            max,
            mean,
            sd,
            unique,
        };
        self.stats = stats;
    }

    pub fn stretch(&mut self) {
        match self.stretch {
            StretchType::MinMax => self.minmax_stretch(),
            _ => panic!("Stretch {:?} currently unsupported.", self.stretch),
        };
    }

    pub fn minmax_stretch(&mut self) {
        self.data = self
            .src_data
            .iter()
            .map(|&val| {
                let val = val as f64;
                let range = self.stats.max - self.stats.min;
                let normalized = if range > 0.0 {
                    ((val - self.stats.min) / range).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                255 - (normalized * 255.0) as u8
            })
            .collect();
    }

    // pub fn minmax_stretch(&mut self) {
    //     self.data = self
    //         .src_data
    //         .iter()
    //         .map(|&val| {

    //             let normalized = ((val - self.stats.min) / (self.stats.max - self.stats.min))
    //                 .max(0.0)
    //                 .min(1.0);
    //             255 - ((normalized * 255f32) as u8)
    //         })
    //         .collect();
    // }
}

#[derive(Clone, Debug, Default)]
pub enum SrcType {
    #[default]
    F32,
}

fn read_geotiff<P: AsRef<Path>>(path: P) -> GeoTiff {
    GeoTiff::read(File::open(path).expect("FileError: Could not read file"))
        .expect("GeoTiffError: Could not read GeoTiff")
}

fn filter_unique<T: Float>(mut vec: Vec<T>) -> Vec<T> {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec.dedup_by(|a, b| a == b);
    vec
}
