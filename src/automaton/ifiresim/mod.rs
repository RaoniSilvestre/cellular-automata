mod automaton;
mod biomes;
mod cell;
mod fuel;
mod preferences;
mod scene;
mod state;
mod wind;

pub use automaton::IFireSim;
pub use scene::IFireScene;

type Matrix = Vec<Vec<f64>>;
