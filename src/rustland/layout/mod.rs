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
use layout::policy::auto_circulation::AutoCirculation;
use layout::property::PropertyBank;
use layout::transition::Transition;
use utils::geometry::PointExt;
use layout::tag::*;

use wlc::{Output, View, WeakView};

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
    pub fn init(outer_geometry: Geometry, grid_w: usize, grid_h: usize) -> Self{
        LayoutTree{
            active_id: PARENT_ELEMENT   ,  
            focused_id: PARENT_ELEMENT,
            elements: Vec::new(),
            tags: TagRegister::init(),
            outer_geometry: outer_geometry,
            layout_policy: box AutoCirculation::init(grid_w * grid_h)
        }
    }

    pub fn refresh(wm_state: &mut WMState){
        TagRegister::refresh_tag_statuses(wm_state);

        let &mut WMState { ref tree, ref graphics_program, .. } = wm_state;
        let mut stacked_padding: Option<u32> = None; 
        let mut stacked_scale = (1.0f32, 1.0f32);
        arrangement::arrange(tree, PARENT_ELEMENT, wm_state.tree.outer_geometry, &mut stacked_padding, &mut stacked_scale, graphics_program.as_ref());
    }

    pub fn geometry_of(&self, element_ident: LayoutElemID) -> Option<Geometry> {
        let mut stacked_padding: Option<u32> = None; 
        let mut stacked_scale = (1.0f32, 1.0f32);
        arrangement::geometry_of(self, PARENT_ELEMENT, element_ident, self.outer_geometry, &mut stacked_padding, &mut stacked_scale)
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
    
    pub fn lookup_element_from_view(&self, view: &View) -> Option<LayoutElemID>{
        match self.tags.view_bindings.keys().find(|&x| *x == view.weak_reference()){
            Some(key) => self.tags.view_bindings.get(&key).cloned(),
            None => None
        }
    }

    pub fn spawn_dummy_element(&mut self, parent_id: Option<LayoutElemID>) -> LayoutElemID{
        self.elements.push(RefCell::new(LayoutElement::init_dummy(self.active_id, parent_id)));

        self.active_id += 1;
        return self.active_id - 1;
    }

    pub fn remove_view_binding_to(&mut self, element_ident: LayoutElemID) {
        self.tags.view_bindings.retain(|_, &mut v| v != element_ident);
    } 

    pub fn reserve_element_identity(&mut self, identity_to_reserv: LayoutElemID, profile: LayoutElementProfile) {
        if let LayoutElementProfile::Window(ref window) = profile { 
            if let Some(ref view) = window.get_view(){
                self.tags.view_bindings.insert(view.weak_reference(), identity_to_reserv); 
            }
        }
        
        if let Some(ref mut element) = self.lookup_element(identity_to_reserv) {
            println!("Reserved identity [{}] for a '{}' element.", identity_to_reserv, profile);
            element.set_profile(profile);
        }
    }

    pub fn swap_element_profile(&mut self, identity: LayoutElemID, new_profile: LayoutElementProfile) -> Option<LayoutElementProfile> {
        let mut old_profile = None; 

        if let LayoutElementProfile::Window(ref window) = new_profile { 
            if let Some(ref view) = window.get_view(){
                self.tags.view_bindings.insert(view.weak_reference(), identity); 
            }
        }

        if let Some(ref mut element) = self.lookup_element(identity) {
            old_profile = Some(element.profile.clone());

            element.set_profile(new_profile);
        }

        if let Some(ref profile) = old_profile {
            if let &LayoutElementProfile::Window(ref window) = profile { 
                if let Some(ref view) = window.get_view(){
                    self.tags.view_bindings.remove(&view.weak_reference()); 
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

    pub fn parent_of(&self, element_ident: LayoutElemID) -> LayoutElemID {
        self.
            lookup_element(element_ident).expect("Element does not exist or is already borrowed!")
            .parent_id.expect("Element does not have a parent!")
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

    pub fn animate_property(&self, element_id: LayoutElemID, transitioning_property: &'static str, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64){
        self.animate_property_after_delay(element_id, transitioning_property, new_value, relative_transition, time_frame_ms, 0);
    }

    pub fn animate_element_property(&self, element: &mut LayoutElement, transitioning_property: &'static str, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64){
        let prev_value = element.get_property(transitioning_property)
            .expect("animate_property_after_delay: Profile does not provide the property.");
        
        if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.try_lock(){    
            active_transitions.push(Transition::new(element.element_id, transitioning_property, prev_value, new_value, relative_transition, time_frame_ms, 0));
        }
    }

    pub fn animate_property_after_delay(&self, element_id: LayoutElemID, transitioning_property: &'static str, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64, delay_ms: u64){
        assert!(time_frame_ms != 0u64, "Time frame can't be zero!");
        
        if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.try_lock(){   
            let mut elem = self.lookup_element(element_id)
                .expect("animate_property_after_delay: Either element identy does exist or it is already borrowed! Maybe use the explicit call instead?");
                
            if let Some(value_origin) = elem.get_property(transitioning_property){
                active_transitions.push(Transition::new(element_id, transitioning_property, value_origin, new_value, relative_transition, time_frame_ms, delay_ms));
            }
            else{
                // Something unexpected happened so we go directly to new value without a transition
                let profile_name = elem.profile.to_string();

                match relative_transition { 
                    true => elem.set_property(transitioning_property, new_value),
                    false => println!("animate_property_after_delay: Profile '{}' of identity [{}] does not provide the property '{}'.", profile_name, element_id, transitioning_property)
                }
            }
        }   
    }

    pub fn animate_property_explicitly(&self, element_id: LayoutElemID, transitioning_property: &'static str, prev_value: DefaultNumericType, new_value: DefaultNumericType, relative_transition: bool, time_frame_ms: u64, delay_ms: u64){
        if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.try_lock(){    
            active_transitions.push(Transition::new(element_id, transitioning_property, prev_value, new_value, relative_transition, time_frame_ms, delay_ms));
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