use macroquad::color::Color;

use crate::automaton::ifiresim::fuel::FuelType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MapBiomasClass {
    // Floresta
    Forest = 1,
    ForestFormation = 3,
    FloodableForest = 6,
    SavannaFormation = 4,
    FloodedSavanna = 7,
    Mangrove = 5,
    WoodedSandbank = 49,

    // Vegetação Herbácea e Arbustiva
    HerbaceousOrShrubbyVegetation = 10,
    Grassland = 12,
    HerbaceousAndShrubFormation = 77,
    Wetland = 11,
    SaltMarsh = 84,
    HerbaceousSandbank = 50,
    HypersalineTidalFlat = 32,
    RockyOutcrop = 29,

    // Agropecuária
    Farming = 14,
    Pasture = 15,
    Agriculture = 18,
    TemporaryCrop = 19,
    Soybean = 39,
    SugarCane = 20,
    Rice = 40,
    Cotton = 62,
    OtherTemporaryCrops = 41,
    PerennialCrop = 36,
    Coffee = 46,
    Citrus = 47,
    PalmOil = 35,
    OtherPerennialCrops = 48,
    ForestPlantation = 9,
    MosaicOfUses = 21,

    // Área não vegetada
    NonVegetatedArea = 22,
    BeachDuneAndSandSpot = 23,
    UrbanArea = 24,
    Mining = 30,
    PhotovoltaicPowerPlant = 75,
    WindFarm = 91,
    OtherNonVegetatedAreas = 25,

    // Corpo D'água
    Water = 26,
    RiverLakeAndOcean = 33,
    Aquaculture = 31,
}

pub const ALL: [MapBiomasClass; 41] = [
    MapBiomasClass::Forest,
    MapBiomasClass::ForestFormation,
    MapBiomasClass::FloodableForest,
    MapBiomasClass::SavannaFormation,
    MapBiomasClass::FloodedSavanna,
    MapBiomasClass::Mangrove,
    MapBiomasClass::WoodedSandbank,
    MapBiomasClass::HerbaceousOrShrubbyVegetation,
    MapBiomasClass::Grassland,
    MapBiomasClass::HerbaceousAndShrubFormation,
    MapBiomasClass::Wetland,
    MapBiomasClass::SaltMarsh,
    MapBiomasClass::HerbaceousSandbank,
    MapBiomasClass::HypersalineTidalFlat,
    MapBiomasClass::RockyOutcrop,
    MapBiomasClass::Farming,
    MapBiomasClass::Pasture,
    MapBiomasClass::Agriculture,
    MapBiomasClass::TemporaryCrop,
    MapBiomasClass::Soybean,
    MapBiomasClass::SugarCane,
    MapBiomasClass::Rice,
    MapBiomasClass::Cotton,
    MapBiomasClass::OtherTemporaryCrops,
    MapBiomasClass::PerennialCrop,
    MapBiomasClass::Coffee,
    MapBiomasClass::Citrus,
    MapBiomasClass::PalmOil,
    MapBiomasClass::OtherPerennialCrops,
    MapBiomasClass::ForestPlantation,
    MapBiomasClass::MosaicOfUses,
    MapBiomasClass::NonVegetatedArea,
    MapBiomasClass::BeachDuneAndSandSpot,
    MapBiomasClass::UrbanArea,
    MapBiomasClass::Mining,
    MapBiomasClass::PhotovoltaicPowerPlant,
    MapBiomasClass::WindFarm,
    MapBiomasClass::OtherNonVegetatedAreas,
    MapBiomasClass::Water,
    MapBiomasClass::RiverLakeAndOcean,
    MapBiomasClass::Aquaculture,
];

