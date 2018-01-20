use std::ops::DerefMut;
use std::cell::RefMut;
use std::fmt;

use common::definitions::{TAG_PREFIX, PROPERTY_PREFIX, LayoutElemID};
use layout::LayoutTree;
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::bisect::Side;
use layout::arrangement;
use wmstate::*;
use utils::geometry::GeometryExt;

use wlc::{Size, Geometry, Visibility};

/// Arrangement  
/// Recursive methods for describing and interacting with the layout

pub fn tree(tree: &LayoutTree, f: &mut fmt::Formatter, outer_element_id: LayoutElemID, indentation_whtspcs: &mut i32){
    let indent = |whtspcs, f: &mut fmt::Formatter| {
        for i in 0..whtspcs * 4{
            write!(f, " ");
        }
    };

    let format_tags = |elem_id| {
        let mut output = String::new();
        
        for tag in tree.tags.address_tags_by_element(elem_id){
            output.push_str(TAG_PREFIX);
            output.push_str(tag.as_str());
            output.push_str(" ");
        }

        output
    };

    let format_props = |elem_id, element: &mut RefMut<LayoutElement>|{
        let mut output = String::new();

        
        let mut property_names = element.properties.get_all_property_names();
        property_names.sort();

        for property_name in property_names{
            if let Some(property_value) = element.get_property(property_name) {
                output.push_str(PROPERTY_PREFIX);
                output.push_str(property_name);
                output.push_str("=");
                output.push_str(format!("{}", property_value).as_str());
            }
            
            output.push_str(" ");
        }
        

        output
    };

    // Use debug for LayoutElement
    if let Some(mut outer_element) = tree.lookup_element(outer_element_id){
        let tags = format_tags(outer_element_id);
        let props = format_props(outer_element_id, &mut outer_element);

        match outer_element.profile{
            LayoutElementProfile::Bisect(ref element) => {
                indent(*indentation_whtspcs, f);
                writeln!(f, "├──[{}] Bisect: {} {}", outer_element_id, tags, props);

                *indentation_whtspcs += 1;
                for (i, child_id) in element.children_iter().enumerate()
                {;
                    //Recursion
                    arrangement::tree(tree, f, *child_id, indentation_whtspcs);
                }
                *indentation_whtspcs -= 1;
            },
            LayoutElementProfile::Grid(ref element) => {
                for (i, child_id) in element.get_all_children().iter().enumerate(){
                    indent(*indentation_whtspcs, f);

                    if *child_id == element.get_active_child_id(){
                        writeln!(f, "├──[{}] Workspace [{}]: {} {}", outer_element_id, i, tags, props);
                    }
                    else{
                        writeln!(f, "├──[{}] Workspace  {}: {} {}", outer_element_id, i, tags, props);
                    }

                    *indentation_whtspcs += 1;

                    //Recursion                    
                    arrangement::tree(tree, f, *child_id, indentation_whtspcs);

                    *indentation_whtspcs -= 1;
                }

                writeln!(f);
            },
            LayoutElementProfile::Padding(ref padding) => {
                indent(*indentation_whtspcs, f);
                writeln!(f, "├──[{}] Padding: {} {}", outer_element_id, tags, props);
                
                *indentation_whtspcs += 1;

                //Recursion                    
                arrangement::tree(tree, f, padding.child_elem_id, indentation_whtspcs);

                *indentation_whtspcs -= 1;
            },
            LayoutElementProfile::Window(ref window) => {
                indent(*indentation_whtspcs, f);
                write!(f, "├──[{}] Window: {} {}", outer_element_id, tags, props);
                
                if tree.get_outer_geometry().overlaps_geometry(window.get_desired_geometry()){
                    write!(f, "[{:?}]", window.get_desired_geometry());
                }
                writeln!(f);
            },
            LayoutElementProfile::None => {
                indent(*indentation_whtspcs, f);
                writeln!(f, "├──[{}] Unoccupied: {}", outer_element_id, tags);
            }
        }  
    }
}

