use serde::ser::Serialize;
use serde::de::Deserialize;
use wlc::input::keyboard::Key;
use wlc::{Point, Size, Geometry};

use common::definitions::ElementReference;
use layout::PARENT_ELEMENT;
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::padding::Padding;
use layout::element::grid::Grid;
use layout::LayoutTree;
use utils::geometry::{PointExt, SizeExt, GeometryExt};

#[derive(Serialize, Deserialize)]
pub struct LayoutConfig {
    pub root_tag: String,
    pub focused_tag: String,
    pub jumper_tag: String,
    pub grid_tag: String,
    workspace_columns: usize,
    workspaces: Vec<String>,
    monitor_resolution: Size
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig { 
            root_tag: "root".to_string(),
            focused_tag: "focused".to_string(),
            jumper_tag: "jumper".to_string(),
            grid_tag: "grid".to_string(),
            workspace_columns: 3usize,
            workspaces: vec![
                "upper_left".to_string(), "upper_mid".to_string(), "upper_right".to_string(), 
                "mid_left".to_string(), "mid_mid".to_string(), "mid_right".to_string(), 
                "bottom_left".to_string(), "bottom_mid".to_string(), "bottom_right".to_string(), 
            ],
            monitor_resolution: Size::new(640u32, 480u32)
        }
    }
}

impl LayoutConfig {
    pub fn grid_width(&self) -> usize{
        self.workspace_columns
    }

    pub fn grid_height(&self) -> usize{
        self.workspaces.len() / self.workspace_columns
    }

    pub fn monitor_geometry(&self) -> Geometry{
        Geometry::new(Point::origin(), self.monitor_resolution) 
    }

    pub fn construct_tree(&self) -> LayoutTree{
        let grid_w = self.grid_width();
        let grid_h = self.grid_height();

        let mut tree = LayoutTree::init(self.monitor_geometry(), grid_w, grid_h);

        tree.tags.tag_element_on_condition(&self.root_tag, |elem_id, _| elem_id == PARENT_ELEMENT);
        tree.tags.tag_element_on_condition(&self.focused_tag, |elem_id, wm_state| elem_id == wm_state.tree.focused_id);

        // Root element
        let (root_ident, root_profile) = Padding::init(tree.spawn_dummy_element(None), &mut tree, 100, None);

        // Jumper element
        let (jumper_ident, jumper_profile) = Padding::init(root_profile.child_elem_id, &mut tree, 0, Some(Point::origin()));
        tree.tags.tag_element(&self.jumper_tag, jumper_ident);

        // Workspaces
        let (grid_ident, grid_profile) = Grid::init(jumper_profile.child_elem_id, &mut tree, grid_w, grid_h);
        tree.tags.tag_element(&self.grid_tag, grid_ident);
        for (index, child_ident) in grid_profile.children_iter().enumerate(){
            if let Some(tag) = self.workspaces.get(index){
                tree.tags.tag_element(&tag, *child_ident);
                tree.tags.tag_element("sub", *child_ident);
            }
        }

        tree.reserve_element_identity(root_ident, LayoutElementProfile::Padding(root_profile));
        tree.reserve_element_identity(jumper_ident, LayoutElementProfile::Padding(jumper_profile));
        tree.reserve_element_identity(grid_ident, LayoutElementProfile::Grid(grid_profile));

        tree.animate_property(root_ident, "gap_size", 0f32, false, 250);

        tree
    }
}