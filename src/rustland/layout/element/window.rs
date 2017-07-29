use std::option::*;
use std::process::Child;

use layout::*;
use utils::geometry::GeometryExt;

use wlc::{View, WeakView, ResizeEdge};

pub struct Window {
    weak_view: Option<WeakView>,
    child_process: Option<Child>,
    desired_geometry: Geometry,
    inner_offset: Option<u32>
}

impl Window{
    fn init(weak_view: WeakView, child_process: Child) -> Window{
        Window{
            weak_view: Some(weak_view),
            child_process: Some(child_process),  
            desired_geometry: Geometry::zero(),
            inner_offset: None
        }
    }

    pub fn init_dummy() -> Window{
        Window{
            weak_view: None,
            child_process: None,
            desired_geometry: Geometry::zero(),
            inner_offset: None
        }
    }

    pub fn attach_view(&mut self, weak_view: WeakView){
        self.weak_view = Some(weak_view);
    }

    pub fn get_view(&self) -> Option<&View>{
        if let Some(ref weak_view) = self.weak_view{
            unsafe { weak_view.upgrade() }
        }
        else{
            None
        }
    }

    pub fn get_desired_geometry(&self) -> Geometry{
        self.desired_geometry
    }

    pub fn set_desired_geometry(&mut self, geometry: Geometry){
        self.desired_geometry = geometry;

        match self.get_view()
        {
            Some(ref mut view) => {
                view.set_geometry(ResizeEdge::Null, 
                    // Application of inner offset
                    if let Some(inner_offet) = self.inner_offset{
                        Geometry::new(
                            Point{
                                x: self.desired_geometry.origin.x + inner_offet as i32, 
                                y: self.desired_geometry.origin.y + inner_offet as i32
                            },
                            Size{
                                w: self.desired_geometry.size.w - 2*inner_offet, 
                                h: self.desired_geometry.size.h - 2*inner_offet
                            }
                        )
                    }
                    else{
                        self.desired_geometry
                    }
                );
            },
            None => { println!("Tried to change location of non-existing window!"); }
        }
    }
}
