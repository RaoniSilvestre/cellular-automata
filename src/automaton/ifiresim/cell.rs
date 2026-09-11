use macroquad::color::Color;

use crate::automaton::ifiresim::{biomes::MapBiomasClass, state::FireState};

/// Célula do grid: junta o estado (que muda) com a vegetação (fixa).
///
/// No Python esses dois conceitos vivem em arrays separados
/// (`grid_array` e `grid_vegetacao`); aqui a vegetação fica na própria célula
/// para que o `Board` consiga colorir cada célula sozinho (via `Into<Color>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IFireCell {
    pub state: FireState,
    pub vegetation: MapBiomasClass,
}

impl From<IFireCell> for Color {
    fn from(value: IFireCell) -> Self {
        match value.state {
            FireState::Burning => Color::from_rgba(226, 50, 54, 255),
            FireState::Burnt => Color::from_rgba(0, 0, 0, 0),
            FireState::Susceptible => value.vegetation.into(),
        }
    }
}

impl Default for IFireCell {
    fn default() -> Self {
        Self {
            state: FireState::Susceptible,
            vegetation: MapBiomasClass::NonVegetatedArea,
        }
    }
}
