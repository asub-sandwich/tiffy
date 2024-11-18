use clap::Parser;
use crate::stats::*;

#[derive(Parser)]
pub struct Args {
    /// Input Image path
    pub input: String,
    /// Color Ramp to apply
    #[arg(short, long, value_enum, default_value_t=Ramp::Elevation)]
    pub color: Ramp,
    /// Mathematical Strech to apply
    #[arg(short, long, value_enum, default_value_t=StretchType::MinMax)]
    pub stretch: StretchType,
    /// Quantitativeness
    #[arg(short, long, value_enum, default_value_t=Quant::Continuous)]
    pub quant: Quant,
}

pub fn get_args() -> Args {
    Args::parse()
}