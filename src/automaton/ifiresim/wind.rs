use serde::{Deserialize, Serialize};

/// Direção do vento.
///
/// sfb.py:163 — `dir_vento = "norte"` (padrão). Os quatro valores possíveis são
/// "norte", "sul", "leste" e "oeste" (projeto.py:149).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindDirection {
    #[default]
    Norte,
    Sul,
    Leste,
    Oeste,
}

impl WindDirection {
    /// Quantas rotações de 90° (anti-horário, `np.rot90`) aplicar à matriz base.
    ///
    /// sfb.py:43-46 (e análogos em 85-88 e 127-130):
    /// norte = base, sul = k=2, leste = k=3, oeste = k=1.
    pub fn rotation_k(self) -> usize {
        match self {
            WindDirection::Norte => 0,
            WindDirection::Sul => 2,
            WindDirection::Leste => 3,
            WindDirection::Oeste => 1,
        }
    }
}

/// As matrizes por direção ficam num `Vec` na mesma ordem em que
/// `PrefMatrices::build` as preenche (`[Norte, Sul, Leste, Oeste]`), então o
/// índice é o próprio discriminante do enum.
impl From<WindDirection> for usize {
    fn from(value: WindDirection) -> Self {
        match value {
            WindDirection::Norte => 0,
            WindDirection::Sul => 1,
            WindDirection::Leste => 2,
            WindDirection::Oeste => 3,
        }
    }
}

/// Velocidade do vento (1 ou 2).
///
/// sfb.py:164 — `vel_vento = 1` (padrão). O valor 2 amplia a vizinhança e usa as
/// matrizes 5x5 (sfb.py:273).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindSpeed {
    #[default]
    Um,
    Dois,
}

impl WindSpeed {
    /// Raio da vizinhança: 1 para velocidade 1, 2 para velocidade 2.
    ///
    /// sfb.py:273 — `alcance = 1 if vel_vento == 1 else 2`.
    pub fn alcance(self) -> i32 {
        match self {
            WindSpeed::Um => 1,
            WindSpeed::Dois => 2,
        }
    }
}

/// As matrizes são agrupadas por velocidade num array de 2 posições: o Python
/// chaveia por `(vel_vento, dir_vento)` (sfb.py:62-71); aqui `vel_vento = 1`
/// vira o índice 0 e `vel_vento = 2` vira o índice 1.
impl From<WindSpeed> for usize {
    fn from(value: WindSpeed) -> Self {
        match value {
            WindSpeed::Um => 0,
            WindSpeed::Dois => 1,
        }
    }
}
