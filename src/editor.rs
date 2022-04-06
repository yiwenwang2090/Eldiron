
use crate::tileselector::TileSelectorWidget;
use crate::editor::areaoptions::AreaOptions;
use crate::editor::behavioroptions::BehaviorOptions;
use crate::editor::behavior_overview_options::BehaviorOverviewOptions;
use crate::editor::areawidget::AreaWidget;
use crate::widget:: {ScreenWidget, Widget, WidgetState, WidgetKey};
use crate::tileset::TileUsage;

use crate::editor::dialog::DialogWidget;

use server::asset::Asset;

mod toolbar;
mod nodegraph;
mod tilemapoptions;
mod tilemapwidget;
mod areawidget;
mod areaoptions;
mod behavioroptions;
mod behavior_overview_options;
mod node;
mod node_preview;
mod statusbar;
pub mod dialog;

use crate::editor::toolbar::ToolBar;
use tilemapwidget::TileMapWidget;

use crate::context::ScreenContext;
use crate::editor::node::NodeUserData;
use crate::editor::node::NodeWidget;
use crate::editor::nodegraph::{ NodeGraph, GraphMode, GraphType };

use self::dialog::{DialogState, DialogEntry};
use self::tilemapoptions::TileMapOptions;
use self::statusbar::StatusBar;

#[derive (PartialEq)]
enum EditorState {
    TilesOverview,
    TilesDetail,
    AreaOverview,
    AreaDetail,
    BehaviorOverview,
    BehaviorDetail
}

/// The Editor struct
pub struct Editor {
    rect                            : (usize, usize, usize, usize),
    state                           : EditorState,
    context                         : ScreenContext,
    toolbar                         : ToolBar,

    tilemap_options                 : TileMapOptions,
    tilemap                         : TileMapWidget,

    area_options                    : AreaOptions,
    area_widget                     : AreaWidget,
    area_tile_selector              : TileSelectorWidget,

    behavior_options                : BehaviorOptions,
    behavior_overview_options       : BehaviorOverviewOptions,

    node_graph_tiles                : NodeGraph,
    node_graph_areas                : NodeGraph,
    node_graph_behavior             : NodeGraph,
    node_graph_behavior_details     : NodeGraph,

    left_width                      : usize,
    mouse_pos                       : (usize, usize),
    mouse_hover_pos                 : (usize, usize),

    dialog                          : DialogWidget,

    status_bar                      : StatusBar,
}

impl ScreenWidget for Editor {