impl MapBiomasClass {
    pub fn name(self) -> &'static str {
        match self {
            MapBiomasClass::Forest => "Floresta",
            MapBiomasClass::ForestFormation => "Formação Florestal",
            MapBiomasClass::FloodableForest => "Floresta Alagável",
            MapBiomasClass::SavannaFormation => "Formação Savânica",
            MapBiomasClass::FloodedSavanna => "Savana Alagada",
            MapBiomasClass::Mangrove => "Mangue",
            MapBiomasClass::WoodedSandbank => "Restinga Arborizada",
            MapBiomasClass::HerbaceousOrShrubbyVegetation => "Veg. Herbácea/Arbustiva",
            MapBiomasClass::Grassland => "Campo",
            MapBiomasClass::HerbaceousAndShrubFormation => "Formação Herbácea/Arbustiva",
            MapBiomasClass::Wetland => "Área Úmida",
            MapBiomasClass::SaltMarsh => "Marisma",
            MapBiomasClass::HerbaceousSandbank => "Restinga Herbácea",
            MapBiomasClass::HypersalineTidalFlat => "Apicum",
            MapBiomasClass::RockyOutcrop => "Afloramento Rochoso",
            MapBiomasClass::Farming => "Lavoura",
            MapBiomasClass::Pasture => "Pastagem",
            MapBiomasClass::Agriculture => "Agricultura",
            MapBiomasClass::TemporaryCrop => "Lavoura Temporária",
            MapBiomasClass::Soybean => "Soja",
            MapBiomasClass::SugarCane => "Cana-de-açúcar",
            MapBiomasClass::Rice => "Arroz",
            MapBiomasClass::Cotton => "Algodão",
            MapBiomasClass::OtherTemporaryCrops => "Outras Lavouras Temporárias",
            MapBiomasClass::PerennialCrop => "Lavoura Perene",
            MapBiomasClass::Coffee => "Café",
            MapBiomasClass::Citrus => "Citrus",
            MapBiomasClass::PalmOil => "Dendê",
            MapBiomasClass::OtherPerennialCrops => "Outras Lavouras Perenes",
            MapBiomasClass::ForestPlantation => "Silvicultura",
            MapBiomasClass::MosaicOfUses => "Mosaico de Usos",
            MapBiomasClass::NonVegetatedArea => "Área Não Vegetada",
            MapBiomasClass::BeachDuneAndSandSpot => "Praia, Duna e Areal",
            MapBiomasClass::UrbanArea => "Área Urbana",
            MapBiomasClass::Mining => "Mineração",
            MapBiomasClass::PhotovoltaicPowerPlant => "Usina Fotovoltaica",
            MapBiomasClass::WindFarm => "Parque Eólico",
            MapBiomasClass::OtherNonVegetatedAreas => "Outras Áreas Não Vegetadas",
            MapBiomasClass::Water => "Água",
            MapBiomasClass::RiverLakeAndOcean => "Rio, Lago e Oceano",
            MapBiomasClass::Aquaculture => "Aquicultura",
        }
    }

    /// Incremento da exposição acumulada quando a célula não pega fogo.
    pub fn exposure_increment(&self) -> f64 {
        match self.into() {
            FuelType::Low => 0.02,
            FuelType::Medium => 0.04,
            FuelType::High => 0.06,
            FuelType::Unburnable => 0.,
        }
    }

    /// Teto da exposição acumulada.
    pub fn exposure_cap(&self) -> f64 {
        match self.into() {
            FuelType::Low => 0.08,
            FuelType::Medium => 0.11,
            FuelType::High => 0.14,
            FuelType::Unburnable => 0.,
        }
    }

    /// Tempo de deterioração (gerações queimando até virar queimada).
    ///
    /// sfb.py:73 (baixa = 3), sfb.py:115 (média = 2), sfb.py:157 (alta = 1).
    pub fn deterioration(&self) -> i32 {
        match self.into() {
            FuelType::Low => 3,
            FuelType::Medium => 2,
            FuelType::High => 1,
            FuelType::Unburnable => 0,
        }
    }
}

