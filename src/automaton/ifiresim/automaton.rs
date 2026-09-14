//! Autômato celular de propagação de incêndio florestal (SFB).
//!
//! Port em Rust do simulador Python em `modelagem-incendios-automatos/`:
//! - `sfb.py`     -> regras de transição, matrizes de preferência e grid.
//! - `projeto.py` -> cores por estado/vegetação e interatividade.
//!
//! A lógica é a mesma do original; apenas os índices numéricos do NumPy foram
//! substituídos por enumerações tipadas.

use crate::automaton::{
    ifiresim::{
        biomes::MapBiomasClass,
        cell::IFireCell,
        fuel::FuelType,
        preferences::PrefMatrices,
        scene::IFireScene,
        state::FireState,
        wind::{WindDirection, WindSpeed},
    },
    Automaton, Board, Point,
};
use gdal::raster::Buffer;
use gdal::Dataset;
use rand::Rng;
use std::sync::Arc;

#[derive(Debug)]
pub struct IFireSim {
    board: Arc<Board<IFireCell>>,
    next: Arc<Board<IFireCell>>,
    exposure: Vec<f64>,
    timer: Vec<i32>,
    wind_direction: WindDirection,
    wind_speed: WindSpeed,
    iterations: usize,
    matrices: PrefMatrices,
}

impl Automaton for IFireSim {
    type Cell = IFireCell;
    type Scene = IFireScene;

    fn name() -> &'static str {
        "Simulação de Incêndios (SFB)"
    }

    fn new(width: usize, height: usize) -> Self {
        Self::with_size(width, height)
    }

    fn board(&self) -> Arc<Board<IFireCell>> {
        self.board.clone()
    }

    fn step(&mut self) {
        self.iterations += 1;

        let width = self.board.width();
        let height = self.board.height();
        let alcance = self.wind_speed.alcance();
        let mut rng = rand::rng();
        let next_board = Arc::make_mut(&mut self.next);

        for x in 0..height {
            for y in 0..width {
                let p: Point = (x as i32, y as i32);
                let idx = x * width + y;
                let cell = self.board.get(p);
                let mut next = cell;

                match cell.state {
                    // sfb.py:352-378 — célula inflamada: decrementa o tempo de
                    // deterioração e vira queimada ao chegar em 0.
                    FireState::Burning => {
                        if self.timer[idx] == 0 {
                            next.state = FireState::Burnt;
                            self.timer[idx] = -1;
                            self.exposure[idx] = 0.0;
                        } else {
                            self.timer[idx] -= 1;
                        }
                    }
                    // sfb.py:272-350 — célula suscetível: pode inflamar ou
                    // acumular exposição.
                    FireState::Susceptible => {
                        let ctx = IgnitionContext {
                            board: &self.board,
                            matrices: &self.matrices,
                            exposure: &self.exposure,
                            wind_speed: self.wind_speed,
                            wind_direction: self.wind_direction,
                            point: p,
                            alcance,
                            veg: cell.vegetation.into(),
                        };

                        let prob = ignition_probability(ctx);

                        if rng.random::<f64>() <= prob {
                            next.state = FireState::Burning;
                        } else {
                            let inc = cell.vegetation.exposure_increment();
                            let cap = cell.vegetation.exposure_cap();
                            self.exposure[idx] = (self.exposure[idx] + inc).min(cap);
                        }
                    }
                    FireState::Burnt => {}
                }

                next_board.set(p, next);
            }
        }

        std::mem::swap(&mut self.board, &mut self.next);

        // sfb.py:200/214/228 (`check`) — ao inflamar, o contador de deterioração
        // é inicializado com o tempo da vegetação da célula.
        for idx in 0..(width * height) {
            let cell = self.board.cells()[idx];
            if cell.state == FireState::Burning && self.timer[idx] == -1 {
                self.timer[idx] = cell.vegetation.deterioration();
            }
        }
    }

    fn from(scene: IFireScene) -> Self {
        let mut sim = Self::with_size(scene.width, scene.height);
        sim.wind_direction = scene.wind_direction;
        sim.wind_speed = scene.wind_speed;

        // Abre o raster
        let dataset = Dataset::open(&scene.tif_path).expect("Falha ao abrir TIFF com GDAL");
        let band = dataset.rasterband(1).expect("Falha ao obter banda 1");

        // Transforma lat/lon (geo) em linha/coluna (pixel)
        // GeoTransform: [x_orig, pixel_w, rot_x, y_orig, rot_y, pixel_h]
        let gt = dataset.geo_transform().expect("Sem GeoTransform no TIFF");
        let col = ((scene.lon - gt[0]) / gt[1]).floor() as isize;
        let row = ((scene.lat - gt[3]) / gt[5]).floor() as isize;

        // Lê apenas a janela solicitada (width x height)
        let buffer: Buffer<u8> = band
            .read_as(
                (col, row),
                (scene.width, scene.height),
                (scene.width, scene.height),
                None,
            )
            .expect("Falha ao ler janela do raster");

        // Preenche o grid do autômato
        let board = Arc::make_mut(&mut sim.board);
        for r in 0..scene.height {
            for c in 0..scene.width {
                let pixel_val = buffer.data()[r * scene.width + c];
                let biome =
                    MapBiomasClass::try_from(pixel_val).unwrap_or(MapBiomasClass::NonVegetatedArea);

                let mut cell = board.get((r as i32, c as i32));
                cell.vegetation = biome;
                board.set((r as i32, c as i32), cell);
            }
        }

        // 5. Inicia os focos
        for (x, y) in scene.fire {
            sim.ignite(x as i32, y as i32);
        }

        sim
    }

    /// Clique esquerdo/scroll: acende a célula sob o cursor (projeto.py:410-412).
    fn paint_left(&mut self, p: Point) {
        self.ignite(p.0, p.1);
    }

    /// Clique direito: acende uma vizinhança 3x3 ao redor do cursor.
    fn paint_right(&mut self, (x, y): Point) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                self.ignite(x + dx, y + dy);
            }
        }
    }

    fn iterations(&self) -> usize {
        self.iterations
    }
}

