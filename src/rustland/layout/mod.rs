#[macro_use] pub mod property;

pub mod arrangement;
pub mod element;
pub mod tag;
pub mod policy;

use std::cmp;
use std::fmt;
use std::collections::HashMap;
use std::cell::*;
use std::cell::*;
use std::sync::RwLock;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::ops::Deref;

use rustwlc::handle::*;
use rustwlc::types::*;
use wmstate::*;
use definitions::{LayoutElemID, MAX_WORKSPACES_LIMIT};
use layout::element::LayoutElement;
use layout::element::bisect::*;
use layout::element::workspace::*;
use layout::element::bisect::*;
use layout::element::window::*;
use layout::policy::LayoutPolicy;
use layout::policy::circulation::Circulation;
use layout::property::{PropertyProvider, PropertyBank};

use utils::interpolation::NumericInterpolation;
use utils::interpolation::methods::LinearInterpolator;

use layout::tag::*;

pub const PARENT_ELEMENT: LayoutElemID = 0;

pub struct LayoutTree{
    // the currently active workspace
    active_workspace: u16,

    // the index of the last added element
    active_id: LayoutElemID,

    // the last focused layout element
    pub focused_id: LayoutElemID,

    // the available workspaces of the layout 
    elements: HashMap<LayoutElemID, RefCell<LayoutElement>>,

    // the complete geometrical surface of all monitors 
    outer_geometry: Geometry,

    // tag register used to give names to layout elements  
    pub tags: TagRegister,

    pub layout_policy: Box<LayoutPolicy>,

    properties: HashMap<LayoutElemID, PropertyBank>
}

impl LayoutTree {
    pub fn init(outer_geometry: Geometry, no_monitors: u16) -> Self{
        const default_workspace: u16 = 1; 
        assert!(default_workspace <= 1 as u16, "The minimum number of workspaces required are {}", default_workspace);

        let mut tree = LayoutTree{
            active_id: PARENT_ELEMENT,  
            focused_id: PARENT_ELEMENT,
            active_workspace: default_workspace,
            elements: HashMap::new(),
            tags: TagRegister::init(),
            outer_geometry: outer_geometry,
            layout_policy: Box::new(Circulation::init()),
            properties: HashMap::new()
        };

        //Place root 
        let parent_id = tree.spawn_element();
        tree.tags.tag_element_on_condition("root", |elem_id, wm_state| elem_id == PARENT_ELEMENT);
        tree.tags.tag_element_on_condition("focused", |elem_id, wm_state| elem_id == wm_state.tree.focused_id);
        
        let workspace = Workspace::init(&mut tree, MAX_WORKSPACES_LIMIT);
        tree.insert_element_at(LayoutElement::Workspace(workspace), parent_id);
        
        tree
    }

    pub fn refresh(wm_state: &mut WMState){
        TagRegister::refresh_tag_statuses(wm_state);
        
        let mut stacked_padding: Option<u32> = None; 
        arrangement::arrange(&wm_state.tree, PARENT_ELEMENT, wm_state.tree.outer_geometry, &mut stacked_padding);
    }

    pub fn lookup_element(&self, elem_id: LayoutElemID) -> Option<RefMut<LayoutElement>>{   
        match self.elements.get(&elem_id)
        {
            Some(element) => Some(element.borrow_mut()),
            None => { panic!("Element out of reach.") }
        }
    }

    pub fn lookup_element_by_tag(&self, tag: String) -> Vec<RefMut<LayoutElement>>{   
        let mut element_references = Vec::<RefMut<LayoutElement>>::new();
        
        for elem_id in self.tags.address_element_by_tag(tag){
            match self.elements.get(&elem_id)
            {
                Some(element) => { element_references.push(element.borrow_mut()) },
                None => { panic!("Element out of reach.") }
            }
        }

        element_references
    }
    pub fn lookup_element_from_view(&self, view_pid: i32) -> LayoutElemID{
        match self.tags.view_bindings.get(&view_pid)
        {
            Some(element_id) => *element_id,
            None => { panic!("Element not found!"); }
        }
    }