    fn new(asset: &Asset, width: usize, height: usize) -> Self where Self: Sized {

        let left_width = 180_usize;
        let context = ScreenContext::new(width, height);

        let toolbar = ToolBar::new(vec!(), (0,0, width, context.toolbar_height), asset, &context);

        // Tile views and nodes

        let tilemap_options = TileMapOptions::new(vec!(), (0, context.toolbar_height, left_width, height - context.toolbar_height), asset, &context);
        let tilemap = TileMapWidget::new(vec!(), (left_width, context.toolbar_height, width - left_width, height - context.toolbar_height), asset, &context);

        let mut tile_nodes = vec![];
        for (index, t) in asset.tileset.maps_names.iter().enumerate() {
            let node = NodeWidget::new(vec![t.to_string()], NodeUserData { position: (100, 50 + 150 * index as isize) });
            tile_nodes.push(node);
        }

        let node_graph_tiles = NodeGraph::new(vec!(), (0, context.toolbar_height, width, height - context.toolbar_height), asset, &context, GraphType::Tiles, tile_nodes);

        // Area views and nodes

        let area_options = AreaOptions::new(vec!(), (0, context.toolbar_height, left_width, height - context.toolbar_height), asset, &context);
        let area_widget = AreaWidget::new(vec!(), (left_width, context.toolbar_height, width - left_width, height - context.toolbar_height - 250), asset, &context);
        let mut area_tile_selector = TileSelectorWidget::new(vec!(), (left_width, area_widget.rect.1 + area_widget.rect.3, width - left_width, 250), asset, &context);
        area_tile_selector.set_tile_type(vec![TileUsage::Environment, TileUsage::EnvBlocking, TileUsage::Water], None, &asset);

        let mut area_nodes = vec![];
        for (index, t) in context.data.areas_names.iter().enumerate() {
            let node = NodeWidget::new(vec![t.to_string()], NodeUserData { position: (100, 50 + 150 * index as isize)});
            area_nodes.push(node);
        }

        let node_graph_areas = NodeGraph::new(vec!(), (0, context.toolbar_height, width, height - context.toolbar_height), asset, &context, GraphType::Areas, area_nodes);

        // Behavior nodegraph

        let behavior_options = BehaviorOptions::new(vec!(), (0, context.toolbar_height, left_width, height - context.toolbar_height), asset, &context);

        let behavior_overview_options = BehaviorOverviewOptions::new(vec!(), (0, context.toolbar_height, left_width, height - context.toolbar_height), asset, &context);

        let mut behavior_nodes = vec![];
        for (index, behavior_name) in context.data.behaviors_names.iter().enumerate() {
            let mut node = NodeWidget::new(vec![behavior_name.to_string()],
             NodeUserData { position: (100, 50 + 150 * index as isize) });

            let node_menu_atom = crate::atom::AtomWidget::new(vec!["Rename".to_string(), "Delete".to_string()], crate::atom::AtomWidgetType::NodeMenu, crate::atom::AtomData::new_as_int("menu".to_string(), 0));
            node.menu = Some(node_menu_atom);

            behavior_nodes.push(node);
        }
        let node_graph_behavior = NodeGraph::new(vec!(), (left_width, context.toolbar_height, width - left_width, height - context.toolbar_height), asset, &context, GraphType::Behavior, behavior_nodes);

        let mut node_graph_behavior_details = NodeGraph::new(vec!(), (left_width, context.toolbar_height, width - left_width, height - context.toolbar_height), asset, &context, GraphType::Behavior, vec![]);

        node_graph_behavior_details.set_mode(GraphMode::Detail, &context);

        let dialog = DialogWidget::new(asset, &context);

        Self {
            rect                    : (0, 0, width, height),
            state                   : EditorState::TilesOverview,
            context,
            toolbar,

            tilemap_options,
            tilemap,

            area_options,
            area_widget,
            area_tile_selector,

            behavior_options,
            behavior_overview_options,

            node_graph_tiles,
            node_graph_areas,
            node_graph_behavior,
            node_graph_behavior_details,

            dialog,

            left_width,
            mouse_pos               : (0,0),
            mouse_hover_pos         : (0,0),

            status_bar              : StatusBar::new(),
        }
    }

    /// Update the editor
    fn update(&mut self) {
        if self.state == EditorState::BehaviorDetail {
            self.node_graph_behavior_details.update(&mut self.context);
        }
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.context.width = width; self.rect.2 = width;
        self.context.height = height; self.rect.3 = height;
        self.toolbar.resize(width, height, &self.context);

        self.tilemap_options.resize(self.left_width, height - self.context.toolbar_height, &self.context);
        self.tilemap.resize(width - self.left_width, height - self.context.toolbar_height, &self.context);

        self.area_options.resize(self.left_width, height - self.context.toolbar_height, &self.context);
        self.area_widget.rect = (self.left_width, self.context.toolbar_height, width - self.left_width, height - self.context.toolbar_height - 250);
        self.area_tile_selector.rect = (self.left_width, self.area_widget.rect.1 + self.area_widget.rect.3, width - self.left_width, 250);
        self.area_tile_selector.resize(width - self.left_width, 250);

        self.behavior_options.resize(self.left_width, height - self.context.toolbar_height, &self.context);
        self.behavior_overview_options.resize(self.left_width, height - self.context.toolbar_height, &self.context);

        self.node_graph_tiles.resize(width, height - self.context.toolbar_height, &self.context);
        self.node_graph_areas.resize(width, height - self.context.toolbar_height, &self.context);
        self.node_graph_behavior.resize(width - self.left_width, height - self.context.toolbar_height, &self.context);
        self.node_graph_behavior_details.resize(width - self.left_width, height - self.context.toolbar_height, &self.context);
    }

