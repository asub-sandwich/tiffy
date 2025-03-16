#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub count: f64,
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
    pub mean: f64,
    pub sd: f64,
    pub unique: Option<usize>,
}

#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum StretchType {
    Irq,
    #[default]
    MinMax,
    Sd,
    Mad,
}

#[derive(Clone, clap::ValueEnum, Debug, Default)]
pub enum Ramp {
    #[default]
    Elevation,
    Ryg,
}

#[derive(Clone, clap::ValueEnum, Debug, Default, PartialEq)]
pub enum Quant {
    #[default]
    Continuous,
    Discrete,
}
