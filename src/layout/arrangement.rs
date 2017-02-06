//extern crate num;
//use num::rational::Ratio;

use std::borrow::BorrowMut;
use std::sync::MutexGuard;
use std::collections::VecDeque;
use std::ops::DerefMut;

use rustwlc::types::{Point, Size};

use super::*;

use wmstate::*;
use super::window::*;
use definitions::{LayoutElemID};


/// Arrangemnet  
/// Recursive methods for describing and interacting with the layout

pub fn tree(wm_state: &MutexGuard<WMState>, outer_element_id: LayoutElemID)
{
    // Use debug for LayoutElement
    if let Some(outer_element) = wm_state.tree.lookup_element(outer_element_id){
        match *outer_element
        {
            LayoutElement::Workspace(ref element) =>
            {
                println!("├── Workspace");
                print!("   ");
            },
            LayoutElement::Segm(ref element) =>
            {
                println!("├──Segmentation ");

                for (i, child_id) in element.get_children().iter().enumerate()
                {
                    print!("   ");

                    //Recursion
                    tree(wm_state, *child_id);
                }
            },
            LayoutElement::Window(ref window) =>
            {
                println!("├── Window: {} [{}]", 
                    if let Some(view) = window.get_view(){
                        view.get_class()
                    }
                    else{
                        String::from("Untitled")
                    },
                    window.get_desired_geometry()
                );
            },
            LayoutElement::None => {
                println!("├── Unoccupied");
            }
        }  
    }
}

pub fn find_first_empty_element(tree: &LayoutTree, outer_element_id: LayoutElemID) -> Option<LayoutElemID>{
    if let Some(outer_element) =  tree.lookup_element(outer_element_id){
        match *outer_element
        {
            LayoutElement::None => {
                return Some(outer_element_id);
            },
            LayoutElement::Segm(ref segm) =>{
                for element_id in segm.get_children()
                {
                    if let Some(candidate_id) = find_first_empty_element(tree, *element_id){
                        return Some(candidate_id);
                    }
                    
                }
            },
            _ => {}
        }
    }
    return None;
}

pub fn arrange(tree: &LayoutTree, outer_element_id: LayoutElemID, outer_geometry: Geometry)
{
    if let Some(mut outer_element) = tree.lookup_element(outer_element_id){
        match outer_element.deref_mut()
        {
            &mut LayoutElement::Segm(ref segm) =>
            {               
                for (i, child_id) in segm.get_children().iter().enumerate()
                {   
                    // Recursion
                    arrange(tree, *child_id, segm.get_offset(outer_geometry, i as i32));
                }
            },
            &mut LayoutElement::Window(ref mut window) =>
            {
                window.set_desired_geometry(outer_geometry.clone());
            },
            _ => {}
        }  
    }
}