impl TryFrom<u8> for MapBiomasClass {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Forest),
            3 => Ok(Self::ForestFormation),
            4 => Ok(Self::SavannaFormation),
            5 => Ok(Self::Mangrove),
            6 => Ok(Self::FloodableForest),
            7 => Ok(Self::FloodedSavanna),
            9 => Ok(Self::ForestPlantation),
            10 => Ok(Self::HerbaceousOrShrubbyVegetation),
            11 => Ok(Self::Wetland),
            12 => Ok(Self::Grassland),
            14 => Ok(Self::Farming),
            15 => Ok(Self::Pasture),
            18 => Ok(Self::Agriculture),
            19 => Ok(Self::TemporaryCrop),
            20 => Ok(Self::SugarCane),
            21 => Ok(Self::MosaicOfUses),
            22 => Ok(Self::NonVegetatedArea),
            23 => Ok(Self::BeachDuneAndSandSpot),
            24 => Ok(Self::UrbanArea),
            25 => Ok(Self::OtherNonVegetatedAreas),
            26 => Ok(Self::Water),
            29 => Ok(Self::RockyOutcrop),
            30 => Ok(Self::Mining),
            31 => Ok(Self::Aquaculture),
            32 => Ok(Self::HypersalineTidalFlat),
            33 => Ok(Self::RiverLakeAndOcean),
            35 => Ok(Self::PalmOil),
            36 => Ok(Self::PerennialCrop),
            39 => Ok(Self::Soybean),
            40 => Ok(Self::Rice),
            41 => Ok(Self::OtherTemporaryCrops),
            46 => Ok(Self::Coffee),
            47 => Ok(Self::Citrus),
            48 => Ok(Self::OtherPerennialCrops),
            49 => Ok(Self::WoodedSandbank),
            50 => Ok(Self::HerbaceousSandbank),
            62 => Ok(Self::Cotton),
            75 => Ok(Self::PhotovoltaicPowerPlant),
            77 => Ok(Self::HerbaceousAndShrubFormation),
            84 => Ok(Self::SaltMarsh),
            91 => Ok(Self::WindFarm),
            invalid => Err(invalid),
        }
    }
}

impl From<MapBiomasClass> for u8 {
    fn from(class: MapBiomasClass) -> Self {
        class as u8
    }
}

