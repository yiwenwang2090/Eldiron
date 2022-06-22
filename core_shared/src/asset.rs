
pub mod tileset;

use std::{path::PathBuf, collections::HashMap};

use fontdue::Font;

pub use tileset::*;

#[cfg(feature = "embed_binaries")]
use core_embed_binaries::Embedded;

pub struct Asset {
    pub tileset                 : TileSet,

    pub game_fonts              : HashMap<String, Font>,
    pub editor_fonts            : HashMap<String, Font>,
}

impl Asset  {

    pub fn new() -> Self where Self: Sized {

        Self {
            tileset             : tileset::TileSet::new(),
            game_fonts          : HashMap::new(),
            editor_fonts        : HashMap::new(),
        }
    }

    /// Load editor font
    pub fn load_editor_font<'a>(&mut self, name: String, resource_name: String) {
        let path = std::path::Path::new("resources").join(resource_name);

        if let Some(font_bytes) = std::fs::read(path).ok() {
            if let Some(font) = Font::from_bytes(font_bytes, fontdue::FontSettings::default()).ok() {
                self.editor_fonts.insert(name, font);
            }
        }
    }

    pub fn get_editor_font(&self, name: &str) -> &Font {
        self.editor_fonts.get(name).unwrap()
    }

    /// Load from the given file path
    pub fn load_from_path(&mut self, path: PathBuf) {
        self.tileset = tileset::TileSet::load_from_path(path.clone());

        // Collect the fonts

        let font_path = path.join("assets").join("fonts");
        let paths = std::fs::read_dir(font_path).unwrap();

        for path in paths {
            // Generate the tile map for this dir element
            let path = &path.unwrap().path();

            if path.is_file() {//&& path.extension().map(|s| s == "ttf").unwrap_or(false) {

                if let Some(font_bytes) = std::fs::read(path).ok() {
                if let Some(font) = Font::from_bytes(font_bytes, fontdue::FontSettings::default()).ok() {
                        self.game_fonts.insert(path.file_stem().unwrap().to_os_string().into_string().unwrap(), font);
                    }
                }
            }
        }
    }

    #[cfg(feature = "embed_binaries")]
    /// Load from embedded binaries
    pub fn load_from_embedded(&mut self) {
        //#[cfg(feature = "embed_binaries")]
        {
            //let index_html = Embedded::get("Cargo.toml").unwrap();
            //println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));
        }

        self.tileset = tileset::TileSet::load_from_embedded();

        for file in Embedded::iter() {
            let name = file.as_ref();
            if name.starts_with("assets/fonts/") {
                if let Some(font_bytes) = Embedded::get(name) {
                    if let Some(font) = Font::from_bytes(font_bytes.data, fontdue::FontSettings::default()).ok() {
                        let buf = std::path::Path::new(name);
                        self.game_fonts.insert(buf.file_stem().unwrap().to_os_string().into_string().unwrap(), font);
                    }
                }
            }
        }
    }

    /// Returns the tilemap of the given id
    pub fn get_map_of_id(&self, id: usize) -> &TileMap {
        &self.tileset.maps[&id]
    }

    /// Returns the tile fo the given id
    pub fn get_tile(&self, id: &(usize, usize, usize)) -> Tile {
        let map = self.get_map_of_id(id.0);
        map.get_tile(&(id.1, id.2))
    }
}
