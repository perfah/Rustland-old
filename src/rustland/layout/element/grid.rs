use std::u16;

use common::definitions::LayoutElemID;
use layout::LayoutTree;
use layout::element::padding::Padding;
use layout::element::LayoutElementProfile;

use wlc::*;

pub enum Direction { LEFT, RIGHT, UP, DOWN }

#[derive(Clone)]
pub struct Grid{
    active_subspace: usize,
    columns: usize,
    subspace_element_ids: Vec<LayoutElemID> 
}

impl Grid{
    pub fn init(ident: LayoutElemID, tree: &mut LayoutTree, columns: usize, rows: usize) -> (LayoutElemID, Grid) {
        assert!(columns * rows > 0, "At least one Grid is required.");
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for _ in 0..(columns * rows){
            let (child_ident, child) = Padding::init(tree.spawn_dummy_element(Some(ident)), tree, 15, None);
            tree.reserve_element_identity(child_ident, LayoutElementProfile::Padding(child));
            children.push(child_ident);
        }
        
        let profile = Grid{
            active_subspace: 0,
            columns: columns,
            subspace_element_ids: children
        };

        (ident, profile)
    }

    pub fn get_active_child_id(&self) -> LayoutElemID {
        match self.subspace_element_ids.get(self.active_subspace as usize){
            Some(active_subspace) => { *active_subspace },
            None => { panic!("Invalid desktop: {}", self.active_subspace); }
        }   
    }

    pub fn get_active_subspace(&self) -> usize{
        self.active_subspace
    }

    pub fn set_active_subspace(&mut self, new_subspace: i16){
        if new_subspace <= usize::min_value() as i16{
            self.active_subspace = 0
        }
        else if new_subspace >= self.subspace_element_ids.len() as i16 {
            self.active_subspace = self.subspace_element_ids.len() - 1;
        }
        else{
            self.active_subspace = new_subspace as usize;
        }
    }

    pub fn switch_to_subspace_in_direction(&mut self, direction: Direction){
        let active_subspace = self.active_subspace as i16;
        let desktop_columns = self.columns as i16;

        match direction{
            Direction::LEFT => self.set_active_subspace(active_subspace - 1i16),
            Direction::RIGHT => self.set_active_subspace(active_subspace + 1i16),
            Direction::UP => self.set_active_subspace(active_subspace - desktop_columns),
            Direction::DOWN => self.set_active_subspace(active_subspace + desktop_columns)
        }
    }

    pub fn get_all_children(&self) -> &Vec<LayoutElemID> {
        &self.subspace_element_ids
    }

    pub fn get_offset_geometry(&self, display_geometry: Geometry, outer_geometry: Geometry, this_desktop: u16, stacked_scale: &mut f32) -> Geometry{
        let index = this_desktop as i32;

        Geometry{
            origin: Point{
                x: outer_geometry.origin.x + ((index % self.columns as i32) as f32 * display_geometry.size.w as f32 * (*stacked_scale)) as i32,
                y: outer_geometry.origin.y + ((index / self.columns as i32) as f32 * display_geometry.size.h as f32 * (*stacked_scale)) as i32
            },
            size: outer_geometry.size
        }
    }
}