    fn draw(&mut self, frame: &mut [u8], anim_counter: usize, asset: &mut Asset) {

        //let start = self.get_time();

        self.toolbar.draw(frame, anim_counter, asset, &mut self.context);

        if self.state == EditorState::TilesOverview {
            self.node_graph_tiles.draw(frame, anim_counter, asset, &mut self.context);
        } else
        if self.state == EditorState::TilesDetail {
            self.tilemap_options.draw(frame, anim_counter, asset, &mut self.context);
            self.tilemap.draw(frame, anim_counter, asset, &mut self.context);
        } else
        if self.state == EditorState::AreaOverview {
            self.node_graph_areas.draw(frame, anim_counter, asset, &mut self.context);
        } else
        if self.state == EditorState::AreaDetail {
            self.area_options.draw(frame, anim_counter, asset, &mut self.context);
            self.area_widget.draw(frame, anim_counter, asset, &mut self.context);
            self.area_tile_selector.draw(frame, self.context.width, anim_counter, asset, &mut self.context);
        } else
        if self.state == EditorState::BehaviorOverview {
            self.behavior_overview_options.draw(frame, anim_counter, asset, &mut self.context);
            self.node_graph_behavior.draw(frame, anim_counter, asset, &mut self.context);
        } else
        if self.state == EditorState::BehaviorDetail {
            self.behavior_options.draw(frame, anim_counter, asset, &mut self.context);
            self.node_graph_behavior_details.draw(frame, anim_counter, asset, &mut self.context);
            self.status_bar.draw(frame, anim_counter, asset, &mut self.context);
        }

        // Drag and drop
        if let Some(drag_context) = &self.context.drag_context {
            if let Some(mut buffer) = drag_context.buffer {
                self.context.draw2d.blend_slice_safe(frame, &mut buffer[..], &(self.mouse_pos.0 as isize - drag_context.offset.0, self.mouse_pos.1 as isize - drag_context.offset.1, 180, 32), self.context.width, &self.rect);
            }
        }

        // Dialog
        if self.context.dialog_state != DialogState::Closed {
            self.dialog.rect.0 = (self.context.width - self.dialog.rect.2) / 2;
            self.dialog.draw(frame, anim_counter, asset, &mut self.context);
        } else
        if self.context.dialog_entry != DialogEntry::None {
            if self.state == EditorState::BehaviorDetail {
                if self.context.dialog_entry == DialogEntry::NodeTile {
                    self.node_graph_behavior_details.set_node_atom_data(self.context.dialog_node_behavior_id.clone(), self.context.dialog_node_behavior_value.clone(), &mut self.context);
                } else {
                    self.node_graph_behavior_details.update_from_dialog(&mut self.context);
                }
            } else
            if self.state == EditorState::BehaviorOverview {
                if self.context.dialog_entry == DialogEntry::NewName && self.context.dialog_accepted == true {
                    //println!("dialog ended {} {}", self.context.dialog_new_name, self.context.dialog_new_name_type);
                    self.context.data.create_behavior(self.context.dialog_new_name.clone(), 0);

                    let mut node = NodeWidget::new(vec![self.context.dialog_new_name.clone()],
                    NodeUserData { position: (100, 50 + 150 * self.node_graph_behavior.nodes.len() as isize) });

                    let node_menu_atom = crate::atom::AtomWidget::new(vec!["Rename".to_string(), "Delete".to_string()], crate::atom::AtomWidgetType::NodeMenu, crate::atom::AtomData::new_as_int("menu".to_string(), 0));
                    node.menu = Some(node_menu_atom);

                    self.node_graph_behavior.nodes.push(node);
                    self.node_graph_behavior.dirty = true;
                    self.toolbar.widgets[0].text = self.context.data.behaviors_names.clone();
                    self.toolbar.widgets[0].dirty = true;
                } else {
                    if self.context.dialog_entry == DialogEntry::NodeName {
                        if self.context.dialog_accepted == true {
                            if let Some(behavior) = self.context.data.behaviors.get_mut(&self.context.data.behaviors_ids[self.context.curr_behavior_index]) {
                                behavior.rename(self.context.dialog_node_behavior_value.4.clone());
                            }
                        }
                    }
                    self.node_graph_behavior.update_from_dialog(&mut self.context);
                }
            }
            self.context.dialog_entry = DialogEntry::None;
        }

        // Draw overlay
        self.toolbar.draw_overlay(frame, &self.rect, anim_counter, asset, &mut self.context);

        //let stop = self.get_time();
        //println!("{:?}", stop - start);
    }

