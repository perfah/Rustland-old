use std::ops::DerefMut;

use common::definitions::LayoutElemID;
use layout::{LayoutTree, PARENT_ELEMENT};
use layout::arrangement:: {find_first_unoccupied, find_all_windows};
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::bisect::{Orientation, Bisect};
use layout::policy::*;
use wmstate::WMState;

use gl::types::GLuint;

/*
    Generates a clockwise rotating layout 
*/

#[derive(Clone)]
pub struct Circulation{
    last_orientation: Orientation
}

impl Circulation{
    pub fn init() -> Circulation{
        Circulation{
            last_orientation: Orientation::Horizontal
        }
    }
}

impl LayoutPolicy for Circulation{
    fn seat_window(&mut self, mut tree: &mut LayoutTree) -> LayoutElemID {
        // Next orientation:
        self.last_orientation = self.last_orientation.opposite();
        
        if let Some(unoccupied_id) = find_first_unoccupied(&tree, PARENT_ELEMENT){
            // Unoccupied spots preexisting in the layout makes this easy: 
            unoccupied_id
        }
        else{
            // Otherwise, we have to extend the layout with new spots:
            println!("NOTICE: Extending the layout structure!");

            let mut active_windows: Vec<LayoutElemID> = Vec::new();
            find_all_windows(&mut active_windows, true, tree, PARENT_ELEMENT);

            if let Some(&last_id) = active_windows.last() {
                let (_, extension) = Bisect::init(last_id, tree, self.last_orientation.clone(), 0.001f32);
                let new_preoccupied_id = extension.get_children()[0];
                let new_unoccupied_id = extension.get_children()[1];

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
       
        if let Some(element) = tree.lookup_element(element_ident){
            match *element.get_profile_mut(){
                LayoutElementProfile::Window(ref mut window) => {
                    if let &mut Some(ref mut program) = graphics_program{
                        window.apply_frame(element_ident, program, 0f32);
                        tree.animate_property_after_delay(element_ident, "frame_opacity", 0f32, 0.7f32, false, 500, 200);
                    }
                },
                _ => {}
            }
        }
    }

    fn box_clone(&self) -> Box<LayoutPolicy> {
        Box::new((*self).clone())
    }
}
