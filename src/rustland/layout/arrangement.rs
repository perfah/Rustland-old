//extern crate num;
//use num::rational::Ratio;

use std::borrow::BorrowMut;
use std::sync::MutexGuard;
use std::collections::VecDeque;
use std::ops::DerefMut;

use rustwlc::types::{Point, Size};

use super::*;

use wmstate::*;
use super::element::*;
use super::element::window::*;
use definitions::{LayoutElemID};


/// Arrangement  
/// Recursive methods for describing and interacting with the layout

pub fn tree(tree: &LayoutTree, f: &mut fmt::Formatter, outer_element_id: LayoutElemID, indentation_whtspcs: &mut i32)
{
    let indent = |whtspcs, f: &mut fmt::Formatter| {
        for i in 0..whtspcs * 4{
            write!(f, " ");
        }
    };

    let format_tags = |elem_id| {
        let mut output = String::new();
        
        for tag in tree.tags.address_tags_by_element(elem_id){
            output.push_str("#");
            output.push_str(tag.as_ref());
            output.push_str(" ");
        }

        output
    };

    // Use debug for LayoutElement
    if let Some(outer_element) = tree.lookup_element(outer_element_id){
        match *outer_element
        {
            LayoutElement::Segm(ref element) =>
            {
                indent(*indentation_whtspcs, f);
                writeln!(f, "├──[{}] Segmentation: {}", outer_element_id, format_tags(outer_element_id));

                *indentation_whtspcs += 1;
                for (i, child_id) in element.get_children().iter().enumerate()
                {;
                    //Recursion
                    arrangement::tree(tree, f, *child_id, indentation_whtspcs);
                }
                *indentation_whtspcs -= 1;
            },
            LayoutElement::Workspace(ref element) =>
            {
                for (i, child_id) in element.get_all_children().iter().enumerate()
                {
                    indent(*indentation_whtspcs, f);

                    if *child_id == element.get_active_child_id(){
                        writeln!(f, "├──[{}] Workspace [{}]: {}", outer_element_id, i, format_tags(outer_element_id));
                    }
                    else{
                        writeln!(f, "├──[{}] Workspace  {}: {}", outer_element_id, i, format_tags(outer_element_id));
                    }

                    *indentation_whtspcs += 1;

                    //Recursion                    
                    arrangement::tree(tree, f, *child_id, indentation_whtspcs);

                    *indentation_whtspcs -= 1;
                }

                writeln!(f);
            },
            LayoutElement::Window(ref window) =>
            {
                indent(*indentation_whtspcs, f);
                write!(f, "├──[{}] Window: {}", outer_element_id, format_tags(outer_element_id));
                
                if tree.get_outer_geometry().contains_geometry(window.get_desired_geometry()){
                    write!(f, "[{}]", window.get_desired_geometry());
                }
                writeln!(f);
            },
            LayoutElement::None => {
                indent(*indentation_whtspcs, f);
                writeln!(f, "├──[{}] Unoccupied", outer_element_id);
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
            LayoutElement::Workspace(ref wrkspc) => {
                // Recursion
                if let Some(candidate_id) = find_first_empty_element(tree, wrkspc.get_active_child_id()){
                    return Some(candidate_id);
                }
            },
            LayoutElement::Segm(ref segm) =>{
                for element_id in segm.get_children()
                {
                    // Recursion
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
            &mut LayoutElement::Workspace(ref wrkspc) =>
            {               
                for (i, child_id) in wrkspc.get_all_children().iter().enumerate()
                {   
                    // Recursion
                    arrange(tree, *child_id, wrkspc.get_offset(tree, outer_geometry, i as u16));
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
