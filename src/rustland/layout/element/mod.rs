use std::fmt;
use std::cell::{RefCell, RefMut};
use std::ops::DerefMut;

pub mod bisect;
pub mod padding;
pub mod window;
pub mod grid;

use common::definitions::LayoutElemID;
use layout::LayoutTree;
use layout::element::LayoutElementProfile::{Bisect, Padding, Window, Grid};
use layout::property::{ElementPropertyProvider, PropertyBank};

pub struct LayoutElement{
    pub parent_id: Option<LayoutElemID>,
    pub element_id: LayoutElemID,
    profile: RefCell<LayoutElementProfile>,
    pub properties: PropertyBank
}

impl LayoutElement {
    pub fn init_dummy(element_id: LayoutElemID, parent_id: Option<LayoutElemID>) -> LayoutElement {
        LayoutElement {
            element_id: element_id,
            parent_id: parent_id,
            profile: RefCell::new(LayoutElementProfile::None),
            properties: PropertyBank::empty()
        }
    }

    pub fn get_profile_mut(&self) -> RefMut<LayoutElementProfile> {
        self.profile.borrow_mut()
    }

    pub fn set_profile(&mut self, new_profile: LayoutElementProfile) {
        match new_profile {
            LayoutElementProfile::Bisect(ref bisect) => bisect.register_properties(&mut self.properties),
            LayoutElementProfile::Grid(_) => {},
            LayoutElementProfile::Padding(ref padding) => padding.register_properties(&mut self.properties),
            LayoutElementProfile::Window(_) => {},
            _ => { println!("Warning: Cannot register properties for an element without a profile."); }
        }
        
        self.profile = RefCell::new(new_profile);
    }

    pub fn get_property(&mut self, name: String) -> Option<f32>{
        let mut profile = self.get_profile_mut();

        if let Some(property_handle) = self.properties.get_handle(name){
            if let Some(property_value) = property_handle(profile.deref_mut(), None){
                return property_value.to_f32();
            }
        }
    
        None
    }

    pub fn set_property(&mut self, name: String, new_value: f32){
        if let Some(handle) = self.properties.get_handle(name){
            handle(self.get_profile_mut().deref_mut(), Some(new_value));
        }
    }
}

#[derive(Clone)]
pub enum LayoutElementProfile {
    // Unallocated space in the layout
    None,

    // A container of exactly two child elements
    Bisect(bisect::Bisect),

    // A container of multiple child elements with only one active in a given moment
    Grid(grid::Grid),
    
    // A container that can be smaller in relation to the outside geometry 
    Padding(padding::Padding),

    // An arbitrary window/application
    Window(window::Window)
}
