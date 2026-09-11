use crate::automaton::ifiresim::{
    fuel::{base_matrix, FuelType},
    wind::{WindDirection, WindSpeed},
    Matrix,
};

/// Matrizes de preferência pré-computadas para todas as combinações de
/// vegetação × velocidade × direção (mesma estrutura dos dicionários
/// `matrizes_pref_veg_*` do sfb.py:62-71, 104-113 e 146-155).
#[derive(Debug, Clone)]
pub(super) struct PrefMatrices {
    mats: Vec<Vec<Vec<Matrix>>>,
}

impl PrefMatrices {
    pub fn build() -> Self {
        let vegs = [
            FuelType::Unburnable,
            FuelType::Low,
            FuelType::Medium,
            FuelType::High,
        ];
        let dirs = [
            WindDirection::Norte,
            WindDirection::Sul,
            WindDirection::Leste,
            WindDirection::Oeste,
        ];
        let speeds = [WindSpeed::Um, WindSpeed::Dois];

        let mut mats = Vec::with_capacity(3);
        for veg in vegs {
            let mut speed_mats = Vec::with_capacity(2);
            for speed in speeds {
                let base = base_matrix(veg, speed);
                let mut dir_mats = Vec::with_capacity(4);
                for dir in dirs {
                    dir_mats.push(rot90(&base, dir.rotation_k()));
                }
                speed_mats.push(dir_mats);
            }
            mats.push(speed_mats);
        }

        Self { mats }
    }

    pub fn get(&self, veg: FuelType, speed: WindSpeed, dir: WindDirection) -> &Matrix {
        &self.mats[usize::from(veg)][usize::from(speed)][usize::from(dir)]
    }
}

/// Rotação de 90° anti-horário, equivalente a `np.rot90(m, k)`.
///
/// sfb.py usa `np.rot90` para derivar as matrizes por direção (ex.: linhas
/// 44-46); `k` é o número de rotações de 90° no sentido anti-horário.
fn rot90<T: Copy + Default>(m: &[Vec<T>], k: usize) -> Vec<Vec<T>> {
    let mut out = m.to_vec();

    for _ in 0..(k % 4) {
        let n = out.len();
        let mut rotated = vec![vec![T::default(); n]; n];

        for r in 0..n {
            for c in 0..n {
                rotated[r][c] = out[c][n - 1 - r];
            }
        }

        out = rotated;
    }

    out
}
