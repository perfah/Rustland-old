use std::cell::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::cmp;
use std::u16;


use rustwlc::*;

use wmstate::*;
use definitions::LayoutElemID;
use layout::*;
use layout::element::bisect::*;
use super::LayoutElement;
use layout::arrangement::*;

pub struct Workspace{
    active_desktop: usize,
    desktops: Vec<LayoutElemID> 
}

impl Workspace{
    pub fn init(tree: &mut LayoutTree, no_partitions: usize) -> Workspace{
        assert!(no_partitions > 0, "A workspace element is expected to contain at least 1 partition.");
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for i in 0..no_partitions{
            children.push(tree.spawn_element())
        }
        
        Workspace{
            active_desktop: 1,
            desktops: children
        }
    }

    pub fn get_active_child_id(&self) -> LayoutElemID {
        match self.desktops.get(self.active_desktop as usize){
            Some(active_desktop) => { *active_desktop },
            None => { panic!("Invalid desktop: {}", self.active_desktop); }
        }   
    }

    pub fn set_active_desktop(&mut self, desktop: i16){
        if desktop <= usize::min_value() as i16{
            self.active_desktop = 0
        }
        else if desktop >= self.desktops.len() as i16 {
            self.active_desktop = self.desktops.len() - 1;
        }
        else{
            self.active_desktop = desktop as usize;
        }
    }

    pub fn next_desktop(&mut self){
        let active_desktop = self.active_desktop;
        self.set_active_desktop(active_desktop as i16 + 1)
    }
    pub fn prev_desktop(&mut self){
        let active_desktop = self.active_desktop;
        self.set_active_desktop(active_desktop as i16 - 1)
    }

    pub fn get_all_children(&self) -> &Vec<LayoutElemID> {
        &self.desktops
    }

    pub fn get_offset_geometry(&self, tree: &LayoutTree, outer_geometry: Geometry, child: u16) -> Geometry{
        let offset = 
            if child == self.active_desktop as u16{
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