impl From<MapBiomasClass> for Color {
    fn from(class: MapBiomasClass) -> Self {
        match class {
            // 1. Floresta
            MapBiomasClass::Forest => Color::from_rgba(0x1F, 0x8D, 0x49, 255), // #1f8d49
            MapBiomasClass::ForestFormation => Color::from_rgba(0x1F, 0x8D, 0x49, 255), // #1f8d49
            MapBiomasClass::FloodableForest => Color::from_rgba(0x00, 0x77, 0x85, 255), // #007785
            MapBiomasClass::SavannaFormation => Color::from_rgba(0x7D, 0xC9, 0x75, 255), // #7dc975
            MapBiomasClass::FloodedSavanna => Color::from_rgba(0x22, 0x8C, 0x70, 255), // #228c70
            MapBiomasClass::Mangrove => Color::from_rgba(0x04, 0x38, 0x1D, 255), // #04381d
            MapBiomasClass::WoodedSandbank => Color::from_rgba(0x02, 0xD6, 0x59, 255), // #02d659

            // 2. Vegetação Herbácea e Arbustiva
            MapBiomasClass::HerbaceousOrShrubbyVegetation => {
                Color::from_rgba(0xD6, 0xBC, 0x74, 255)
            } // #d6bc74
            MapBiomasClass::Grassland => Color::from_rgba(0xD6, 0xBC, 0x74, 255), // #d6bc74
            MapBiomasClass::HerbaceousAndShrubFormation => Color::from_rgba(0x86, 0xB0, 0x74, 255), // #86b074
            MapBiomasClass::Wetland => Color::from_rgba(0x51, 0x97, 0x99, 255), // #519799
            MapBiomasClass::SaltMarsh => Color::from_rgba(0x81, 0xDB, 0xBF, 255), // #81dbbf
            MapBiomasClass::HerbaceousSandbank => Color::from_rgba(0xFF, 0xAA, 0x5F, 255), // #ffaa5f
            MapBiomasClass::HypersalineTidalFlat => Color::from_rgba(0xFC, 0x81, 0x14, 255), // #fc8114
            MapBiomasClass::RockyOutcrop => Color::from_rgba(0xAD, 0x51, 0x00, 255), // #ad5100

            // 3. Agropecuária
            MapBiomasClass::Farming => Color::from_rgba(0xFF, 0xEF, 0xC3, 255), // #ffefc3
            MapBiomasClass::Pasture => Color::from_rgba(0xED, 0xDE, 0x8E, 255), // #edde8e
            MapBiomasClass::Agriculture => Color::from_rgba(0xE9, 0x74, 0xED, 255), // #E974ED
            MapBiomasClass::TemporaryCrop => Color::from_rgba(0xC2, 0x7B, 0xA0, 255), // #C27BA0
            MapBiomasClass::Soybean => Color::from_rgba(0xF5, 0xB3, 0xC8, 255), // #f5b3c8
            MapBiomasClass::SugarCane => Color::from_rgba(0xDB, 0x70, 0x93, 255), // #db7093
            MapBiomasClass::Rice => Color::from_rgba(0xC7, 0x15, 0x85, 255),    // #c71585
            MapBiomasClass::Cotton => Color::from_rgba(0xFF, 0x69, 0xB4, 255),  // #ff69b4
            MapBiomasClass::OtherTemporaryCrops => Color::from_rgba(0xF5, 0x4C, 0xA9, 255), // #f54ca9
            MapBiomasClass::PerennialCrop => Color::from_rgba(0xD0, 0x82, 0xDE, 255), // #d082de
            MapBiomasClass::Coffee => Color::from_rgba(0xD6, 0x8F, 0xE2, 255),        // #d68fe2
            MapBiomasClass::Citrus => Color::from_rgba(0x99, 0x32, 0xCC, 255),        // #9932cc
            MapBiomasClass::PalmOil => Color::from_rgba(0x90, 0x65, 0xD0, 255),       // #9065d0
            MapBiomasClass::OtherPerennialCrops => Color::from_rgba(0xE6, 0xCC, 0xFF, 255), // #e6ccff
            MapBiomasClass::ForestPlantation => Color::from_rgba(0x7A, 0x59, 0x00, 255), // #7a5900
            MapBiomasClass::MosaicOfUses => Color::from_rgba(0xFF, 0xEF, 0xC3, 255),     // #ffefc3

            // 4. Área não vegetada
            MapBiomasClass::NonVegetatedArea => Color::from_rgba(0xD4, 0x27, 0x1E, 255), // #d4271e
            MapBiomasClass::BeachDuneAndSandSpot => Color::from_rgba(0xFF, 0xA0, 0x7A, 255), // #ffa07a
            MapBiomasClass::UrbanArea => Color::from_rgba(0xD4, 0x27, 0x1E, 255), // #d4271e
            MapBiomasClass::Mining => Color::from_rgba(0x9C, 0x00, 0x27, 255),    // #9c0027
            MapBiomasClass::PhotovoltaicPowerPlant => Color::from_rgba(0x75, 0x72, 0x72, 255), // #757272
            MapBiomasClass::WindFarm => Color::from_rgba(0x40, 0x3D, 0x3E, 255), // #403d3e
            MapBiomasClass::OtherNonVegetatedAreas => Color::from_rgba(0xDB, 0x4D, 0x4F, 255), // #db4d4f

            // 5. Corpo D'água
            MapBiomasClass::Water => Color::from_rgba(0x25, 0x32, 0xE4, 255), // #2532e4
            MapBiomasClass::RiverLakeAndOcean => Color::from_rgba(0x25, 0x32, 0xE4, 255), // #2532e4
            MapBiomasClass::Aquaculture => Color::from_rgba(0x09, 0x10, 0x77, 255), // #091077
        }
    }
}
