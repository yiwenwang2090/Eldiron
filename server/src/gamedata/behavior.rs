use serde::{Deserialize, Serialize};

use std::fs;
use std::path;
use std::path::PathBuf;

use std::collections::HashMap;

use crate::asset::tileset::TileUsage;

#[derive(Serialize, Deserialize)]
pub struct GameBehaviorData {
    #[serde(with = "vectorize")]
    pub tiles           : HashMap<(isize, isize), (usize, usize, usize, TileUsage)>,
    pub id              : usize,
    pub curr_pos        : (isize, isize),
    pub min_pos         : (isize, isize),
    pub max_pos         : (isize, isize),
}

pub struct GameBehavior {
    pub name            : String,
    pub path            : PathBuf,
    pub data            : GameBehaviorData,
}

impl GameBehavior {
    pub fn new(path: &PathBuf) -> Self {

        let name = path::Path::new(&path).file_stem().unwrap().to_str().unwrap();

        // Gets the content of the settings file
        let json_path = path.join( format!("{}{}", name, ".json"));
        let contents = fs::read_to_string( json_path )
            .unwrap_or("".to_string());

        // Construct the json settings
        let data = serde_json::from_str(&contents)
            .unwrap_or(GameBehaviorData { tiles: HashMap::new(), id: 0, curr_pos: (0,0), min_pos: (0,0), max_pos: (0,0) });

        Self {
            name        : name.to_string(),
            path        : path.clone(),
            data,
        }
    }

    /// Save the TileAreaData to file
    pub fn save_data(&self) {
        let json_path = self.path.join( format!("{}{}", self.name, ".json"));
        let json = serde_json::to_string(&self.data).unwrap();
        fs::write(json_path, json)
            .expect("Unable to write area file");
    }
}