use std::cell::*;
use std::borrow::Borrow;
use std::collections::HashMap;

use rustwlc::*;

use wmstate::*;
use definitions::LayoutElemID;
use layout::*;
use layout::element::segmentation::*;
use super::LayoutElement;
use layout::arrangement::*;

pub struct Workspace{
    pub active_child: u16,
    children: Vec<LayoutElemID> 
}

impl Workspace{
    pub fn init(tree: &mut LayoutTree, no_partitions: usize) -> Workspace{
        assert!(no_partitions > 0, "A workspace element is expected to contain at least 1 partition.");
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for i in 0..no_partitions{
            children.push(tree.spawn_element())
        }
        
        Workspace{
            active_child: 1,
            children: children
        }
    }

    pub fn get_active_child_id(&self) -> LayoutElemID {
        match self.children.get(self.active_child as usize){
            Some(active_child) => { *active_child },
            None => { panic!("Internal erorr!"); }
        }
        
    }

    pub fn get_all_children(&self) -> &Vec<LayoutElemID>
    {
        &self.children
    }

    pub fn get_offset(&self, tree: &LayoutTree, outer_geometry: Geometry, child: u16) -> Geometry
    {
        let offset = 
            if child == self.active_child {
                Geometry::zero()
            }
            else{
                tree.get_outer_geometry()
            };

        Geometry{
            origin: Point{
                x: outer_geometry.origin.x + offset.size.w as i32,
                y: outer_geometry.origin.y + offset.size.h as i32
            },
            size: outer_geometry.size
        }
    }
}
