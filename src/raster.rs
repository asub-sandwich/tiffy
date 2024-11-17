use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::{geotiff::*, stats::*};

#[derive(Clone, Debug, Default)]
pub struct Raster {
    pub file_name: PathBuf,
    pub src_type: SrcType,
    pub stretch: StretchType,
    pub ramp: Ramp,
    pub cols: usize,
    pub rows: usize,
    pub band_count: usize,
    pub stats: Stats,
    pub src_data: Vec<f32>,
    pub data: Vec<u8>,
}

impl Raster {
    pub fn new(file_name: PathBuf, src_type: Option<SrcType>, stretch: StretchType, ramp: Ramp) -> Self {
        let src_type = match src_type {
            Some(v) => v,
            None => SrcType::default(),
        };
        Self {
            file_name: file_name,
            src_type: src_type,
            stretch: stretch,
            ramp: ramp,
            ..Default::default()
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let src = read_geotiff(&self.file_name);
        self.cols = src.raster_width;
        self.rows = src.raster_height;
        self.band_count = src.num_samples;
        dbg!(&src.raster_data);
        
        self.src_data = match src.raster_data {
            RasterData::F32(v) => v,
            RasterData::F64(v) => v.iter().map(|&i| i as f32).collect(),
            RasterData::I16(v) => v.iter().map(|&i| i as f32).collect(),
            e => panic!("Raster format `{}` not yet supported", e.type_of()),
        };

        Ok(())
    }

    pub fn calc_stats(&mut self) {
        let mut sorted_data = self.src_data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted_data.len();
        let count = len as f32;
        let q1 = sorted_data[len / 4];
        let median = sorted_data[len / 2];
        let q3 = sorted_data[3 * len / 4];
        let sum = sorted_data.iter().sum::<f32>();
        let mean = sum / count;
        let variance = sorted_data
            .iter()
            .map(|val| {
                let diff = mean - (*val);
                diff * diff
            })
            .sum::<f32>()
            / count;
        let sd = variance.sqrt();

        let mut l1 = std::f32::MAX;
        let mut l2 = std::f32::MAX;
        let mut h1 = std::f32::MIN;
        let mut h2 = std::f32::MIN;

        for &value in &sorted_data {
            if value < l1 {
                l2 = l1;
                l1 = value;
            } else if value > l1 && value < l2 {
                l2 = value;
            } else if value > h1 {
                h2 = h1;
                h1 = value;
            } else if value < h1 && value > h2 {
                h2 = value;
            }
        }
        let min = l2;
        let max = h2;

        let stats = Stats {
            count, min, q1, median, q3, max, mean, sd
        };
        self.stats = stats;
    }

    pub fn stretch(&mut self) {
        match self.stretch {
            StretchType::MinMax => self.minmax_stretch(),
            _ => panic!("Stretch {:?} currently unsupported.", self.stretch)
        };
    }

    pub fn minmax_stretch(&mut self) {
        self.data = self.src_data
            .iter()
            .map(|&val| {
                let normalized = ((val - self.stats.min) / (self.stats.max - self.stats.min))
                    .max(0.0).min(1.0);
                255 - ((normalized * 255f32) as u8)
            })
            .collect();
    }
}

#[derive(Clone, Debug, Default)]
pub enum SrcType {
    #[default]
    F32,
}

fn read_geotiff<P: AsRef<Path>>(path: P) -> GeoTiff {
    GeoTiff::read(File::open(path).expect("FileError: Could not read file")).expect("GeoTiffError: Could not read GeoTiff")
}
