
#![feature(box_syntax)]
#![feature(core)]
#![feature(unboxed_closures)]

use std::collections::HashSet;
use std::vec::Vec;
use std::option::*;
use std::boxed::Box;
use std::cell::*;
use std::process::Child;

use rustwlc::types::*;
use rustwlc::handle::*;
use rustwlc::callback;

use wmstate::*;
use definitions::{ElementReference, WM_CATCH_EVENT};
use common::job::{Job, JobType};
use layout::*;
use layout::arrangement::*;
use super::LayoutElement;

pub struct Window {
    view: Option<WlcView>,
    child_process: Option<Child>,
    desired_geometry: Geometry,
    inner_offset: Option<u32>
}

impl Window{
    fn init(view: WlcView, child_process: Child) -> Window{
        Window{
            view: Some(view),
            child_process: Some(child_process),  
            desired_geometry: Geometry::zero(),
            inner_offset: None
        }
    }

    fn init_dummy() -> Window{
        Window{
            view: None,
            child_process: None,
            desired_geometry: Geometry::zero(),
            inner_offset: None
        }
    }

    fn attach_view(&mut self, view: WlcView){
        self.view = Some(view);
    }

    pub fn get_view(&self) -> Option<WlcView>{
        self.view
    }

    pub fn get_view_mut(&mut self) -> &mut Option<WlcView>{
        &mut self.view
    }


    pub fn get_desired_geometry(&self) -> Geometry{
        self.desired_geometry
    }

    pub fn set_desired_geometry(&mut self, geometry: Geometry){
        println!("Debug: {} positioned at: {}", 
            if let Some(view) = self.view { view.get_class() } else { String::from("Untitled") }, 
            self.desired_geometry
        );

        self.desired_geometry = geometry;

        match self.view
        {
            Some(ref mut view) => {
                view.set_geometry(ResizeEdge::empty(), 
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
            None => { panic!("Tried to change location of non-existing window!"); }
        }
    }
}

pub extern fn on_view_created(view: WlcView) -> bool {
    let mut wm_state = WM_STATE.write().unwrap();

    view.set_type(VIEW_BIT_UNMANAGED, false);
    view.set_mask(view.get_output().get_mask());
    view.bring_to_front();
    view.focus();    

    let mut window = Window::init_dummy();
    window.attach_view(view);

    if view.get_type().is_empty(){
        let mut layout_policy = wm_state.tree.layout_policy.clone();
        let window_elem_id = layout_policy.attach_window(&mut wm_state.tree);
        wm_state.tree.layout_policy = layout_policy;

        wm_state.tree.insert_element_at(LayoutElement::Window(window), window_elem_id);  
        
        let mut tag = view.get_class().to_lowercase();
        if tag.is_empty(){
            tag = view.get_title().split_whitespace().next().unwrap_or("").to_lowercase();
        }
        if !tag.is_empty(){
            wm_state.tree.tags.tag_element(tag.as_ref(), window_elem_id);
        }

        if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
            pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
        } 

        view.bring_to_front();
    }

    WM_CATCH_EVENT
}

pub extern fn on_view_destroyed(view: WlcView) {
    let mut wm_state = WM_STATE.write().unwrap();
    
    /*
        TODO: This will cause seg. fault for some reason:
        let element_id = wm_state.tree.lookup_element_from_view(view.get_pid());
        wm_state.tree.swap_element(element_id, LayoutElement::None);
    */

    if let Some(top_view) = get_topmost_view(&view.get_output(), 0) {
        top_view.focus();
    }
}

fn get_topmost_view(output: &WlcOutput, offset: usize) -> Option<WlcView> {
    let views = output.get_views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()].clone())
    }
}

pub extern fn on_view_focus(view: WlcView, focused: bool) { 
    if focused && view.get_type().is_empty(){
        if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
            pending_jobs.push(Job::init(JobType::FOCUS, Some(ElementReference::ViewPID(view.get_pid())), Vec::new()));
        }  
    }
}

pub extern fn on_view_request_move(view: WlcView, origin: &Point) {
    
}

pub extern fn on_view_request_resize(view: WlcView, edges: ResizeEdge, origin: &Point) {
    
}