    fn key_down(&mut self, char: Option<char>, key: Option<WidgetKey>, asset: &mut Asset) -> bool {
        if self.context.dialog_state == DialogState::Open {
            return self.dialog.key_down(char, key, asset, &mut self.context);
        }
        false
    }

    fn mouse_down(&mut self, pos: (usize, usize), asset: &mut Asset) -> bool {

        if self.context.dialog_state == DialogState::Open {
            return self.dialog.mouse_down(pos, asset, &mut self.context);
        }

        let mut consumed = false;
        if self.toolbar.mouse_down(pos, asset, &mut self.context) {

            // Tile Button
            if self.toolbar.widgets[1].clicked {
                if self.toolbar.widgets[1].selected {
                    self.node_graph_tiles.set_mode_and_rect( GraphMode::Overview, (0, self.rect.1 + self.context.toolbar_height, self.rect.2, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::TilesOverview;
                    self.node_graph_tiles.mark_all_dirty();
                } else
                if self.toolbar.widgets[1].right_selected {
                    self.node_graph_tiles.set_mode_and_rect( GraphMode::Detail, (self.left_width, self.rect.1 + self.context.toolbar_height, self.rect.2 - self.left_width, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::TilesDetail;
                }

                self.toolbar.widgets[2].selected = false;
                self.toolbar.widgets[2].right_selected = false;
                self.toolbar.widgets[3].selected = false;
                self.toolbar.widgets[3].right_selected = false;
                self.toolbar.widgets[2].dirty = true;
                self.toolbar.widgets[3].dirty = true;

                self.toolbar.widgets[0].text = asset.tileset.maps_names.clone();
                self.toolbar.widgets[0].curr_index = self.context.curr_tileset_index;
                self.toolbar.widgets[0].dirty = true;
            } else
            // Area Button
            if self.toolbar.widgets[2].clicked {
                if self.toolbar.widgets[2].selected {
                    self.node_graph_areas.set_mode_and_rect( GraphMode::Overview, (0, self.rect.1 + self.context.toolbar_height, self.rect.2, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::AreaOverview;
                    self.node_graph_areas.mark_all_dirty();
                } else
                if self.toolbar.widgets[2].right_selected {
                    self.node_graph_areas.set_mode_and_rect( GraphMode::Detail, (self.left_width, self.rect.1 + self.context.toolbar_height, self.rect.2 - self.left_width, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::AreaDetail;
                }

                self.toolbar.widgets[1].selected = false;
                self.toolbar.widgets[1].right_selected = false;
                self.toolbar.widgets[3].selected = false;
                self.toolbar.widgets[3].right_selected = false;
                self.toolbar.widgets[1].dirty = true;
                self.toolbar.widgets[3].dirty = true;

                self.toolbar.widgets[0].text = self.context.data.areas_names.clone();
                self.toolbar.widgets[0].curr_index = self.context.curr_area_index;
                self.toolbar.widgets[0].dirty = true;
            } else
            // Behavior Button
            if self.toolbar.widgets[3].clicked {
                if self.toolbar.widgets[3].selected {
                    self.node_graph_behavior.set_mode_and_rect( GraphMode::Overview, (self.left_width, self.rect.1 + self.context.toolbar_height, self.rect.2 - self.left_width, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::BehaviorOverview;
                    self.node_graph_behavior.mark_all_dirty();
                } else
                if self.toolbar.widgets[3].right_selected {
                    self.node_graph_behavior.set_mode_and_rect( GraphMode::Detail, (self.left_width, self.rect.1 + self.context.toolbar_height, self.rect.2 - self.left_width, self.rect.3 - self.context.toolbar_height), &self.context);
                    self.state = EditorState::BehaviorDetail;
                    self.node_graph_behavior_details.set_behavior_id(self.context.data.behaviors_ids[self.context.curr_behavior_index] , &mut self.context);
                }

                self.toolbar.widgets[1].selected = false;
                self.toolbar.widgets[1].right_selected = false;
                self.toolbar.widgets[1].dirty = true;
                self.toolbar.widgets[2].selected = false;
                self.toolbar.widgets[2].right_selected = false;
                self.toolbar.widgets[2].dirty = true;

                self.toolbar.widgets[0].text = self.context.data.behaviors_names.clone();
                self.toolbar.widgets[0].curr_index = self.context.curr_behavior_index;
                self.toolbar.widgets[0].dirty = true;
            }
            consumed = true;
        }

        if consumed == false && self.state == EditorState::TilesOverview {
            if consumed == false && self.node_graph_tiles.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
                if self.node_graph_tiles.clicked {
                    self.toolbar.widgets[0].curr_index = self.context.curr_tileset_index;
                    self.toolbar.widgets[0].dirty = true;
                    self.tilemap.set_tilemap_id(asset.tileset.maps_ids[self.context.curr_tileset_index]);
                    self.node_graph_tiles.clicked = false;
                }
            }
        } else
        if consumed == false && self.state == EditorState::TilesDetail {
            if consumed == false && self.tilemap_options.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.tilemap.mouse_down(pos, asset, &mut self.context) {
                if self.tilemap.clicked == true {
                    self.tilemap_options.adjust_tile_usage(asset, &self.context);
                }
                if self.context.curr_tile.is_some() {
                    self.tilemap_options.set_state(WidgetState::Normal);
                } else {
                    self.tilemap_options.set_state(WidgetState::Disabled);
                }
                consumed = true;
            }
        } else
        if consumed == false && self.state == EditorState::AreaOverview {
            if consumed == false && self.node_graph_areas.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
                if self.node_graph_areas.clicked {
                    self.toolbar.widgets[0].curr_index = self.context.curr_area_index;
                    self.toolbar.widgets[0].dirty = true;
                    self.area_widget.set_area_id(self.context.data.areas_ids[self.context.curr_area_index]);
                    self.node_graph_areas.clicked = false;
                }
            }
        } else
        if consumed == false && self.state == EditorState::AreaDetail {
            for atom in &mut self.area_options.widgets {
                if atom.mouse_down(pos, asset, &mut self.context) {
                    if atom.clicked {
                        if atom.atom_data.id == "Tilemaps" {
                            if atom.curr_index == 0 {
                                self.area_tile_selector.set_tile_type(vec![TileUsage::Environment, TileUsage::EnvBlocking, TileUsage::Water], None, &asset);
                            } else {
                                self.area_tile_selector.set_tile_type(vec![TileUsage::Environment, TileUsage::EnvBlocking, TileUsage::Water], Some(atom.curr_index - 1), &asset);
                            }
                        } else
                        if atom.atom_data.id == "remap" {
                            if let Some(area) = self.context.data.areas.get_mut(&self.area_widget.area_id) {
                                area.remap(asset);
                            }
                        }
                    }
                    consumed = true;
                }
            }
            if consumed == false && self.area_tile_selector.mouse_down(pos, asset, &mut self.context) {
                consumed = true;

                if let Some(selected) = &self.area_tile_selector.selected {
                    self.context.curr_area_tile = Some(selected.clone());
                } else {
                    self.context.curr_area_tile = None;
                }
            }
            if consumed == false && self.area_widget.mouse_down(pos, asset, &mut self.context) {

                if let Some(clicked) = self.area_widget.clicked {
                    if let Some(selected) = &self.area_tile_selector.selected {

                        if let Some(area) = self.context.data.areas.get_mut(&self.area_widget.area_id) {
                            area.set_value(clicked, selected.clone());
                            area.save_data();
                        }
                    }

                }
                consumed = true;
            }
        }
        if consumed == false && self.state == EditorState::BehaviorOverview {
            if consumed == false && self.behavior_overview_options.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.node_graph_behavior.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
                if self.node_graph_behavior.clicked {
                    self.toolbar.widgets[0].curr_index = self.context.curr_behavior_index;
                    self.toolbar.widgets[0].dirty = true;
                    self.node_graph_behavior.clicked = false;
                }
            }
        }
        if consumed == false && self.state == EditorState::BehaviorDetail {
            if consumed == false && self.behavior_options.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.node_graph_behavior_details.mouse_down(pos, asset, &mut self.context) {
                consumed = true;
            }
        }

        consumed
    }

    fn mouse_up(&mut self, pos: (usize, usize), asset: &mut Asset) -> bool {

        if self.context.dialog_state == DialogState::Open {
            return self.dialog.mouse_up(pos, asset, &mut self.context);
        }

        let mut consumed = false;
        if self.toolbar.mouse_up(pos, asset, &mut self.context) {

            if self.toolbar.widgets[0].new_selection.is_some() {
                if self.state == EditorState::TilesOverview || self.state == EditorState::TilesDetail {
                    self.node_graph_tiles.changed_selection(self.context.curr_tileset_index, self.toolbar.widgets[0].curr_index);
                    self.context.curr_tileset_index = self.toolbar.widgets[0].curr_index;
                    self.tilemap.set_tilemap_id(asset.tileset.maps_ids[self.context.curr_tileset_index]);
                    self.context.curr_tile = None;
                    self.tilemap_options.set_state(WidgetState::Disabled);
                } else
                if self.state == EditorState::AreaOverview || self.state == EditorState::AreaDetail {
                    self.node_graph_areas.changed_selection(self.context.curr_area_index, self.toolbar.widgets[0].curr_index);
                    self.context.curr_area_index = self.toolbar.widgets[0].curr_index;
                    self.area_widget.set_area_id(self.context.data.areas_ids[self.context.curr_area_index]);
                } else
                if self.state == EditorState::BehaviorOverview || self.state == EditorState::BehaviorDetail {
                    self.node_graph_behavior.changed_selection(self.context.curr_behavior_index, self.toolbar.widgets[0].curr_index);
                    self.context.curr_behavior_index = self.toolbar.widgets[0].curr_index;
                    self.node_graph_behavior_details.set_behavior_id(self.context.data.behaviors_ids[self.context.curr_behavior_index] , &mut self.context);
                }
                self.toolbar.widgets[0].new_selection = None;
            }
            consumed = true;
        }

        if self.state == EditorState::TilesOverview {
            if consumed == false && self.node_graph_tiles.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::TilesDetail {
            if self.tilemap_options.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
            if self.tilemap.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorOverview {
            if self.behavior_overview_options.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
            if self.node_graph_behavior.mouse_up(pos, asset, &mut self.context) {
                consumed = true;

                // In case a behavior was deleted
                if self.toolbar.widgets[0].text.len() != self.context.data.behaviors_names.len() {
                    self.toolbar.widgets[0].text = self.context.data.behaviors_names.clone();
                    self.context.curr_behavior_index = 0;
                    self.toolbar.widgets[0].dirty = true;
                    self.toolbar.widgets[0].curr_index = 0;
                }
            }
        } else
        if self.state == EditorState::BehaviorDetail {
            if self.behavior_options.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
            if self.node_graph_behavior_details.mouse_up(pos, asset, &mut self.context) {
                consumed = true;
            }
        }

        // Node Drag ?
        if let Some(drag_context) = &self.context.drag_context {

            if self.state == EditorState::BehaviorOverview {
                if self.context.contains_pos_for(pos, self.node_graph_behavior.rect) {

                    let mut position = (pos.0 as isize, pos.1 as isize);
                    position.0 -= self.node_graph_behavior.rect.0 as isize + self.node_graph_behavior.offset.0 + drag_context.offset.0;
                    position.1 -= self.node_graph_behavior.rect.1 as isize + self.node_graph_behavior.offset.1 + drag_context.offset.1;

                    self.context.dialog_state = DialogState::Opening;
                    self.context.dialog_height = 0;
                    self.context.target_fps = 60;
                    self.context.dialog_entry = DialogEntry::NewName;
                    self.context.dialog_new_name = "New Behavior".to_string();
                    self.context.dialog_new_name_type = format!("NewBehavior_{}", drag_context.text);
                    self.context.dialog_new_node_position = position;
                }
            } else
            if self.state == EditorState::BehaviorDetail {
                if self.context.contains_pos_for(pos, self.node_graph_behavior_details.rect) {
                    let mut position = (pos.0 as isize, pos.1 as isize);
                    position.0 -= self.node_graph_behavior_details.rect.0 as isize + self.node_graph_behavior_details.offset.0 + drag_context.offset.0;
                    position.1 -= self.node_graph_behavior_details.rect.1 as isize + self.node_graph_behavior_details.offset.1 + drag_context.offset.1;

                    self.node_graph_behavior_details.add_node_of_name(drag_context.text.clone(), position, &mut self.context);
                }
            }
            // Cleanup DnD
            self.context.drag_context = None;
            self.context.target_fps = self.context.default_fps;
            consumed = true;
        }
        consumed
    }

    fn mouse_dragged(&mut self, pos: (usize, usize), asset: &mut Asset) -> bool {

        if self.context.dialog_state == DialogState::Open {
            return self.dialog.mouse_dragged(pos, asset, &mut self.context);
        }

        let mut consumed = false;
        self.toolbar.mouse_dragged(pos, asset, &mut self.context);

        if self.state == EditorState::TilesOverview {
            if consumed == false && self.node_graph_tiles.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::TilesDetail {
            if self.tilemap_options.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
            if self.tilemap.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::AreaOverview {
            if consumed == false && self.node_graph_areas.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::AreaDetail {
            if consumed == false && self.area_widget.mouse_dragged(pos, asset, &mut self.context) {

                if let Some(clicked) = self.area_widget.clicked {
                    if let Some(selected) = &self.area_tile_selector.selected {

                        if let Some(area) = self.context.data.areas.get_mut(&self.area_widget.area_id) {
                            area.set_value(clicked, selected.clone());
                            area.save_data();
                        }
                    }
                }
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorOverview {
            if consumed == false && self.behavior_overview_options.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.node_graph_behavior.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorDetail {
            if consumed == false && self.behavior_options.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.node_graph_behavior_details.mouse_dragged(pos, asset, &mut self.context) {
                consumed = true;
            }
        }
        self.mouse_pos = pos.clone();
        consumed
    }

    fn mouse_hover(&mut self, pos: (usize, usize), asset: &mut Asset) -> bool {

        if self.context.dialog_state == DialogState::Open {
            return self.dialog.mouse_hover(pos, asset, &mut self.context);
        }

        let mut consumed = false;
        self.mouse_hover_pos = pos.clone();

        if consumed == false && self.toolbar.mouse_hover(pos, asset, &mut self.context) {
            consumed = true;
        } else
        if self.state == EditorState::TilesDetail {
            if consumed == false && self.tilemap_options.mouse_hover(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::AreaDetail {
            if consumed == false && self.area_options.mouse_hover(pos, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorDetail {
            if consumed == false && self.node_graph_behavior_details.mouse_hover(pos, asset, &mut self.context) {
                consumed = true;
            }
        }
        consumed
    }

    fn mouse_wheel(&mut self, delta: (isize, isize), asset: &mut Asset) -> bool {

        if self.context.dialog_state == DialogState::Open {
            return self.dialog.mouse_wheel(delta, asset, &mut self.context);
        }

        let mut consumed = false;
        if consumed == false && self.toolbar.mouse_wheel(delta, asset, &mut self.context) {
            consumed = true;
        } else
        if self.state == EditorState::TilesOverview {
            if consumed == false && self.node_graph_tiles.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::TilesDetail {
            if consumed == false && self.tilemap.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::AreaOverview {
            if consumed == false && self.node_graph_areas.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::AreaDetail {
            if consumed == false && self.context.contains_pos_for(self.mouse_hover_pos,self.area_widget.rect) && self.area_widget.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
            if consumed == false && self.context.contains_pos_for(self.mouse_hover_pos,self.area_tile_selector.rect) && self.area_tile_selector.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorOverview {
            if consumed == false && self.node_graph_behavior.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        } else
        if self.state == EditorState::BehaviorDetail {
            if consumed == false && self.node_graph_behavior_details.mouse_wheel(delta, asset, &mut self.context) {
                consumed = true;
            }
        }
        consumed
    }

    fn get_target_fps(&self) -> usize {
        self.context.target_fps
    }
}