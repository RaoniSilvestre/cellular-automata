use crate::automaton::ifiresim::wind::{WindDirection, WindSpeed};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IFireScene {
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub wind_direction: WindDirection,
    #[serde(default)]
    pub wind_speed: WindSpeed,
    #[serde(default)]
    pub fire: Vec<(usize, usize)>,

    pub tif_path: PathBuf,
    pub lat: f64,
    pub lon: f64,
}
