use std::u16;
use num::clamp;

use common::definitions::{LayoutElemID, FPS};
use layout::LayoutTree;
use layout::element::padding::Padding;
use layout::element::LayoutElementProfile;

use wlc::*;

pub enum Direction { LEFT, RIGHT, UP, DOWN }

#[derive(Serialize, Deserialize, Clone)]
pub struct Grid{
    active_subspace: usize,
    columns: usize,
    subspace_element_ids: Vec<LayoutElemID>,
    urgent_subspace_updates: Vec<usize>
}

impl Grid{
    pub fn init(ident: LayoutElemID, tree: &mut LayoutTree, columns: usize, rows: usize) -> (LayoutElemID, Grid) {
        assert!(columns * rows > 0, "At least one Grid is required.");
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for _ in 0..(columns * rows){
            let (child_ident, child) = Padding::init(tree.spawn_dummy_element(Some(ident)), tree, 200, None);
            tree.reserve_element_identity(child_ident, LayoutElementProfile::Padding(child));
            children.push(child_ident);
        }
        
        let profile = Grid{
            active_subspace: 0,
            columns: columns,
            subspace_element_ids: children,
            urgent_subspace_updates: Vec::with_capacity(2)
        };

        (ident, profile)
    }

    pub fn get_active_child_id(&self) -> LayoutElemID {
        match self.subspace_element_ids.get(self.active_subspace as usize){
            Some(active_subspace) => { *active_subspace },
            None => { panic!("Invalid desktop: {}", self.active_subspace); }
        }   
    }

    pub fn active_subspace(&self) -> usize{
        self.active_subspace
    }

    pub fn set_active_subspace(&mut self, new_subspace: i16){
        self.active_subspace = clamp(
            new_subspace as usize, 
            0usize, 
            (self.subspace_element_ids.len() - 1) as usize
        ); 
    }

    pub fn switch_to_subspace_in_direction(&mut self, direction: Direction){
        let active_subspace = self.active_subspace as i16;
        let x_tot = self.columns as i16;
        let xy_tot = self.subspace_element_ids.len() as i16;

        self.set_active_subspace(active_subspace as i16 + match direction{
            Direction::LEFT if active_subspace % x_tot > 0 => -1i16,
            Direction::RIGHT if (active_subspace + 1i16) % x_tot > 0 => 1i16,
            Direction::UP if (active_subspace - x_tot) >= 0 => -x_tot,
            Direction::DOWN if (active_subspace + x_tot) < xy_tot => x_tot,
            _ => 0i16
        });      
    }

    pub fn get_all_children(&self) -> &Vec<LayoutElemID> {
        &self.subspace_element_ids
    }

    pub fn children_iter(&self) -> impl Iterator<Item = &LayoutElemID> {
        self.subspace_element_ids.iter()
    }

    pub fn get_offset_geometry(&self, display_geometry: Geometry, outer_geometry: Geometry, this_desktop: u16, stacked_scale: &mut (f32, f32)) -> Geometry{
        let index = this_desktop as i32;

        Geometry{
            origin: Point{
                x: outer_geometry.origin.x + ((index % self.columns as i32) as f32 * display_geometry.size.w as f32 * (*stacked_scale).0) as i32,
                y: outer_geometry.origin.y + ((index / self.columns as i32) as f32 * display_geometry.size.h as f32 * (*stacked_scale).1) as i32
            },
            size: outer_geometry.size
    }
}
}
