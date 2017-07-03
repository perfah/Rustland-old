use std::cell::*;
use std::sync::MutexGuard;

pub use rustwlc::types::{Geometry, Point, Size};

use wmstate::*;
use layout::*;
use definitions::{LayoutElemID};
use layout::arrangement::*;

use super::*;

pub const LOWER_SEGM_BOUND: i32 = 0;
pub const HIGHER_SEGM_BOUND: i32 = 1;

#[derive(Clone)]
pub enum Orientation{
    Horizontal,
    Vertical,
}

pub struct Bisect{
    children: Vec<LayoutElemID>,
    orientation: Orientation,
    ratio: f32,
    inner_padding: Option<u32>
}

impl Bisect{
    pub fn init(tree: &mut LayoutTree, orientation: Orientation) -> Bisect{
        let no_partitions = 2;
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for i in 0..no_partitions{
            children.push(tree.spawn_element())
        }

        Bisect{
            children: children,
            orientation: orientation,
            ratio: 1.0 / no_partitions as f32,
            inner_padding: Some(10)
        }
    }
    pub fn init_horiz_50_50(tree: &mut LayoutTree) -> Bisect{
        Bisect{
            children: vec![tree.spawn_element(), tree.spawn_element()],
            orientation: Orientation::Horizontal,
            ratio: 0.5,
            inner_padding: Some(10)
        }
    }
    pub fn init_vert_50_50(tree: &mut LayoutTree) -> Bisect{
        Bisect{
            children: vec![tree.spawn_element(), tree.spawn_element()],
            orientation: Orientation::Vertical,
            ratio: 0.5,
            inner_padding: Some(10)
        }
    }

    pub fn get_children(&self) -> &Vec<LayoutElemID>{
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<LayoutElemID>{
        &mut self.children
    }


    pub fn get_orientation(self) -> Orientation{
        self.orientation
    }

    pub fn get_offset(&self, outer_geometry: Geometry, child_index: i32) -> Geometry{
        let padding = self.inner_padding.unwrap_or(0 as u32);

        Geometry{
            origin: match self.orientation{
                Orientation::Horizontal => {  
                    Point{
                        x: outer_geometry.origin.x 
                            + child_index * (self.ratio * outer_geometry.size.w as f32) as i32
                            + if child_index == LOWER_SEGM_BOUND { padding } else { padding / 2 } as i32,
                        y: padding as i32
                    }
                }
                Orientation::Vertical => {
                    Point{
                        x: padding as i32,
                        y: outer_geometry.origin.y 
                            + child_index * (self.ratio * outer_geometry.size.h as f32) as i32
                            + if child_index == LOWER_SEGM_BOUND { padding } else { padding / 2 } as i32,
                    }
                }
            },
            size: match self.orientation
            {
                Orientation::Horizontal => {  
                    Size{
                        w: (self.ratio * outer_geometry.size.w as f32) as u32 - 3 * padding / 2,
                        h: outer_geometry.size.h - 2*padding
                    }
                }
                Orientation::Vertical => {
                    Size{
                        w: outer_geometry.size.w - 2*padding,
                        h: (self.ratio * outer_geometry.size.h as f32) as u32 - 3 * padding / 2
                    }
                }
            }
        }
    }
}
