use std::ops::DerefMut;

use common::definitions::LayoutElemID;
use layout::{LayoutTree, PARENT_ELEMENT};
use layout::arrangement:: {find_first_unoccupied, find_all_windows};
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::bisect::{Orientation, Bisect, Side};
use layout::policy::*;
use wmstate::WMState;

use gl::types::GLuint;

/*
    Generates a clockwise rotating layout 
*/

#[derive(Clone)]
pub struct AutoCirculation{
    last_orientations: Vec<Orientation>
}

impl AutoCirculation{
    pub fn init(num_workspaces: usize) -> AutoCirculation{
        let mut init_values = Vec::new();
        for i in 0..num_workspaces {
            init_values.push(Orientation::Horizontal);
        }


        AutoCirculation{
            last_orientations: init_values
        }
    }
}

impl LayoutPolicy for AutoCirculation{
    fn seat_window(&mut self, mut tree: &mut LayoutTree) -> LayoutElemID {
        if let Some(unoccupied_id) = find_first_unoccupied(&tree, PARENT_ELEMENT){
            // Unoccupied spots preexisting in the layout makes this easy: 
            let parent_ident = tree.parent_of(unoccupied_id);
            tree.animate_property(parent_ident, "gap_size", 7f32, false, 125);    
            
            
            unoccupied_id
        }
        else{
            // Otherwise, we have to extend the layout with new spots:
            println!("NOTICE: Extending the layout structure!");

            let mut active_windows: Vec<LayoutElemID> = Vec::new();
            find_all_windows(&mut active_windows, true, tree, PARENT_ELEMENT);

            if let Some(&last_id) = active_windows.last() {
                let current_workspace = {
                    let mut element = tree.lookup_element(2).unwrap();
                    
                    match element.profile{
                        LayoutElementProfile::Grid(ref mut grid) => grid.active_subspace(),
                        _ => { panic!("Expected element to be a workspace.") }        
                    }
                };

                let (_, extension) = Bisect::init(last_id, tree, self.last_orientations[current_workspace], 0.001f32);
                self.last_orientations[current_workspace] = self.last_orientations[current_workspace].opposite();

                let (new_preoccupied_id, new_unoccupied_id) = {
                    let mut iter = extension.children_iter().cloned();

                    (iter.next().unwrap(), iter.next().unwrap())
                };

                // update tags according to element swap
                tree.tags.handle_element_swap(last_id, new_preoccupied_id);

                if let Some(thrown_out_profile) = tree.swap_element_profile(last_id, LayoutElementProfile::Bisect(extension)){
                    tree.reserve_element_identity(new_preoccupied_id, thrown_out_profile);
                    tree.animate_property(last_id, "ratio", 0.5f32, false, 125);
                    new_unoccupied_id
                }
                else {
                    panic!("ERROR: No space in layout found!");
                }
            }
            else{
                panic!("Last index did not exist!");
            }
        }
    }

    fn decorate_window(&mut self, wm_state: &mut RwLockWriteGuard<WMState>, element_ident: LayoutElemID) {
        let &mut WMState{ref tree, ref mut graphics_program, ..} = wm_state.deref_mut();
       
        if let Some(mut element) = tree.lookup_element(element_ident){
            match element.profile{
                LayoutElementProfile::Window(ref mut window) => {
                    if let &mut Some(ref mut program) = graphics_program{
                        window.apply_frame(element_ident, program, 0f32);
                        tree.animate_property_after_delay(element_ident, "frame_opacity", 0.7f32, false, 500, 200);
                    }
                },
                _ => {}
            }
        }
    }

    fn detach_window(&mut self, tree: &mut LayoutTree, element_ident: LayoutElemID) -> Option<WeakView>{
        tree.reserve_element_identity(element_ident, LayoutElementProfile::None);
        
        let (bisect_parent_ident, bisect_removal, child_side) = {
            let parent_ident = tree.parent_of(element_ident);
            
            if let LayoutElementProfile::Bisect(ref bisect) = tree.lookup_element(parent_ident).unwrap().profile {
                let side = bisect.child_side(element_ident);
                let missing_adjacent_element = bisect.count_active_children(tree) == 0;
                
                (Some(parent_ident), missing_adjacent_element, side)      
            } 
            else { 
                (None, false, Side::Neither)
            }
        };

        if let Some(parent_ident) = bisect_parent_ident{
            // Parent is a bisect which have to either disappear or readjust for element detachment 

            let time_frame = 125;   

            if bisect_removal {
                // No elements left in bisect - it can go away:
                let grand_parent_ident = tree.parent_of(parent_ident);
                if let Some(mut grand_parent) = tree.lookup_element(grand_parent_ident){
                    
                    let (property, new_value) = match grand_parent.profile {
                        LayoutElementProfile::Bisect(ref bisect) => ("ratio", 1.0f32),
                        _ => ("", 0f32)
                    };

                    use std::process::Command;
                    Command::new("sh")
                        .arg("-c")
                        .arg("notify-send Bisect removal")
                        .output()
                        .expect("failed to execute process");

                    if property != "" { tree.animate_element_property(&mut grand_parent, property, new_value, false, time_frame); }
                }
                
                tree.reserve_element_identity(parent_ident, LayoutElementProfile::None);
            }
            else{
                // One element left in bisect - give it full space:
                tree.animate_property(parent_ident, "ratio", 
                    match child_side{
                        Side::Left => 0.0,
                        Side::Right => 1.0,
                        Side::Neither => panic!()
                    }
                , false, time_frame);            
            }
        }

        if let Some(mut element) = tree.lookup_element(element_ident){
            match element.profile{
                LayoutElementProfile::Window(ref mut window) => window.detach_view(),
                _ => None
            }
        }
        else {
            None
        }
    }

    fn box_clone(&self) -> Box<LayoutPolicy> {
        Box::new((*self).clone())
    }
}
