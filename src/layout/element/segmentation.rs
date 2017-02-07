use std::cell::*;
use std::sync::MutexGuard;

pub use rustwlc::types::{Geometry, Point, Size};

use wmstate::*;
use layout::*;
use definitions::{LayoutElemID};
use layout::arrangement::*;

use super::*;


#[derive(Clone)]
pub enum Orientation{
    Horizontal,
    Vertical,
}

pub struct Segmentation{
    children: Vec<LayoutElemID>,
    orientation: Orientation,
    ratio: f32
}


impl Segmentation{
    pub fn init(tree: &mut LayoutTree, partitions: u16, orientation: Orientation) -> Segmentation{
        assert!(partitions > 0, "A segmentation is expected to contain at least 1 child element.");
        
        let mut children: Vec<LayoutElemID> = Vec::new();
        for i in 0..partitions{
            children.push(tree.spawn_element())
        }

        Segmentation{
            children: children,
            orientation: orientation,
            ratio: 1.0 / partitions as f32
        }
    }
    pub fn init_horiz_50_50(tree: &mut LayoutTree) -> Segmentation{
        Segmentation{
            children: vec![tree.spawn_element(), tree.spawn_element()],
            orientation: Orientation::Horizontal,
            ratio: 0.5
        }
    }
    pub fn init_vert_50_50(tree: &mut LayoutTree) -> Segmentation{
        Segmentation{
            children: vec![tree.spawn_element(), tree.spawn_element()],
            orientation: Orientation::Vertical,
            ratio: 0.5
        }
    }

    pub fn get_children(&self) -> &Vec<LayoutElemID>
    {
        &self.children
    }

    pub fn get_orientation(self) -> Orientation
    {
        self.orientation
    }

    pub fn get_offset(&self, outer_geometry: Geometry, index: i32) -> Geometry
    {
        Geometry{
            origin: match self.orientation{
                Orientation::Horizontal => {  
                    Point{
                        x: outer_geometry.origin.x + index * (self.ratio * outer_geometry.size.w as f32) as i32,
                        y: 0i32
                    }
                }
                Orientation::Vertical => {
                    Point{
                        x: 0i32,
                        y: outer_geometry.origin.y + index * (self.ratio * outer_geometry.size.h as f32) as i32,
                    }
                }
            },
            size: match self.orientation
            {
                Orientation::Horizontal => {  
                    Size{
                        w: (self.ratio * outer_geometry.size.w as f32) as u32,
                        h: outer_geometry.size.h
                    }
                }
                Orientation::Vertical => {
                    Size{
                        w: outer_geometry.size.w,
                        h: (self.ratio * outer_geometry.size.h as f32) as u32
                    }
                }
            }
        }
    }
}
