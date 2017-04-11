
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
use layout::rules::*;
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
            inner_offset: Some(30)
        }
    }

    fn init_dummy() -> Window{
        Window{
            view: None,
            child_process: None,
            desired_geometry: Geometry::zero(),
            inner_offset: Some(30)
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
                view.set_state(VIEW_RESIZING, true);
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
                view.set_state(VIEW_RESIZING, false);
                view.bring_to_front();
            },
            None => { panic!("Tried to change location of non-existing window!"); }
        }
    }

    pub fn offset(&mut self, offset: &Point){
        let pos = match self.view  
        {
            Some(ref view) => {
                if let Some(geometry) = view.get_geometry(){
                    Geometry{
                        origin: Point{
                            x: geometry.origin.x + offset.x,
                            y: geometry.origin.y + offset.y
                        },
                        size: Size{
                            w: 6,
                            h: 6
                        }
                    }
                }
                else
                {
                    Geometry{
                        origin: Point{x: 0, y: 0},
                        size: Size{w: 0, h:0 }
                    }
                }
            },
            None => {
                Geometry{
                    origin: Point{x: 0, y: 0},
                    size: Size{w: 0, h:0 }
                }
            }
        };

        self.set_desired_geometry(pos);
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
        let window_id = 
            if let Some(unoccupied_id) = find_first_empty_element(&wm_state.tree, PARENT_ELEMENT)
            {
                unoccupied_id
            }
            else
            {
                println!("NOTICE: Extending the layout structure!");

                if let Some(last_id) = wm_state.tree.last_window_id(){
                    let extension = super::segmentation::Segmentation::init_horiz_50_50(&mut wm_state.tree);
                    let new_preoccupied_id = extension.get_children()[0];
                    let new_unoccupied_id = extension.get_children()[1];

                    // update tags according to element swap
                    wm_state.tree.tags.handle_element_swap(last_id, new_preoccupied_id);

                    if let Some(thrown_out) = wm_state.tree.swap_element(last_id, LayoutElement::Segm(extension))
                    {
                        wm_state.tree.swap_cell(new_preoccupied_id, thrown_out);
                        new_unoccupied_id
                    }
                    else {
                        panic!("Last index did not exist!");
                    }
                }
                else{
                    panic!("ERROR: No space in layout found!")
                }
            };

        // Add tag
        wm_state.tree.tags.tag_element(view.get_class().to_lowercase().as_ref(), window_id);

        wm_state.tree.swap_element(window_id, LayoutElement::Window(window));  
        if let Some(element) = wm_state.tree.lookup_element(window_id)
        {
            
        }
    
        LayoutTree::refresh(&mut wm_state);
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
            pending_jobs.push(Job::init(JobType::FOCUS, Some(ElementReference::ViewID(view.get_pid())), Vec::new()));
        }  
    }
}

pub extern fn on_view_request_move(view: WlcView, origin: &Point) {
    
}

pub extern fn on_view_request_resize(view: WlcView, edges: ResizeEdge, origin: &Point) {
    
}
