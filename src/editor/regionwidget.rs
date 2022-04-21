use server::asset::{ Asset };
use server::asset::tileset::TileUsage;

use crate::widget::context::ScreenContext;
use crate::editor::TileSelectorWidget;
use crate::editor::RegionOptions;

use super::regionoptions::RegionEditorMode;

pub struct RegionWidget {
    pub rect                : (usize, usize, usize, usize),
    pub region_id           : usize,

    grid_size               : usize,

    offset                  : (isize, isize),
    screen_offset           : (usize, usize),

    pub tile_selector       : TileSelectorWidget,

    mouse_hover_pos         : (usize, usize),
    pub clicked             : Option<(isize, isize)>,
}

impl RegionWidget {

    pub fn new(_text: Vec<String>, rect: (usize, usize, usize, usize), asset: &Asset, context: &ScreenContext) -> Self {

        let mut tile_selector = TileSelectorWidget::new(vec!(), (rect.0, rect.1 + rect.3 - 250, rect.2, 250), asset, &context);
        tile_selector.set_tile_type(vec![TileUsage::Environment, TileUsage::EnvBlocking, TileUsage::Water], None, &asset);

        Self {
            rect,
            region_id               : 0,
            grid_size               : 32,

            offset                  : (0, 0),
            screen_offset           : (0, 0),

            tile_selector,

            mouse_hover_pos         : (0, 0),
            clicked                 : None,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize, _context: &ScreenContext) {
        self.rect.2 = width;
        self.rect.3 = height;

        self.tile_selector.rect = (self.rect.0, self.rect.1 + self.rect.3 - 250, width, 250);
        self.tile_selector.resize(width, 250);
    }

    pub fn draw(&mut self, frame: &mut [u8], anim_counter: usize, asset: &mut Asset, context: &mut ScreenContext) {
        context.draw2d.draw_rect(frame, &self.rect, context.width, &[0,0,0,255]);

        let mut rect = self.rect;
        rect.3 -= 250;
        let grid_size = self.grid_size;

        let left_offset = (self.rect.2 % grid_size) / 2;
        let top_offset = (self.rect.3 % grid_size) / 2;

        self.screen_offset = (left_offset, top_offset);

        //let grid = (rect.2 / grid_size, rect.3 / grid_size);
        //let max_tiles = grid.0 * grid.1;

        if let Some(region) = context.data.regions.get(&self.region_id) {

            let x_tiles = (rect.2 / grid_size) as isize;
            let y_tiles = (rect.3 / grid_size) as isize;

            for y in 0..y_tiles {
                for x in 0..x_tiles {
                    if let Some(value) = region.get_value((x - self.offset.0, y - self.offset.1)) {
                        let pos = (rect.0 + left_offset + (x as usize) * grid_size, rect.1 + top_offset + (y as usize) * grid_size);

                        let map = asset.get_map_of_id(value.0);
                        context.draw2d.draw_animated_tile(frame, &pos, map,context.width,&(value.1, value.2), anim_counter, grid_size);
                    }
                }
            }
        }

        self.tile_selector.draw(frame, context.width, anim_counter, asset, context);
    }

    pub fn mouse_down(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext, region_options: &mut RegionOptions) -> bool {

        if context.contains_pos_for(pos, self.tile_selector.rect) {
            if self.tile_selector.mouse_down(pos, asset, context) {
                return true;
            }
        }

        if context.contains_pos_for(pos, self.rect) {

            let grid_size = self.grid_size;

            let x = ((pos.0 - self.rect.0 - self.screen_offset.0) / grid_size) as isize - self.offset.0;
            let y = ((pos.1 - self.rect.1 - self.screen_offset.0) / grid_size) as isize - self.offset.1;

            self.clicked = Some((x, y));

            let editor_mode = region_options.get_editor_mode();

            if editor_mode == RegionEditorMode::Tiles {
                if let Some(selected) = &self.tile_selector.selected {
                    if let Some(region) = context.data.regions.get_mut(&self.region_id) {
                        region.set_value((x,y), selected.clone());
                        region.save_data();
                    }
                }
            } else
            if editor_mode == RegionEditorMode::Areas {

            }

            return true;
        }

        if self.tile_selector.mouse_down(pos, asset, context) {

            if let Some(selected) = &self.tile_selector.selected {
                context.curr_region_tile = Some(selected.clone());
            } else {
                context.curr_region_tile = None;
            }
        }

        false
    }

    pub fn mouse_up(&mut self, _pos: (usize, usize), _asset: &mut Asset, _context: &mut ScreenContext, _region_options: &mut RegionOptions) -> bool {
        self.clicked = None;

        false
    }

    pub fn mouse_hover(&mut self, pos: (usize, usize), _asset: &mut Asset, _context: &mut ScreenContext, _region_options: &mut RegionOptions) -> bool {
        self.mouse_hover_pos = pos.clone();
        false
    }

    pub fn mouse_dragged(&mut self, pos: (usize, usize), _asset: &mut Asset, context: &mut ScreenContext, region_options: &mut RegionOptions) -> bool {
        if context.contains_pos_for(pos, self.rect) {

            let grid_size = self.grid_size;

            let x = ((pos.0 - self.rect.0 - self.screen_offset.0) / grid_size) as isize - self.offset.0;
            let y = ((pos.1 - self.rect.1 - self.screen_offset.0) / grid_size) as isize - self.offset.1;

            if self.clicked != Some((x, y)) {

                self.clicked = Some((x, y));
                let editor_mode = region_options.get_editor_mode();

                if editor_mode == RegionEditorMode::Tiles {
                    if let Some(selected) = &self.tile_selector.selected {
                        if let Some(region) = context.data.regions.get_mut(&self.region_id) {
                            region.set_value((x,y), selected.clone());
                            region.save_data();
                        }
                    }
                }
            }

            return true;
        }
        false
    }

    pub fn mouse_wheel(&mut self, delta: (isize, isize), asset: &mut Asset, context: &mut ScreenContext) -> bool {
        if context.contains_pos_for(self.mouse_hover_pos, self.tile_selector.rect) && self.tile_selector.mouse_wheel(delta, asset, context) {
        } else {
            self.offset.0 -= delta.0 / self.grid_size as isize;
            self.offset.1 += delta.1 / self.grid_size as isize;
        }
        true
    }

    /// Sets a new map index
    pub fn set_region_id(&mut self, id: usize, context: &mut ScreenContext, region_options: &mut RegionOptions) {
        self.region_id = id;

        if let Some(region) = context.data.regions.get_mut(&self.region_id) {
            region_options.area_widgets[0].text = region.get_area_names();
            region_options.area_widgets[0].dirty = true;
        }
    }
}