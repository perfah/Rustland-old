#![macro_use]

use std::fmt;
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};

pub mod bisect;
pub mod padding;
pub mod window;
pub mod grid;

use common::definitions::LayoutElemID;
use layout::element::LayoutElementProfile::{Bisect, Padding, Window, Grid};
use layout::property::{ElementPropertyProvider, PropertyBank};
use sugars::Renderable;
#[macro_use] use utils;

#[derive(Serialize, Deserialize)]
pub struct LayoutElement{
    pub parent_id: Option<LayoutElemID>,
    pub element_id: LayoutElemID,
    
    pub profile: LayoutElementProfile,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub properties: PropertyBank
}

impl LayoutElement {
    pub fn init_dummy(element_id: LayoutElemID, parent_id: Option<LayoutElemID>) -> LayoutElement {
        LayoutElement {
            element_id: element_id,
            parent_id: parent_id,
            profile: LayoutElementProfile::None,
            properties: PropertyBank::empty()
        }
    }

    pub fn set_profile(&mut self, new_profile: LayoutElementProfile) {
        match new_profile {
            LayoutElementProfile::Bisect(ref bisect) => bisect.register_properties(&mut self.properties),
            LayoutElementProfile::Grid(_) => {},
            LayoutElementProfile::Padding(ref padding) => padding.register_properties(&mut self.properties),
            LayoutElementProfile::Window(_) => {},
            LayoutElementProfile::None => {}
            _ => { println!("Warning: No properties registered for element {}", self.element_id); }
        }
        
        self.profile = new_profile;
    }

    pub fn get_property(&mut self, name: &'static str) -> Option<f32>{
        if let Some(property_handle) = self.properties.get_handle(name){
            if let Some(property_value) = property_handle(&mut self.profile, None){
                return property_value.to_f32();
            }
        }
    
        None
    }

    pub fn set_property(&mut self, name: &'static str , new_value: f32){
        if let Some(handle) = self.properties.get_handle(name){
            handle(&mut self.profile, Some(new_value));
        }
    }   
}
impl fmt::Display for LayoutElementProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 
            match self {
                &LayoutElementProfile::Bisect(_) => "bisect",
                &LayoutElementProfile::Grid(_) => "grid",
                &LayoutElementProfile::Padding(_) => "padding",
                &LayoutElementProfile::Window(_) => "window",
                _ => "n/a"
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

//my_macro!();
 //enum_derive_trait!(LayoutElementProfile, Renderable, draw);

impl LayoutElementProfile {
    pub fn is_none(&self) -> bool{
        match *self {
            LayoutElementProfile::None => true,
            _ => false 
        }
    } 
}

trait ElementContainer{
    fn disown_child(&mut self, element_ident: LayoutElemID);
}