impl IFireSim {
    fn with_size(width: usize, height: usize) -> Self {
        Self {
            board: Arc::new(Board::new(width, height)),
            next: Arc::new(Board::new(width, height)),
            exposure: vec![0.0; width * height],
            timer: vec![-1; width * height],
            wind_direction: WindDirection::Norte,
            wind_speed: WindSpeed::Um,
            iterations: 0,
            matrices: PrefMatrices::build(),
        }
    }

    /// Acende uma célula (define como inflamada).
    ///
    /// Equivalente a `handle_mouse(..., status=1)` (sfb.py:409-415) e ao clique
    /// esquerdo de projeto.py:410-412.
    fn ignite(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }

        let h = self.board.height();
        let w = self.board.width();

        let x = x as usize;
        let y = y as usize;

        if x >= h || y >= w {
            return;
        }

        let board = Arc::make_mut(&mut self.board);
        let mut cell = board.get((x as i32, y as i32));
        cell.state = match cell.state {
            FireState::Susceptible => FireState::Burning,
            _ => FireState::Burnt,
        };

        let idx = x * w + y;
        self.timer[idx] = 3;

        board.set((x as i32, y as i32), cell);
    }
}

struct IgnitionContext<'a> {
    board: &'a Board<IFireCell>,
    matrices: &'a PrefMatrices,
    exposure: &'a [f64],
    wind_speed: WindSpeed,
    wind_direction: WindDirection,
    point: Point,
    alcance: i32,
    veg: FuelType,
}

/// Probabilidade de uma célula suscetível pegar fogo nesta geração.
///
/// Porta a parte suscetível de `rules` (sfb.py:272-350): soma as
/// preferências dos vizinhos inflamados e adiciona a exposição acumulada;
/// se a soma for < 0.01 a probabilidade é 0 (sfb.py:290-293).
///
/// Recebe os campos de forma explícita (em vez de `&self`) para permitir
/// chamá-la enquanto `self.next` está mutavelmente emprestado em `step`.
fn ignition_probability(ctx: IgnitionContext) -> f64 {
    let IgnitionContext {
        board,
        matrices,
        exposure,
        wind_speed,
        wind_direction,
        point,
        alcance,
        veg,
    } = ctx;

    let (x, y) = point;

    let matrix = matrices.get(veg, wind_speed, wind_direction);
    let width = board.width();
    let height = board.height();
    let mut soma = 0.0;

    for i in -alcance..=alcance {
        for j in -alcance..=alcance {
            if i == 0 && j == 0 {
                continue;
            }

            let xi = x + i;
            let yj = y + j;

            if xi < 0 || yj < 0 || xi as usize >= height || yj as usize >= width {
                continue;
            }

            if board.get((xi, yj)).state == FireState::Burning {
                // sfb.py:288 — `matriz_preferencia[j + alcance][i + alcance]`.
                soma += matrix[(i + alcance) as usize][(j + alcance) as usize];
            }
        }
    }

    if soma < 0.01 {
        0.0
    } else {
        (soma + exposure[x as usize * width + y as usize]).min(1.0)
    }
}
