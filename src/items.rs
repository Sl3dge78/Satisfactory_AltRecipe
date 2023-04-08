use macroquad::prelude::*; 
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::ItemTextureMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredients {
    pub name: Item,
    pub nb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub product: Item,
    pub input: Vec<Ingredients>,
    pub rate: f32,
}

impl Recipe {
    pub async fn load(&mut self, texs: &mut ItemTextureMap) {
        self.product.load(texs).await;
        for inp in &mut self.input {
            inp.name.load(texs).await;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String")]
pub struct Item {
    pub name: String,
    #[serde(skip)]
    pub texture: Option<Texture2D>,
}

impl From<String> for Item {
    fn from(value: String) -> Self {
        Self {
            name: value,
            texture: None
        }
    }
}

impl Item {
    pub async fn load(&mut self, texs: &mut ItemTextureMap) {
        self.texture = match texs.get_mut(&self.name as &str) {
            None => None,
            Some(v) => {
                match *v {
                    Some(_) => *v,
                    None => {
                        let path = format!("res/images/{}", IMAGE_MAP.get(&self.name as &str).unwrap());
                        match load_texture(&path).await {
                            Ok(tex) => Some(tex),
                            Err(e) => {
                                error!("Unable to load {}: {}", path, e);
                                None
                            },
                        }
                    }
                }
            }
        };

    }
}

lazy_static! {
pub static ref IMAGE_MAP: HashMap<&'static str, &'static str> = HashMap::from([
    ("Iron Ore", "Iron_Ore.png"),
    ("Copper Ore", "Copper_Ore.png"),
    ("Iron Plate", "Iron_Plate.png"),
    ("Screw", "Screw.png"),
    ("Wire", "Wire.png"),
    ("Iron Ingot", "Iron_Ingot.png"),
    ("Sulfur", "Sulfur.png"),
    ("Compacted Coal", "Compacted_Coal.png"),
    ("Caterium Ingot", "Caterium_Ingot.png"),
    ("Copper Ingot", "Copper_Ingot.png"),
    ("Silica", "Silica.png"),
    ("Limestone", "Limestone.png"),
    ("Raw Quartz", "Raw_Quartz.png"),
    ("Reinforced Iron Plate", "Reinforced_Iron_Plate.png"),
    ("Copper Sheet", "Copper_Sheet.png"),
    ("Steel Ingot", "Steel_Ingot.png"),
    ("Steel Pipe", "Steel_Pipe.png"),
    ("Steel Beam", "Steel_Beam.png"),
    ("Coal", "Coal.png"),
    ("Crystal Oscillator", "Crystal_Oscillator.png"),
    ("Biomass", "Biomass.png"),
    ("Wood", "Wood.png"),
    ("Water", "Water.png"),
    ("Caterium Ore", "Caterium_Ore.png"),
    ("Concrete", "Concrete.png"),
    ("Stator", "Stator.png"),
    ("High-Speed Connector", "High-Speed_Connector.png"),
    ("Quickwire", "Quickwire.png"),
    ("Rotor", "Rotor.png"),
    ("Modular Frame", "Modular_Frame.png"),
    ("Encased Industrial Beam", "Encased_Industrial_Beam.png"),
    ("Rubber", "Rubber.png"),
    ("Motor", "Motor.png"),
    ("Iron Rod", "Iron_Rod.png"),
    ("Circuit Board", "Circuit_Board.png"),
    ("Heavy Oil Residue", "Heavy_Oil_Residue.png"),
    ("Petroleum Coke", "Petroleum_Coke.png"),
    ("Crude Oil", "Crude_Oil.png"),
    ("Plastic", "Plastic.png"),
    ("Packaged Water", "Packaged_Water.png"),
    ("Fuel", "Fuel.png"),
    ("Quartz Crystal", "Quartz_Crystal.png"),
    ("AI Limiter", "AI_Limiter.png"),
    ("Alclad Aluminum Sheet", "Alclad_Aluminum_Sheet.png"),
    ("Electromagnetic Control Rod", "Electromagnetic_Control_Rod.png"),
    ("Radio Control Unit", "Radio_Control_Unit.png"),
    ("Cooling System", "Cooling_System.png"),
    ("Computer", "Computer.png"),
    ("Battery", "Battery.png"),
    ("Bauxite", "Bauxite.png"),
    ("Aluminum Ingot", "Aluminum_Ingot.png"),
    ("Aluminum Scrap", "Aluminum_Scrap.png"),
    ("Alumina Solution", "Alumina_Solution.png"),
    ("Aluminum Casing", "Aluminum_Casing.png"),
    ("Sulfuric Acid", "Sulfuric_Acid.png"),
    ("Heat Sink", "Heat_Sink.png"),
    ("Nitrogen Gas", "Nitrogen_Gas.png"),
    ("Uranium", "Uranium.png"),
    ("Encased Uranium Cell", "Encased_Uranium_Cell.png"),
    ("Beacon", "Beacon.png"),
    ("Non-fissile Uranium", "Non-fissile_Uranium.png"),
    ("Heavy Modular Frame", "Heavy_Modular_Frame.png"),
    ("Nitric Acid", "Nitric_Acid.png"),
    ("Uranium Waste", "Uranium_Waste.png"),
    ("Encased Plutonium Cell", "Encased_Plutonium_Cell.png"),
    ("Pressure Conversion Cube", "Pressure_Conversion_Cube.png"),
    ("Packaged Nitrogen Gas", "Packaged_Nitrogen_Gas.png"),
    ("Iron Ingot", "Iron_Ingot.png"),
    ("Reinforced Iron Plate", "Reinforced_Iron_Plate.png"),
    ("Reinforced Iron Plate", "Reinforced_Iron_Plate.png"),
    ("Screw", "Screw.png"),
    ("Wire", "Wire.png"),
    ("Black Powder", "Black_Powder.png"),
    ("Wire", "Wire.png"),
    ("Wire", "Wire.png"),
    ("Quickwire", "Quickwire.png"),
    ("Concrete", "Concrete.png"),
    ("Silica", "Silica.png"),
    ("Modular Frame", "Modular_Frame.png"),
    ("Rotor", "Rotor.png"),
    ("Iron Rod", "Iron_Rod.png"),
    ("Modular Frame", "Modular_Frame.png"),
    ("Rotor", "Rotor.png"),
    ("Screw", "Screw.png"),
    ("Steel Ingot", "Steel_Ingot.png"),
    ("Steel Ingot", "Steel_Ingot.png"),
    ("Beacon", "Beacon.png"),
    ("Coal", "Coal.png"),
    ("Coal", "Coal.png"),
    ("Concrete", "Concrete.png"),
    ("Copper Ingot", "Copper_Ingot.png"),
    ("Copper Sheet", "Copper_Sheet.png"),
    ("Iron Ingot", "Iron_Ingot.png"),
    ("Caterium Ingot", "Caterium_Ingot.png"),
    ("Quartz Crystal", "Quartz_Crystal.png"),
    ("Encased Industrial Beam", "Encased_Industrial_Beam.png"),
    ("Automated Wiring", "Automated_Wiring.png"),
    ("Stator", "Stator.png"),
    ("Motor", "Motor.png"),
    ("Empty Canister", "Empty_Canister.png"),
    ("Empty Canister", "Empty_Canister.png"),
    ("Heavy Modular Frame", "Heavy_Modular_Frame.png"),
    ("Heavy Modular Frame", "Heavy_Modular_Frame.png"),
    ("Portable Miner", "Portable_Miner.png"),
    ("Computer", "Computer.png"),
    ("Computer", "Computer.png"),
    ("Cable", "Cable.png"),
    ("Cable", "Cable.png"),
    ("Circuit Board", "Circuit_Board.png"),
    ("Concrete", "Concrete.png"),
    ("Heavy Oil Residue", "Heavy_Oil_Residue.png"),
    ("Iron Plate", "Iron_Plate.png"),
    ("Iron Plate", "Iron_Plate.png"),
    ("Packaged Fuel", "Packaged_Fuel.png"),
    ("Plastic", "Plastic.png"),
    ("Polymer Resin", "Polymer_Resin.png"),
    ("Reinforced Iron Plate", "Reinforced_Iron_Plate.png"),
    ("Rubber", "Rubber.png"),
    ("Smart Plating", "Smart_Plating.png"),
    ("Steel Ingot", "Steel_Ingot.png"),
    ("Versatile Framework", "Versatile_Framework.png"),
    ("Cable", "Cable.png"),
    ("Circuit Board", "Circuit_Board.png"),
    ("High-Speed Connector", "High-Speed_Connector.png"),
    ("Crystal Oscillator", "Crystal_Oscillator.png"),
    ("Circuit Board", "Circuit_Board.png"),
    ("Turbofuel", "Turbofuel.png"),
    ("Battery", "Battery.png"),
    ("Motor", "Motor.png"),
    ("Supercomputer", "Supercomputer.png"),
    ("Supercomputer", "Supercomputer.png"),
    ("Alumina Solution", "Alumina_Solution.png"),
    ("Aluminum Casing", "Aluminum_Casing.png"),
    ("Aluminum Ingot", "Aluminum_Ingot.png"),
    ("Aluminum Scrap", "Aluminum_Scrap.png"),
    ("Fuel", "Fuel.png"),
    ("Radio Control Unit", "Radio_Control_Unit.png"),
    ("Aluminum Scrap", "Aluminum_Scrap.png"),
    ("Turbofuel", "Turbofuel.png"),
    ("Cooling System", "Cooling_System.png"),
    ("Heat Sink", "Heat_Sink.png"),
    ("Radio Control Unit", "Radio_Control_Unit.png"),
    ("Turbo Motor", "Turbo_Motor.png"),
    ("Electromagnetic Control Rod", "Electromagnetic_Control_Rod.png"),
    ("Encased Uranium Cell", "Encased_Uranium_Cell.png"),
    ("Uranium Fuel Rod", "Uranium_Fuel_Rod.png"),
    ("Encased Plutonium Cell", "Encased_Plutonium_Cell.png"),
    ("Fused Modular Frame", "Fused_Modular_Frame.png"),
    ("Non-fissile Uranium", "Non-fissile_Uranium.png"),
    ("Plutonium Fuel Rod", "Plutonium_Fuel_Rod.png"),
    ("Turbo Motor", "Turbo_Motor.png"),
]);
}
