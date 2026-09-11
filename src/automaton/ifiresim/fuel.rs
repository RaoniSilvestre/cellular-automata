use crate::automaton::ifiresim::{biomes::MapBiomasClass, wind::WindSpeed, Matrix};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelType {
    Unburnable, // Água, mineração, área urbana, rocha
    Low,        // Pastagem, lavouras temporárias, marismas, vegetação campestre
    Medium,     // Savana, restingas, cafezais, pomares
    High,       // Formações florestais fechadas, silvicultura
}

impl From<MapBiomasClass> for FuelType {
    fn from(biome: MapBiomasClass) -> Self {
        match biome {
            // Não combustíveis
            MapBiomasClass::Water
            | MapBiomasClass::RiverLakeAndOcean
            | MapBiomasClass::Aquaculture
            | MapBiomasClass::UrbanArea
            | MapBiomasClass::Mining
            | MapBiomasClass::RockyOutcrop
            | MapBiomasClass::BeachDuneAndSandSpot
            | MapBiomasClass::HypersalineTidalFlat
            | MapBiomasClass::PhotovoltaicPowerPlant
            | MapBiomasClass::WindFarm
            | MapBiomasClass::NonVegetatedArea
            | MapBiomasClass::OtherNonVegetatedAreas => FuelType::Unburnable,

            // Carga baixa / rasteira
            MapBiomasClass::Grassland
            | MapBiomasClass::Pasture
            | MapBiomasClass::TemporaryCrop
            | MapBiomasClass::Soybean
            | MapBiomasClass::SugarCane
            | MapBiomasClass::Rice
            | MapBiomasClass::Cotton
            | MapBiomasClass::OtherTemporaryCrops
            | MapBiomasClass::Agriculture
            | MapBiomasClass::SaltMarsh => FuelType::Low,

            // Carga média / savânica / perene
            MapBiomasClass::SavannaFormation
            | MapBiomasClass::FloodedSavanna
            | MapBiomasClass::HerbaceousOrShrubbyVegetation
            | MapBiomasClass::HerbaceousAndShrubFormation
            | MapBiomasClass::Wetland
            | MapBiomasClass::HerbaceousSandbank
            | MapBiomasClass::PerennialCrop
            | MapBiomasClass::Coffee
            | MapBiomasClass::Citrus
            | MapBiomasClass::MosaicOfUses
            | MapBiomasClass::OtherPerennialCrops
            | MapBiomasClass::Farming => FuelType::Medium,

            // Carga alta / dossel fechado
            MapBiomasClass::Forest
            | MapBiomasClass::ForestFormation
            | MapBiomasClass::FloodableForest
            | MapBiomasClass::Mangrove
            | MapBiomasClass::WoodedSandbank
            | MapBiomasClass::PalmOil
            | MapBiomasClass::ForestPlantation => FuelType::High,
        }
    }
}

/// As matrizes de preferência e os parâmetros por vegetação (deterioração,
/// exposição) são armazenados em `Vec`s indexados por tipo. O Python usa os
/// inteiros 0/1/2 em `grid_vegetacao`; aqui fazemos a mesma indexação, adicionando apenas o estado
/// 0 para ser `Unburnable`.
impl From<FuelType> for usize {
    fn from(value: FuelType) -> Self {
        match value {
            FuelType::Unburnable => 0,
            FuelType::Low => 1,
            FuelType::Medium => 2,
            FuelType::High => 3,
        }
    }
}

impl From<&MapBiomasClass> for FuelType {
    fn from(value: &MapBiomasClass) -> Self {
        (*value).into()
    }
}

/// Matriz de preferência base (vento norte) por vegetação e velocidade.
pub(super) fn base_matrix(veg: FuelType, speed: WindSpeed) -> Matrix {
    let rows: &[&[f64]] = match (veg, speed) {
        (FuelType::Unburnable, WindSpeed::Um) => {
            &[&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]]
        }
        (FuelType::Unburnable, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0, 0.0],
        ],

        (FuelType::Low, WindSpeed::Um) => {
            &[&[0.01, 0.01, 0.01], &[0.01, 0.0, 0.01], &[0.01, 0.02, 0.01]]
        }
        (FuelType::Low, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.01, 0.0, 0.01, 0.0],
            &[0.0, 0.02, 0.03, 0.02, 0.0],
            &[0.0, 0.01, 0.02, 0.01, 0.0],
        ],

        (FuelType::Medium, WindSpeed::Um) => {
            &[&[0.01, 0.02, 0.01], &[0.02, 0.0, 0.02], &[0.02, 0.03, 0.02]]
        }
        (FuelType::Medium, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.02, 0.0, 0.02, 0.0],
            &[0.0, 0.03, 0.04, 0.03, 0.0],
            &[0.0, 0.01, 0.03, 0.01, 0.0],
        ],

        (FuelType::High, WindSpeed::Um) => {
            &[&[0.01, 0.02, 0.01], &[0.03, 0.0, 0.03], &[0.03, 0.04, 0.03]]
        }
        (FuelType::High, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.03, 0.0, 0.03, 0.0],
            &[0.0, 0.04, 0.05, 0.04, 0.0],
            &[0.0, 0.01, 0.04, 0.01, 0.0],
        ],
    };

    rows.iter().map(|r| r.to_vec()).collect()
}
