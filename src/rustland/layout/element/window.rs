use std::option::*;
use std::process::Child;

use layout::*;
use layout::element::ElementPropertyProvider;
use sugars::program::GraphicsProgram;
use sugars::frame::Frame;
use utils::geometry::GeometryExt;
use wmstate::WMState;

use wlc::{View, WeakView, ResizeEdge};
use gl::types::GLuint;

#[derive(Clone)]
pub struct Window {
    weak_view: Option<WeakView>,
    desired_geometry: Geometry,
    inner_offset: Option<u32>,
    pub frame: Option<Frame>
}

impl Window{
    fn init(tree: &mut LayoutTree, parent_id: LayoutElemID, weak_view: WeakView, child_process: Child) -> (LayoutElemID, Window) {
        let window_ident = tree.spawn_dummy_element(Some(parent_id));
        
        let window = Window{
            weak_view: Some(weak_view), 
            desired_geometry: Geometry::zero(),
            inner_offset: None,
            frame: None
        };

        (window_ident, window)
    }

    pub fn init_dummy() -> Window{
        Window{
            weak_view: None,
            desired_geometry: Geometry::zero(),
            inner_offset: None,
            frame: None
        }
    }

    pub fn apply_frame(&mut self, element_ident: LayoutElemID, graphics_program: &mut GraphicsProgram, initial_opacity: f32){
        self.frame = Some(Frame::new(graphics_program.id, initial_opacity));
    }

    pub fn attach_view(&mut self, weak_view: WeakView){
        self.weak_view = Some(weak_view);
    }

    pub fn detach_view(&mut self) -> Option<WeakView>{
        self.weak_view.take()
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

    pub fn draw(&mut self, graphics_program: &GraphicsProgram){
        //self.frame.expect("window.draw(...): This method requires a frame to be present.").draw(graphics_program, self.desired_geometry);
    }
}

impl ElementPropertyProvider for Window{
    fn register_properties(&self, property_bank: &mut PropertyBank){    
        property_bank.address_property("frame_opacity", |profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Window, profile, window, {
                if let Some(ref mut frame) = window.frame{
                    
                    match new_value {
                        Some(value) => frame.opacity = value, 
                        None => {}
                    }

                    Some(&frame.opacity)
                }
                else { None }
            }
        )});
    }
}