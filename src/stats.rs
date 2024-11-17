
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub count: f32,
    pub min: f32,
    pub q1: f32,
    pub median: f32,
    pub q3: f32,
    pub max: f32,
    pub mean: f32,
    pub sd: f32
}

#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum StretchType {
    Irq,
    #[default]
    MinMax,
    Sd,
    Mad
}

#[derive(Clone, clap::ValueEnum, Debug, Default)]
pub enum Ramp {
    #[default]
    Elevation,
    Ryg,
}