use std::collections::HashMap;
use std::fmt;
use num::traits::ToPrimitive;

use common::definitions::LayoutElemID;
use layout::LayoutTree;
use layout::property::{PropertyProvider, PropertyBank};
use wmstate::WMState;

pub mod workspace;
pub mod window;
pub mod bisect;
pub mod padding;

use layout::element::LayoutElement::{Bisect, Workspace, Padding, Window};

pub enum LayoutElement{
    // Unallocated space in the layout
    None,

    // A container of exactly two child elements
    Bisect(bisect::Bisect),

    // A container of multiple child elements with only one active in a given moment
    Workspace(workspace::Workspace),
    
    // A container that can be smaller in relation to the outside geometry 
    Padding(padding::Padding),

    // An arbitrary window/application
    Window(window::Window)
}

impl PropertyProvider for LayoutElement{
    fn register_properties(&self, property_bank: &mut PropertyBank){
        match self{
            &Bisect(ref bisect) => {},
            &Workspace(ref workspace) => {},
            &Padding(ref padding) => padding.register_properties(property_bank),
            &Window(ref window) => {},
            _ => {}
        }
    }

    fn get_property(&mut self, tree: &LayoutTree, elem_id: LayoutElemID, name: String) -> Option<f32>{
        if let Some(property_bank) = tree.properties.get(&elem_id){
            if let Some(property_handle) = property_bank.get_handle(name){
                if let Some(property_value) = property_handle(self, None){
                    return property_value.to_f32();
                }
            }
        }

        None
    }

    fn set_property(&mut self, tree: &LayoutTree, elem_id: LayoutElemID, name: String, new_value: f32){
        if let Some(property_bank) = tree.properties.get(&elem_id){
            if let Some(handle) = property_bank.get_handle(name){
                handle(self, Some(new_value));
            }
        }
    }
}

impl fmt::Debug for LayoutElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}