pub fn find_first_unoccupied(tree: &LayoutTree, outer_element_id: LayoutElemID) -> Option<LayoutElemID>{
    if let Some(ref mut outer_element) =  tree.lookup_element(outer_element_id){
        let bisect_ratio = outer_element.get_property("ratio");

        match outer_element.profile {
            LayoutElementProfile::None => {
                return Some(outer_element_id);
            },
            LayoutElementProfile::Padding(ref padding) => {
                // Recursion to another layer of depth in the tree structure
                if let Some(candidate_id) = find_first_unoccupied(tree, padding.child_elem_id){
                    return Some(candidate_id);
                }
            },
            LayoutElementProfile::Bisect(ref bisect) =>{
                let mut iter = bisect.children_iter();

                while let Some(&child_ident) = iter.next(){
                    if let Some(element) = tree.lookup_element(child_ident){
                        if element.profile.is_none() {
                            tree.animate_property_explicitly(outer_element_id, "ratio", bisect_ratio.unwrap(), 0.5f32, false, 125, 0);    
                            return Some(child_ident);
                        }
                    }
                }

                for element_id in bisect.children_iter() {
                    // Recursion to another layer of depth in the tree structure
                    if let Some(candidate_id) = find_first_unoccupied(tree, *element_id){
                        return Some(candidate_id);
                    }   
                }
            },
            LayoutElementProfile::Grid(ref wrkspc) => {
                // Recursion to another layer of depth in the tree structure
                if let Some(candidate_id) = find_first_unoccupied(tree, wrkspc.get_active_child_id()){
                    return Some(candidate_id);
                }
            },
            _ => {}
        }
    }
    return None;
}

pub fn arrange(tree: &LayoutTree, outer_element_id: LayoutElemID, outer_geometry: Geometry, stacked_padding: &mut Option<u32>, stacked_scale: &mut f32){
    if let Some(mut outer_element) = tree.lookup_element(outer_element_id){
        match outer_element.profile{
            LayoutElementProfile::Bisect(ref bisect) => {               
                for (i, child_id) in bisect.children_iter().enumerate() {   
                    // Recursion
                    arrange(tree, *child_id, bisect.get_offset_geometry(outer_geometry, stacked_padding, i as i32), stacked_padding, stacked_scale);
                }
            },
            LayoutElementProfile::Grid(ref wrkspc) => { 
                for (i, child_id) in wrkspc.get_all_children().iter().enumerate() {   
                    // Recursion
                    arrange(tree, *child_id, wrkspc.get_offset_geometry(tree.get_outer_geometry(), outer_geometry, i as u16, stacked_scale), stacked_padding, stacked_scale);
                }
            },
            LayoutElementProfile::Padding(ref mut padding) => {
                *stacked_scale *= padding.inner_scale;
                *stacked_padding = Some(padding.gap_size);
                
                // Recursion
                arrange(tree, padding.child_elem_id, padding.get_offset_geometry(outer_geometry, stacked_scale), stacked_padding, stacked_scale);
                
                *stacked_scale /= if padding.inner_scale != 0f32 { padding.inner_scale } else { panic!("Scaling can't be 0.") };
                *stacked_padding = None;
            },
            LayoutElementProfile::Window(ref mut window) => {
                if let Some(view) = window.get_view(){
                    view.set_visibility(
                        if tree.outer_geometry.overlaps_geometry(outer_geometry) { Visibility::Slot1 } 
                        else { Visibility::Null }
                    );
                }
            
                window.set_desired_geometry(outer_geometry.clone());
            },
            _ => {}
        }  
    }
}

pub fn find_all_windows(matches: &mut Vec<LayoutElemID>, needs_to_be_active: bool, tree: &LayoutTree, outer_element_id: LayoutElemID) {
    if let Some(ref mut outer_element) =  tree.lookup_element(outer_element_id){
        match outer_element.profile {
            LayoutElementProfile::Padding(ref padding) => {
                // Recursion to another layer of depth in the tree structure
                find_all_windows(matches, needs_to_be_active, tree, padding.child_elem_id);
            },
            LayoutElementProfile::Bisect(ref bisect) =>{
                for candidate_id in bisect.children_iter() {
                    // Recursion to another layer of depth in the tree structure
                    find_all_windows(matches, needs_to_be_active, tree, *candidate_id);
                }
            },
            LayoutElementProfile::Grid(ref wrkspc) => {
                // Recursion to another layer of depth in the tree structure
                if needs_to_be_active {
                    find_all_windows(matches, needs_to_be_active, tree, wrkspc.get_active_child_id());
                }
                else {
                    for candidate_id in wrkspc.get_all_children().iter() {
                        find_all_windows(matches, needs_to_be_active, tree, *candidate_id);
                    }
                }

            },
            LayoutElementProfile::Window(_) => {
                matches.push(outer_element_id);
            },
            _ => {}
        }
    }
}

pub fn move_element(wm_state: &mut WMState, carry: LayoutElemID, destination: LayoutElemID) -> Result<String, String>{
    if let Some(mut destination) = wm_state.tree.lookup_element(destination){
        match destination.profile {
            LayoutElementProfile::Bisect(ref mut bisect) => {
                Err(String::from("Unimplemented!"))         
            },
            LayoutElementProfile::Grid(ref mut wrkspc) => {
                Err(String::from(""))
            },
            _ => Err(String::from("The destination needs to be either a segmentation or a workspace."))
        }
    }
    else{
        Err(String::from("Destination element missing in layout."))
    }
}