    pub fn insert_element_at(&mut self, new_element: LayoutElement, elem_id: LayoutElemID) -> Option<RefCell<LayoutElement>>{
        new_element.register_properties(self.properties.get_mut(&elem_id).expect("An element with this has not spawned?!"));

        self.swap_cell(elem_id, RefCell::new(new_element))
    }

    pub fn swap_cell(&mut self, elem_id: LayoutElemID, new_cell: RefCell<LayoutElement>) -> Option<RefCell<LayoutElement>>{
        match *(new_cell.borrow()){
            LayoutElement::Window(ref window) => { 
                if let Some(view) = window.get_view(){
                    self.tags.view_bindings.insert(view.get_pid(), elem_id); 
                }
            }
            _ => {}
        }

        (*new_cell.borrow()).register_properties(self.properties.get_mut(&elem_id).expect("An element with this has not spawned?!"));

        let old_cell = self.elements.insert(
            elem_id, 
            new_cell
        );

        if let Some(ref old_element) = old_cell{
            match *(old_element.borrow()){
                LayoutElement::Window(ref window) => { 
                    if let Some(view) = window.get_view(){    
                        self.tags.view_bindings.remove(&view.get_pid()); 
                    }
                }
                _ => {}
            }
        }

        old_cell
    }

    pub fn spawn_element(&mut self) -> LayoutElemID{
        self.elements.insert(self.active_id, RefCell::new(LayoutElement::None));
        self.properties.insert(self.active_id, PropertyBank::new());

        self.active_id += 1;
        return self.active_id - 1;
    } 

    pub fn root(&self) -> RefMut<LayoutElement>{
        match self.lookup_element(PARENT_ELEMENT)
        {
            Some(parent) => parent,
            None => { panic!("Root not found!"); }
        }
    }

    pub fn get_all_element_ids(&self) -> Vec<LayoutElemID>{
        let mut elements_ids = Vec::new();

        for elem_id in self.elements.keys() {
            elements_ids.push(elem_id.clone())
        }

        elements_ids
    }

    pub fn last_window_id(&self) -> Option<LayoutElemID>{
        let mut i = self.active_id - 1;
        
        while {
            if let Some(a) = self.elements.get(&i)
            {   
                match *a.borrow(){
                    LayoutElement::Window(ref window) => { return Some(i) }
                    _ => { true }
                }
            }
            else {
                false
            }
        }{
            match i{
                0 => { break; },
                _ => { i -= 1; }
            }
        };

        None        
    }

    pub fn get_outer_geometry(&self) -> Geometry{
        self.outer_geometry
    }

    pub fn set_outer_geometry(&mut self, new_geometry: Geometry){
        self.outer_geometry = new_geometry;
    }

    pub fn get_element_properties(&self, elem_id: LayoutElemID) -> Option<&PropertyBank>{
        self.properties.get(&elem_id)
    }

/*
    fn animate<T: AnimatedObject>(&self, element_id: LayoutElemID, destination: u32) -> Animation{
        let origin_value = 
            if let Some(element) = self.lookup_element(element_id){
                *((element.borrow_mut() as &mut AnimatedObject).find_value())
            }
            else{
                destination
            };

        Animation{
            element_id: element_id,
            interpolation: NumericInterpolation::new(box LinearInterpolator{}, origin_value, 1f32, 100)
        }
    }
    */
}

impl fmt::Display for LayoutTree{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut indentation_whtspcs = 0;
        arrangement::tree(self, f, PARENT_ELEMENT, &mut indentation_whtspcs);

        writeln!(f)
    }
}

pub extern fn on_output_resolution(output: WlcOutput, _from: &Size, _to: &Size) {
    let mut wm_state = WM_STATE.write().unwrap();

    wm_state.tree.set_outer_geometry(Geometry::new(Point::origin(), *_to));

    println!("Updated resolution: {}", _to);
}
