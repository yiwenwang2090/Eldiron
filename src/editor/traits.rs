use crate::editor::node::NodeWidget;
use server::gamedata::behavior::*;
use server::asset::Asset;

use crate::editor::ScreenContext;
use crate::WidgetState;

#[allow(unused)]
pub trait EditorOptions {

    fn new(_text: Vec<String>, rect: (usize, usize, usize, usize), asset: &Asset, context: &ScreenContext) -> Self where Self: Sized;

        fn test(&mut self) {
            println!("test");
        }

    fn resize(&mut self, width: usize, height: usize, _context: &ScreenContext);

    fn draw(&mut self, frame: &mut [u8], anim_counter: usize, asset: &mut Asset, context: &mut ScreenContext, content: &mut Option<Box<dyn EditorContent>>);

    fn mouse_down(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext, content: &mut Option<Box<dyn EditorContent>>) -> bool;

    fn mouse_up(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext) -> bool;

    fn mouse_dragged(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext) -> bool;

    fn mouse_wheel(&mut self, delta: (isize, isize), asset: &mut Asset, context: &mut ScreenContext) -> bool { false }

    fn mouse_hover(&mut self, pos: (usize, usize), _asset: &mut Asset, _context: &mut ScreenContext) -> bool { false }

    // Sets the state of the atom widgets
    fn set_state(&mut self, state: WidgetState) {}


    // For TilemapOptions

    /// Updates the group widget based on the selected tile
    fn adjust_tile_usage(&mut self, asset: &Asset, context: &ScreenContext) {}

    /// Sets the tile anim for the current tile
    fn set_anim(&mut self, asset: &mut Asset, context: &ScreenContext) {}

    /// Clears the tile anim for the current tile
    fn clear_anim(&mut self, asset: &mut Asset, context: &ScreenContext) {}

    /// Sets the default tile for the current map
    fn set_default_tile(&mut self, asset: &mut Asset, context: &ScreenContext) {}

    /// Set the tile tags
    fn set_tags(&mut self, tags: String, asset: &mut Asset, context: &ScreenContext) {}
}

#[derive(PartialEq)]
pub enum GraphMode {
    Overview,
    Detail
}

#[allow(unused)]
pub trait EditorContent {

    fn new(_text: Vec<String>, rect: (usize, usize, usize, usize), behavior_type: BehaviorType, asset: &Asset, context: &ScreenContext) -> Self where Self: Sized;

    fn resize(&mut self, width: usize, height: usize, _context: &ScreenContext);

    fn draw(&mut self, frame: &mut [u8], anim_counter: usize, asset: &mut Asset, context: &mut ScreenContext, options: &mut Option<Box<dyn EditorOptions>>);

    fn mouse_down(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext, options: &mut Option<Box<dyn EditorOptions>>) -> bool;

    fn mouse_up(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext) -> bool;

    fn mouse_dragged(&mut self, pos: (usize, usize), asset: &mut Asset, context: &mut ScreenContext) -> bool;

    fn mouse_wheel(&mut self, delta: (isize, isize), asset: &mut Asset, context: &mut ScreenContext) -> bool;

    fn mouse_hover(&mut self, pos: (usize, usize), _asset: &mut Asset, _context: &mut ScreenContext) -> bool { false }


    // For TileMapWidget

    // Set the current tilemap id
    fn set_tilemap_id(&mut self, id: usize) {}

    /// Converts a screen position to a map grid position
    fn screen_to_map(&self, asset: &Asset, screen_pos: (usize, usize)) -> Option<(usize, usize)> { None }

    // For NodeGraphs

    fn update(&mut self, context: &mut ScreenContext) {}

    fn set_mode(&mut self, mode: GraphMode, context: &ScreenContext) {}
    fn set_mode_and_rect(&mut self, mode: GraphMode, rect: (usize, usize, usize, usize), context: &ScreenContext) {}
    fn set_mode_and_nodes(&mut self, mode: GraphMode, nodes: Vec<NodeWidget>, _context: &ScreenContext) {}

    /// Returns the rectangle for the given node either in relative or absolute coordinates
    fn get_node_rect(&self, node_index: usize, relative: bool) -> (isize, isize, usize, usize) { (0,0,0,0) }

    /// Updates a node value from the dialog
    fn update_from_dialog(&mut self, context: &mut ScreenContext) {}

    /// Marks the two nodes as dirty
    fn changed_selection(&mut self, old_selection: usize, new_selection: usize) {}

    /// Mark all nodes as dirty
    fn mark_all_dirty(&mut self) {}

    /// Set the behavior id, this will take the bevhavior node data and create the node widgets
    fn set_behavior_id(&mut self, id: usize, context: &mut ScreenContext) {}

    /// Adds a node of the type identified by its name
    fn add_node_of_name(&mut self, name: String, position: (isize, isize), context: &mut ScreenContext) {}

    /// Inits the node widget (atom widgets, id)
    fn init_node_widget(&mut self, behavior_data: &GameBehaviorData, node_data: &BehaviorNode, node_widget: &mut NodeWidget, context: &ScreenContext) {}

    /// Sets up the corner node widget
    fn setup_corner_node_widget(&mut self, behavior_data: &GameBehaviorData, node_data: &BehaviorNode, node_widget: &mut NodeWidget, context: &ScreenContext) {}

    /// Converts the index of a node widget to a node id
    fn widget_index_to_node_id(&self, index: usize) -> usize { 0 }

    /// Converts the id of a node to a widget index
    fn node_id_to_widget_index(&self, id: usize) -> usize { 0 }

    /// Returns true if the node connector is a source connector (Right or Bottom)
    fn connector_is_source(&self, connector: BehaviorNodeConnector) -> bool { false }

    /// Disconnect the node from all connections
    fn disconnect_node(&mut self, id: usize, context: &mut ScreenContext) {}

    /// Disconnect the node from all connections
    fn delete_node(&mut self, id: usize, context: &mut ScreenContext) {}

    /// Sets the widget and behavior data for the given atom id
    fn set_node_atom_data(&mut self, node_atom_id: (usize, usize, String), data: (f64, f64, f64, f64, String), context: &mut ScreenContext) {}

    /// Checks the visibility of a node
    fn check_node_visibility(&mut self, context: &ScreenContext) {}

    /// Marks all connected nodes as visible
    fn mark_connections_visible(&mut self, id: usize, context: &ScreenContext) {}

    /// Checks if the given node id is part of an unconnected branch.
    fn belongs_to_standalone_branch(&mut self, id: usize, context: &ScreenContext) -> bool { false }

    /// Collects the children indices of the given node id so that they can all be dragged at once
    fn collect_drag_children_indices(&mut self, id: usize, context: &ScreenContext) {}

    /// Returns the behavior id for the current behavior and graph type
    fn get_curr_behavior_id(&self, context: &ScreenContext) -> usize { 0 }

    /// Returns the current node id for the given graph type
    fn get_curr_node_id(&self, context: &ScreenContext) -> usize { 0 }
}