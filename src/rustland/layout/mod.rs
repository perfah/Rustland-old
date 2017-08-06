#[macro_use] pub mod property;

pub mod arrangement;
pub mod element;
pub mod tag;
pub mod policy;
pub mod transition;

use std::cmp;
use std::fmt;
use std::collections::HashMap;
use std::cell::*;
use std::cell::*;
use std::sync::RwLock;
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use wmstate::*;
use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::element::LayoutElement;
use layout::element::bisect::*;
use layout::element::grid::*;
use layout::element::padding::*;
use layout::element::LayoutElementProfile;
use layout::policy::LayoutPolicy;
use layout::policy::circulation::Circulation;
use layout::property::PropertyBank;
use layout::transition::Transition;
use utils::geometry::PointExt;
use layout::tag::*;

use wlc::Output;

pub const PARENT_ELEMENT: LayoutElemID = 0;

pub struct LayoutTree{
    // the index of the last added element
    active_id: LayoutElemID,

    // the last focused layout element
    pub focused_id: LayoutElemID,

    // the available workspaces of the layout 
    elements: Vec<RefCell<LayoutElement>>,

    // the complete geometrical surface of all monitors 
    outer_geometry: Geometry,

    // tag register used to give names to layout elements  
    pub tags: TagRegister,

    pub layout_policy: Box<LayoutPolicy>,
}

impl LayoutTree {
    pub fn init(outer_geometry: Geometry) -> Self{
        let mut tree = LayoutTree{
            active_id: PARENT_ELEMENT   ,  
            focused_id: PARENT_ELEMENT,
            elements: Vec::new(),
            tags: TagRegister::init(),
            outer_geometry: outer_geometry,
            layout_policy: Box::new(Circulation::init())
        };

        tree.tags.tag_element_on_condition("root", |elem_id, _| elem_id == PARENT_ELEMENT);
        tree.tags.tag_element_on_condition("focused", |elem_id, wm_state| elem_id == wm_state.tree.focused_id);

        // Root element
        let (root_ident, root_profile) = Padding::init(tree.spawn_dummy_element(None), &mut tree, 100, Some(Point::origin()));
        
        // Workspaces
        let (grid_ident, grid_profile) = Grid::init(root_profile.child_elem_id, &mut tree, 2, 2);

        tree.reserve_element_identity(root_ident, LayoutElementProfile::Padding(root_profile));
        tree.reserve_element_identity(grid_ident, LayoutElementProfile::Grid(grid_profile));

        tree.animate_property(root_ident, "gap_size".to_string(), 0f32, false, 250);

        tree

    }
    pub fn refresh(wm_state: &mut WMState){
        TagRegister::refresh_tag_statuses(wm_state);

        let mut stacked_padding: Option<u32> = None; 
        arrangement::arrange(&wm_state.tree, PARENT_ELEMENT, wm_state.tree.outer_geometry, &mut stacked_padding);
    }

    pub fn lookup_element(&self, elem_id: LayoutElemID) -> Option<RefMut<LayoutElement>>{   
        let position = self.elements.iter().position(|element: &RefCell<LayoutElement>| {
            match element.try_borrow(){
                Ok(elem) => elem.element_id == elem_id,
                _ => false
            }
        });
        
        match position{
            Some(index) =>  unsafe { Some(self.elements.get_unchecked(index).borrow_mut()) },
            None => None
        }
    }

    pub fn lookup_element_by_tag(&self, tag: String) -> Vec<RefMut<LayoutElement>>{   
        let mut element_references = Vec::<RefMut<LayoutElement>>::new();
        
        for elem_id in self.tags.address_element_by_tag(tag){
            match self.lookup_element(elem_id)
            {
                Some(element_ref) => { element_references.push(element_ref); },
                None => {}
            };
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

    pub fn spawn_dummy_element(&mut self, parent_id: Option<LayoutElemID>) -> LayoutElemID{
        self.elements.push(RefCell::new(LayoutElement::init_dummy(self.active_id, parent_id)));

        self.active_id += 1;
        return self.active_id - 1;
    } 

    pub fn reserve_element_identity(&mut self, identity_to_reserv: LayoutElemID, profile: LayoutElementProfile) {
        if let LayoutElementProfile::Window(ref window) = profile { 
            if let Some(ref view) = window.get_view(){
                self.tags.view_bindings.insert(view.pid(), identity_to_reserv); 
            }
        }
        
        if let Some(ref mut element) = self.lookup_element(identity_to_reserv) {
            element.set_profile(profile);
        }
    }

    pub fn swap_element_profile(&mut self, identity: LayoutElemID, new_profile: LayoutElementProfile) -> Option<LayoutElementProfile> {
        let mut old_profile = None; 

        if let LayoutElementProfile::Window(ref window) = new_profile { 
            if let Some(ref view) = window.get_view(){
                self.tags.view_bindings.insert(view.pid(), identity); 
            }
        }

        if let Some(ref mut element) = self.lookup_element(identity) {
            old_profile = Some(element.get_profile_mut().clone());

            element.set_profile(new_profile);
        }

        if let Some(ref profile) = old_profile {
            if let &LayoutElementProfile::Window(ref window) = profile { 
                if let Some(ref view) = window.get_view(){
                    self.tags.view_bindings.remove(&view.pid()); 
                }
            }
        }

        old_profile
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

        for element in self.elements.iter() {
            elements_ids.push(element.borrow().element_id.clone())
        }

        elements_ids
    }

    pub fn get_outer_geometry(&self) -> Geometry{
        self.outer_geometry
    }

    pub fn set_outer_geometry(&mut self, new_geometry: Geometry){
        self.outer_geometry = new_geometry;
    }

    pub fn get_element_properties(&self, elem_id: LayoutElemID) -> Option<&PropertyBank>{
        if let Some(element) = self.lookup_element(elem_id)  {
            None //Some(&element.properties)
        }
        else {
            None
        }
    }

    pub fn animate_property(&self, element_id: LayoutElemID, transitioning_property: String, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64){
        if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.lock(){    
            if let Some(ref mut elem) = self.lookup_element(element_id){
                if let Some(value_origin) = (*elem).get_property(transitioning_property.clone()){
                    active_transitions.push(Transition::new(element_id, transitioning_property, value_origin, new_value, relative_transition, time_frame_ms, 0));
                }
                else{
                    // Something unexpected happened so we go directly to new value without a transition
                    elem.set_property(transitioning_property.clone(), if relative_transition { new_value } else { panic!("Element either doesn't exist or it doesn't provide that property!")});
                }
            }
        }
    }

    pub fn animate_property_after_delay(&self, element_id: LayoutElemID, transitioning_property: String, prev_value: DefaultNumericType, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64, delay_ms: u64){
        if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.lock(){    
            if let Some(ref mut elem) = self.lookup_element(element_id){
                active_transitions.push(Transition::new(element_id, transitioning_property, prev_value, new_value, relative_transition, time_frame_ms, delay_ms));
            }
        }
    }
}

impl fmt::Display for LayoutTree{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut indentation_whtspcs = 0;
        arrangement::tree(self, f, PARENT_ELEMENT, &mut indentation_whtspcs);

        writeln!(f)
    }
}

/*
fn get_topmost_view(output: &Output, offset: usize) -> Option<&View> {
    let views = output.views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()])
    }
}

*/